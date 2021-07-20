use glsl::parser::Parse as _;
use glsl::syntax::{Declaration, ExternalDeclaration, NonEmpty, ShaderStage, TranslationUnit};
use crate::download::download;
use crate::rebuild;
use core::fmt::Write;

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
    let stage = ShaderStage::parse(shadertoy_glsl).unwrap();
    let mut out_write = String::new();
    // println!("{:?}", stage);
    glsl::transpiler::glsl::show_translation_unit(&mut out_write, &stage);
    println!("{:?}", out_write);
}

/// Creates an AST of uniforms and adds it to the final transpiled program
#[test]
pub fn test_glsl_uniform_insert(){
    // Stage is the input ast
    let stage = ShaderStage::parse(shadertoy_glsl).unwrap();

    let res = "
    #version 450
    in vec2 fragCoord;
    out vec4 fragColor;

    uniform vec4 iMouse;
    uniform float iTime;
    uniform float iTimeDelta;
    uniform int iFrame;
    uniform vec4 iDate;
    uniform vec2 iResolution;

    void main()
    {
        // Normalized pixel coordinates (from 0 to 1)
        vec2 uv = fragCoord/iResolution.xy;
    
        // Time varying pixel color
        vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));
    
        // Output to screen
        fragColor = vec4(col,1.0);
    }
    ";

    // Ans is the correct AST
    let ans = ShaderStage::parse(res).unwrap();

    // Rebuilt is the newly parsed AST
    let rebuilt = rebuild::parse(stage.clone());

    assert_eq!(rebuilt, ans);
    let mut out_write = String::new();
    glsl::transpiler::glsl::show_translation_unit(&mut out_write, &rebuilt);
    //println!("{:?}", out_write);
//    assert_eq!(ans, rebuilt);
}

#[test]
pub fn test_download_shadertoy(){
    let (name, code) = download("tsXBzS", false).unwrap();
    let out = port_shadertoy(&code);

    std::fs::write("bbbbb.frag", out).expect("Unable to write file");
}

#[test]
pub fn test_download_shadertoy2(){
    let (name, code) = download("stXXzS", false).unwrap();
    let out = port_shadertoy(&code);

    std::fs::write("bbbbb2.frag", out).expect("Unable to write file");
}

#[test]
pub fn test_download_shadertoy3(){
    let (name, code) = download("slsXDX", false).unwrap();
    let out = port_shadertoy(&code);

    std::fs::write("bbbbb3.frag", out).expect("Unable to write file");
}




pub fn port_shadertoy(in_frag : &String) -> String {
    let mut out_str = String::new();
    let stage = ShaderStage::parse(in_frag).unwrap();
    let rebuilt = rebuild::parse(stage.clone());

    glsl::transpiler::glsl::show_translation_unit(&mut out_str, &rebuilt);
    out_str
}