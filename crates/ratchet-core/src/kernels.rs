// This file is generated by build.rs. Do not edit it manually.
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    pub static ref KERNELS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert(
            "abs_scalar",
            include_str!(r"../kernels/generated/abs_scalar.wgsl"),
        );
        m.insert(
            "relu_vec2",
            include_str!(r"../kernels/generated/relu_vec2.wgsl"),
        );
        m.insert(
            "sub_vec4",
            include_str!(r"../kernels/generated/sub_vec4.wgsl"),
        );
        m.insert(
            "tanh_vec2",
            include_str!(r"../kernels/generated/tanh_vec2.wgsl"),
        );
        m.insert(
            "sin_vec4",
            include_str!(r"../kernels/generated/sin_vec4.wgsl"),
        );
        m.insert(
            "mul_scalar",
            include_str!(r"../kernels/generated/mul_scalar.wgsl"),
        );
        m.insert(
            "log_vec2",
            include_str!(r"../kernels/generated/log_vec2.wgsl"),
        );
        m.insert(
            "mul_vec2",
            include_str!(r"../kernels/generated/mul_vec2.wgsl"),
        );
        m.insert(
            "gelu_vec4",
            include_str!(r"../kernels/generated/gelu_vec4.wgsl"),
        );
        m.insert(
            "sqrt_scalar",
            include_str!(r"../kernels/generated/sqrt_scalar.wgsl"),
        );
        m.insert(
            "exp_vec2",
            include_str!(r"../kernels/generated/exp_vec2.wgsl"),
        );
        m.insert(
            "cos_vec2",
            include_str!(r"../kernels/generated/cos_vec2.wgsl"),
        );
        m.insert(
            "sub_scalar",
            include_str!(r"../kernels/generated/sub_scalar.wgsl"),
        );
        m.insert(
            "tanh_vec4",
            include_str!(r"../kernels/generated/tanh_vec4.wgsl"),
        );
        m.insert(
            "sin_vec2",
            include_str!(r"../kernels/generated/sin_vec2.wgsl"),
        );
        m.insert(
            "ceil_scalar",
            include_str!(r"../kernels/generated/ceil_scalar.wgsl"),
        );
        m.insert(
            "relu_vec4",
            include_str!(r"../kernels/generated/relu_vec4.wgsl"),
        );
        m.insert(
            "sub_vec2",
            include_str!(r"../kernels/generated/sub_vec2.wgsl"),
        );
        m.insert(
            "mul_vec4",
            include_str!(r"../kernels/generated/mul_vec4.wgsl"),
        );
        m.insert(
            "gelu_vec2",
            include_str!(r"../kernels/generated/gelu_vec2.wgsl"),
        );
        m.insert(
            "exp_vec4",
            include_str!(r"../kernels/generated/exp_vec4.wgsl"),
        );
        m.insert(
            "cos_vec4",
            include_str!(r"../kernels/generated/cos_vec4.wgsl"),
        );
        m.insert(
            "tanh_scalar",
            include_str!(r"../kernels/generated/tanh_scalar.wgsl"),
        );
        m.insert(
            "log_vec4",
            include_str!(r"../kernels/generated/log_vec4.wgsl"),
        );
        m.insert(
            "relu_scalar",
            include_str!(r"../kernels/generated/relu_scalar.wgsl"),
        );
        m.insert(
            "add_scalar",
            include_str!(r"../kernels/generated/add_scalar.wgsl"),
        );
        m.insert(
            "floor_vec2",
            include_str!(r"../kernels/generated/floor_vec2.wgsl"),
        );
        m.insert(
            "div_scalar",
            include_str!(r"../kernels/generated/div_scalar.wgsl"),
        );
        m.insert(
            "exp_scalar",
            include_str!(r"../kernels/generated/exp_scalar.wgsl"),
        );
        m.insert(
            "floor_scalar",
            include_str!(r"../kernels/generated/floor_scalar.wgsl"),
        );
        m.insert(
            "ceil_vec4",
            include_str!(r"../kernels/generated/ceil_vec4.wgsl"),
        );
        m.insert(
            "div_vec4",
            include_str!(r"../kernels/generated/div_vec4.wgsl"),
        );
        m.insert(
            "cos_scalar",
            include_str!(r"../kernels/generated/cos_scalar.wgsl"),
        );
        m.insert(
            "sqrt_vec2",
            include_str!(r"../kernels/generated/sqrt_vec2.wgsl"),
        );
        m.insert(
            "add_vec2",
            include_str!(r"../kernels/generated/add_vec2.wgsl"),
        );
        m.insert(
            "sin_scalar",
            include_str!(r"../kernels/generated/sin_scalar.wgsl"),
        );
        m.insert(
            "gelu_scalar",
            include_str!(r"../kernels/generated/gelu_scalar.wgsl"),
        );
        m.insert(
            "abs_vec4",
            include_str!(r"../kernels/generated/abs_vec4.wgsl"),
        );
        m.insert(
            "ceil_vec2",
            include_str!(r"../kernels/generated/ceil_vec2.wgsl"),
        );
        m.insert(
            "floor_vec4",
            include_str!(r"../kernels/generated/floor_vec4.wgsl"),
        );
        m.insert(
            "sqrt_vec4",
            include_str!(r"../kernels/generated/sqrt_vec4.wgsl"),
        );
        m.insert(
            "add_vec4",
            include_str!(r"../kernels/generated/add_vec4.wgsl"),
        );
        m.insert(
            "abs_vec2",
            include_str!(r"../kernels/generated/abs_vec2.wgsl"),
        );
        m.insert(
            "log_scalar",
            include_str!(r"../kernels/generated/log_scalar.wgsl"),
        );
        m.insert(
            "div_vec2",
            include_str!(r"../kernels/generated/div_vec2.wgsl"),
        );
        m.insert("qgemm_vec4", include_str!(r"../kernels/qgemm_vec4.wgsl"));
        m.insert(
            "sgemm_scalar",
            include_str!(r"../kernels/sgemm_scalar.wgsl"),
        );
        m.insert("add_scalar", include_str!(r"../kernels/add_scalar.wgsl"));
        m.insert("sgemm_vec2", include_str!(r"../kernels/sgemm_vec2.wgsl"));
        m.insert(
            "softmax_vec2",
            include_str!(r"../kernels/softmax_vec2.wgsl"),
        );
        m.insert("sgemm_vec4", include_str!(r"../kernels/sgemm_vec4.wgsl"));
        m.insert(
            "softmax_scalar",
            include_str!(r"../kernels/softmax_scalar.wgsl"),
        );
        m.insert(
            "softmax_vec4",
            include_str!(r"../kernels/softmax_vec4.wgsl"),
        );
        m
    };
}
