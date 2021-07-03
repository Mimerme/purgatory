use glsl::parser::Parse as _;
use glsl::syntax::{Declaration, ExternalDeclaration, ShaderStage};
use crate::download::download;
use core::fmt::Write;



#[test]
pub fn test_glsl_transpile() {
    bevy::log::debug!("Transpiling demo glsl");
    let glsl = "
         layout (location = 0) in vec3 pos;
         layout (location = 1) in vec4 col;
       
         out vec4 v_col;
       
         uniform mat4 projview;
       
         void main() {
           v_col = col; // pass color to the next stage
           gl_Position = projview * vec4(pos, 1.);
         }
       ";
    
    let stage = ShaderStage::parse(glsl);
    for s in stage.unwrap().into_iter() {
        match s {
            ExternalDeclaration::Preprocessor(_) => todo!(),
            ExternalDeclaration::FunctionDefinition(_) => {println!("Function def")}, 
            ExternalDeclaration::Declaration(dec) => {
                match dec {
                    Declaration::InitDeclaratorList(lst) => {
                        println!("{:?}", lst);
                    },
                    Declaration::FunctionPrototype(_) => {},
                    _ => {}
                }
            },
        }
    }
}

struct Transpiler(String);
impl Write for Transpiler {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
#[test]
pub fn test_glsl_write(){
    bevy::log::debug!("Writing glsl");
    let glsl = "
         layout (location = 0) in vec3 pos;
         layout (location = 1) in vec4 col;
         out vec4 v_col;
         uniform mat4 projview;
       
         void main() {
           v_col = col; // pass color to the next stage
           gl_Position = projview * vec4(pos, 1.);
         }
       ";
    let stage = ShaderStage::parse(glsl).unwrap();
    let mut out_write = Transpiler(String::new());
    // println!("{:?}", stage);
    glsl::transpiler::glsl::show_translation_unit(&mut out_write, &stage);
    println!("{:?}", out_write.0);
}


#[test]
pub fn test_download_shadertoy(){
    download("tsXBzS").unwrap();
}
