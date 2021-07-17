use glsl::syntax::NonEmpty;
use glsl::syntax::ExternalDeclaration;
use glsl::syntax::ShaderStage;
use glsl::parser::Parse;
use std::fmt::Write;

pub fn load_glsl_shader(file_path : &str) -> NonEmpty<ExternalDeclaration> {
	let content = std::fs::read_to_string(file_path).unwrap();
	let stage = ShaderStage::parse(content).unwrap().0;
	stage
}


pub struct AtoryTranspiler(String);
impl Write for AtoryTranspiler {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}