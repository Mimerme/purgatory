use glsl::parser::Parse as _;
use glsl::syntax::{Declaration, ExternalDeclaration, NonEmpty, ShaderStage, TranslationUnit};
use crate::download::download;
use crate::rebuild;
use core::fmt::Write;

static glsl : &str = "
         layout (location = 0) in vec3 pos;
         layout (location = 1) in vec4 col;
         out vec4 v_col;
         uniform mat4 projview;
       
         void main() {
           
           v_col = col; // pass color to the next stage
           gl_Position = projview * vec4(pos, 1.);
         }
       ";
static shadertoy_glsl : &str = "
void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(col,1.0);
}";

// #[test]
// pub fn test_glsl_transpile() {
//     let glsl = "
//          layout (location = 0) in vec3 pos;
//          layout (location = 1) in vec4 col;
       
//          out vec4 v_col;
       
//          uniform mat4 projview;
       
//          void main() {
//            v_col = col; // pass color to the next stage
//            gl_Position = projview * vec4(pos, 1.);
//          }
//        ";

/*
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
*/


#[test]
pub fn test_glsl_write(){
    let stage = ShaderStage::parse(glsl).unwrap();
    let mut out_write = Transpiler(String::new());
    // println!("{:?}", stage);
    glsl::transpiler::glsl::show_translation_unit(&mut out_write, &stage);
    println!("{:?}", out_write.0);
}

/// Creates an AST of uniforms and adds it to the final transpiled program
#[test]
pub fn test_glsl_uniform_insert(){
    // Stage is the input ast
    let stage = ShaderStage::parse(glsl).unwrap();

    let res = "
    uniform float iTime;
    uniform vec4 iMouse;
    uniform float iTimeDelta;
    uniform int iFrame;
    uniform vec4 iDate;
    uniform vec2 iResolution;


    void mainImage( out vec4 fragColor, in vec2 fragCoord )
    {
        // Normalized pixel coordinates (from 0 to 1)
        vec2 uv = fragCoord/iResolution.xy;
    
        // Time varying pixel color
        vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));
    
        // Output to screen
        fragColor = vec4(col,1.0);
    }

    void main(){

    }
    ";

    // Ans is the correct AST
    let ans = ShaderStage::parse(res).unwrap();

    // Rebuilt is the newly parsed AST
    let rebuilt = rebuild::parse(stage.clone());
    assert_eq!(rebuilt, stage);

//    assert_eq!(ans, rebuilt);
}

// #[test]
// pub fn test_download_shadertoy(){
//     download("tsXBzS").unwrap();
// }
