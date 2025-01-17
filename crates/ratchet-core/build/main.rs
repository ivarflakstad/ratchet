mod binary;
mod concat;
mod gemm;
mod gemv;
mod norm;
mod reindex;
mod unary;

use anyhow::Context as anyhowCtx;
use binary::BinaryOp;
use concat::ConcatOp;
use gemm::Gemm;
use gemv::Gemv;
use norm::NormOp;
use reindex::ReindexOp;
use unary::UnaryOp;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tera::Tera;

const KERNEL_TEMPLATES_DIR: &str = "kernel-templates";
const KERNEL_HANDWRITTEN_DIR: &str = "kernel-handwritten";
const KERNEL_GENERATED_DIR: &str = "kernel-generated";
const KERNELS_RS: &str = "kernels.rs";

/// # Generate
///
/// This trait is used to generate the kernels for the different operations.
pub trait Generate {
    fn generate(renderer: &mut KernelRenderer) -> anyhow::Result<()>;
}

#[derive(strum_macros::EnumIter, Debug)]
pub enum KernelElement {
    Scalar,
    Vec2,
    Vec4,
}

impl std::fmt::Display for KernelElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KernelElement::Scalar => "scalar",
            KernelElement::Vec2 => "vec2",
            KernelElement::Vec4 => "vec4",
        };
        write!(f, "{}", s)
    }
}

pub enum WgslDType {
    F32,
}

impl std::fmt::Display for WgslDType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WgslDType::F32 => write!(f, "f32"),
        }
    }
}

impl KernelElement {
    pub fn as_wgsl(&self, dtype: WgslDType) -> String {
        match self {
            KernelElement::Scalar => dtype.to_string(),
            KernelElement::Vec2 => format!("vec2<{}>", dtype),
            KernelElement::Vec4 => format!("vec4<{}>", dtype),
        }
    }

    pub fn as_size(&self) -> usize {
        match self {
            KernelElement::Scalar => 1,
            KernelElement::Vec2 => 2,
            KernelElement::Vec4 => 4,
        }
    }
}

#[derive(Debug)]
pub struct KernelRenderer {
    tera: Tera,
    dest_path: PathBuf,
    templates_path: PathBuf,
}

impl Default for KernelRenderer {
    fn default() -> Self {
        let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));
        KernelRenderer {
            tera: Tera::default(),
            dest_path: base_path.join(KERNEL_GENERATED_DIR),
            templates_path: base_path.join(KERNEL_TEMPLATES_DIR),
        }
    }
}

impl KernelRenderer {
    fn generate(&mut self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.dest_path)?;
        UnaryOp::generate(self)?;
        BinaryOp::generate(self)?;
        ReindexOp::generate(self)?;
        NormOp::generate(self)?;
        Gemm::generate(self)?;
        Gemv::generate(self)?;
        ConcatOp::generate(self)?;
        Ok(())
    }
}

fn embed_kernels() -> anyhow::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = manifest_dir.join("src");
    let mut file = std::fs::File::create(out_dir.join(KERNELS_RS)).context(
        "Failed to create `src/kernels.rs`. Make sure you have `src` directory in your project.",
    )?;

    const HEADER: &str = r#"
// This file is generated by build.rs. Do not edit it manually.
use std::collections::HashMap;
pub fn kernels() -> &'static HashMap<&'static str, &'static str> {
    static KERNELS: std::sync::OnceLock<HashMap<&'static str, &'static str>> = std::sync::OnceLock::new();
    KERNELS.get_or_init(|| HashMap::from_iter([
"#;

    const FOOTER: &str = r#"
    ]))
}
    "#;

    writeln!(file, "{}", HEADER.trim_matches('\n'))?;

    let paths = {
        let mut paths: Vec<PathBuf> = Iterator::chain(
            globwalk::glob(
                manifest_dir
                    .join(KERNEL_GENERATED_DIR)
                    .join("**.wgsl")
                    .to_string_lossy(),
            )?,
            globwalk::glob(
                manifest_dir
                    .join(KERNEL_HANDWRITTEN_DIR)
                    .join("**.wgsl")
                    .to_string_lossy(),
            )?,
        )
        .flatten()
        .map(|entry| entry.path().to_owned())
        .collect();
        paths.sort();
        paths
    };

    for path in paths {
        let name = path.file_stem().unwrap().to_str().unwrap();

        // Account for Windows-isms
        let diff = pathdiff::diff_paths(&path, &out_dir)
            .ok_or(anyhow::format_err!("Failed to get path diff"))?;
        let normalized_path = diff.to_string_lossy().replace('\\', "/");

        writeln!(
            &mut file,
            "        (\"{name}\", include_str!(r\"{normalized_path}\")),"
        )?;
    }

    writeln!(file, "{}", FOOTER.trim_matches('\n'))?;

    Ok(())
}

fn main() {
    // Only run the build script when the templates change to prevent unnecessary rebuilds
    println!("cargo:rerun-if-changed={KERNEL_HANDWRITTEN_DIR}");
    println!("cargo:rerun-if-changed={KERNEL_TEMPLATES_DIR}");

    let mut generator = KernelRenderer::default();
    generator.generate().unwrap();
    embed_kernels().unwrap();
    if let Err(e) = Command::new("cargo").args(["fmt"]).status() {
        eprintln!("Failed to execute `cargo fmt`: {}", e);
    }
}
