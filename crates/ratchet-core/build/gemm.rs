#![allow(non_snake_case)]
use crate::{Generate, KernelRenderer};
use std::{fs::File, io::Write};
use tera::Context;

pub struct Gemm;

//TODO
//1. Add different kernel elements
//2. Add different tile sizes
//3. Add different row per thread
//4. Add transposing
impl Generate for Gemm {
    fn generate(renderer: &mut KernelRenderer) -> anyhow::Result<()> {
        let A_FIT = [false, true];
        let B_FIT = [false, true];
        let OUT_FIT = [false, true];
        let QUANTIZED_B = [false, true];

        let path = renderer.templates_path.join("gemm.wgsl");
        renderer.tera.add_template_file(path, Some("gemm"))?;
        for a_fit in A_FIT.iter() {
            for b_fit in B_FIT.iter() {
                for out_fit in OUT_FIT.iter() {
                    for quantize_b in QUANTIZED_B.iter() {
                        let mut context = Context::new();
                        context.insert("A_FIT", a_fit);
                        context.insert("B_FIT", b_fit);
                        context.insert("OUT_FIT", out_fit);
                        context.insert("QUANTIZED_B", quantize_b);
                        context.insert("TILE_DIM", &32);
                        context.insert("ROW_PER_THREAD", &4);

                        let rendered = renderer.tera.render("gemm", &context)?;

                        let op_name = if *quantize_b { "qgemm" } else { "sgemm" };

                        let kernel_fname = format!(
                            "{}_A_FIT{}_B_FIT{}_OUT_FIT{}_vec4.wgsl",
                            op_name, a_fit, b_fit, out_fit
                        );
                        let mut file = File::create(renderer.dest_path.join(kernel_fname))?;
                        file.write_all(rendered.as_bytes())?;
                    }
                }
            }
        }
        Ok(())
    }
}
