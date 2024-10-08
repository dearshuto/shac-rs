mod shader_compiler;
mod shader_converter;

pub use shader_compiler::ShaderCompiler;
pub use shader_converter::{convert_glsl_to_wgsl, ShaderConverter};

pub enum Stage {
    Vertex,
    Pixel,
    Compute,
}
