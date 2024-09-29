use std::io::Write;

use std::path::Path;

use crate::ShaderConverter;

pub enum ShaderStage {
    Vertex,
    Pixel,
    Compute,
}

#[derive(Debug, Default)]
pub struct ShaderCompiler {}

impl ShaderCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_binary(
        &mut self,
        #[allow(unused_variables)] source: &str,
        #[allow(unused_variables)] shader_stage: ShaderStage,
    ) -> Vec<u8> {
        match shader_stage {
            ShaderStage::Vertex => self.create_binary_impl(source, shader_stage),
            ShaderStage::Pixel => self.create_binary_impl(source, shader_stage),
            ShaderStage::Compute => {
                let wglsl = ShaderConverter::convert_glsl_to_wgsl(source);
                ShaderConverter::convert_wgsl_to_spirv(&wglsl)
            }
        }
    }

    pub fn build_graphics_shader<TPath: AsRef<Path>>(
        &mut self,
        #[allow(unused_variables)] vertex_shader_path: &TPath,
        #[allow(unused_variables)] pixel_shader_path: &TPath,
    ) {
        let vertex_shader_source = std::fs::read_to_string(vertex_shader_path).unwrap();
        let pixel_shader_source = std::fs::read_to_string(pixel_shader_path).unwrap();

        let vertex_shader_binary = self.create_binary(&vertex_shader_source, ShaderStage::Vertex);
        let pixel_shader_binary = self.create_binary(&pixel_shader_source, ShaderStage::Pixel);

        let output_directory_path = std::path::Path::new("outputs/resources/shaders");
        let _ = std::fs::create_dir_all(&output_directory_path).expect("");

        let vertex_shader_file_path = vertex_shader_path.as_ref().with_extension("vs.spv");
        let pixel_shader_file_path = vertex_shader_path.as_ref().with_extension("fs.spv");

        // 頂点シェーダ
        std::fs::File::create(
            output_directory_path.join(vertex_shader_file_path.file_name().unwrap()),
        )
        .unwrap()
        .write_all(&vertex_shader_binary)
        .unwrap();

        // ピクセルシェーダ
        std::fs::File::create(
            output_directory_path.join(pixel_shader_file_path.file_name().unwrap()),
        )
        .unwrap()
        .write_all(&pixel_shader_binary)
        .unwrap();
    }

    pub fn create_binary_impl(
        &mut self,
        #[allow(unused_variables)] source: &str,
        #[allow(unused_variables)] shader_stage: ShaderStage,
    ) -> Vec<u8> {
        let stage = match shader_stage {
            ShaderStage::Vertex => naga::ShaderStage::Vertex,
            ShaderStage::Pixel => naga::ShaderStage::Fragment,
            ShaderStage::Compute => panic!(),
        };
        let options = naga::front::glsl::Options::from(stage);
        let module = naga::front::glsl::Frontend::default()
            .parse(&options, source)
            .unwrap();
        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap();
        unsafe {
            let options = naga::back::spv::Options::default();
            let mut data = naga::back::spv::write_vec(&module, &info, &options, None).unwrap();

            let ratio = std::mem::size_of::<u32>() / std::mem::size_of::<u8>();
            let length = data.len() * ratio;
            let capacity = data.capacity() * ratio;
            let ptr = data.as_mut_ptr() as *mut u8;
            let u8_data: Vec<u8> = Vec::from_raw_parts(ptr, length, capacity).clone();

            // 元データが 2 重に破棄されないように、元データを破棄しないようにする
            std::mem::forget(data);

            u8_data
        }
    }
}
