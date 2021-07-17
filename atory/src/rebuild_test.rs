use glsl::parser::Parse as _;
use glsl::syntax::{Declaration, ExternalDeclaration, NonEmpty, ShaderStage, TranslationUnit};
use crate::download::download;
use crate::rebuild;
use core::fmt::Write;

// static glsl : &str = "
//          layout (location = 0) in vec3 pos;
//          layout (location = 1) in vec4 col;
//          out vec4 v_col;
//          uniform mat4 projview;
       
//          void main() {
           
//            v_col = col; // pass color to the next stage
//            gl_Position = projview * vec4(pos, 1.);
//          }
//        ";
static shadertoy_glsl : &str = "
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

struct Transpiler(String);
impl Write for Transpiler {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.push_str(s);
        Ok(())
    }
}
#[test]
pub fn test_glsl_parse(){
    let stage = ShaderStage::parse(shadertoy_glsl).unwrap();
    let mut out_write = Transpiler(String::new());
    println!("{:#?}", stage);
//     glsl::transpiler::glsl::show_translation_unit(&mut out_write, &stage);
//     println!("{:?}", out_write.0);
}

#[test]
pub fn test_glsl_uniform_insert_defs(){
    // Stage is the input ast
    let
 stage = ShaderStage::parse(shadertoy_glsl).unwrap();

    let res = "
     layout(set = 2, binding = 0) uniform ShaderToyUniform_time {
         float iTime;
     };
     
     layout(set = 2, binding = 1) uniform ShaderToyUniform_mouse {
         vec4 iMouse;
     };
     
     layout(set = 2, binding = 2) uniform ShaderToyUniform_time_delta {
         float iTimeDelta;
     };
     
     layout(set = 2, binding = 3) uniform ShaderToyUniform_frame {
         int iFrame;
     };
     
     layout(set = 2, binding = 4) uniform ShaderToyUniform_date {
         vec4 iDate;
     };
     
     layout(set = 2, binding = 5) uniform ShaderToyUniform_resolution {
         vec2 iResolution;
     };


    void mainImage( out vec4 fragColor, in vec2 fragCoord )
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
    let rebuilt = rebuild::parse(stage.clone(), false);
    // println!("{:#?}", ans);

    // let mut out_write = Transpiler(String::new());
    // glsl::transpiler::glsl::show_translation_unit(&mut out_write, &rebuilt);

    // let mut ans_write = Transpiler(String::new());
    // // glsl::transpiler::glsl::show_translation_unit(&mut ans_write, &ans);
    // println!("Rebuilt src: {:?}", out_write.0);
    // println!("Correct: {:?}", ans_write.0);
    // // assert_eq!(rebuilt, ans);
    assert_eq!(rebuilt, ans);
}


#[test]
pub fn test_glsl_uniform_insert_defs_and_main(){
    // Stage is the input ast
    let
 stage = ShaderStage::parse(shadertoy_glsl).unwrap();

    let res = "
     layout(set = 2, binding = 0) uniform ShaderToyUniform_time {
         float iTime;
     };
     
     layout(set = 2, binding = 1) uniform ShaderToyUniform_mouse {
         vec4 iMouse;
     };
     
     layout(set = 2, binding = 2) uniform ShaderToyUniform_time_delta {
         float iTimeDelta;
     };
     
     layout(set = 2, binding = 3) uniform ShaderToyUniform_frame {
         int iFrame;
     };
     
     layout(set = 2, binding = 4) uniform ShaderToyUniform_date {
         vec4 iDate;
     };
     
     layout(set = 2, binding = 5) uniform ShaderToyUniform_resolution {
         vec2 iResolution;
     };


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
    let rebuilt = rebuild::parse(stage.clone(), true);
    // println!("{:#?}", ans);

    // let mut out_write = Transpiler(String::new());
    // glsl::transpiler::glsl::show_translation_unit(&mut out_write, &rebuilt);

    // let mut ans_write = Transpiler(String::new());
    // // glsl::transpiler::glsl::show_translation_unit(&mut ans_write, &ans);
    // println!("Rebuilt src: {:?}", out_write.0);
    // println!("Correct: {:?}", ans_write.0);
    // // assert_eq!(rebuilt, ans);
    assert_eq!(rebuilt, ans);
}

pub fn write_ast_to_src(root : TranslationUnit, dest : &str){
    let mut write = Transpiler(String::new());
    glsl::transpiler::glsl::show_translation_unit(&mut write, &root);
    let mut f = std::fs::File::create(dest).expect("Unable to create file");
    std::io::Write::write_all(&mut f, write.0.as_bytes()).expect("Unable to write data");
}

/// Do the whole download shader and transpiling shindig
/// Depends ont he purg crate to run the shader in the Bevy engine
#[test]
pub fn test_shadertoy_shader(){
    let (name, shadertoy_raw_glsl) = crate::download::download("wlVGWd", false).unwrap();
    let trans_glsl = ShaderStage::parse(shadertoy_raw_glsl).unwrap();
    let rebuilt = rebuild::parse(trans_glsl, true);
    write_ast_to_src(rebuilt, "test.frag");

    // println!("{:#?}", ans);

    // let mut out_write = Transpiler(String::new());
    // glsl::transpiler::glsl::show_translation_unit(&mut out_write, &rebuilt);

    // let mut ans_write = Transpiler(String::new());
    // // glsl::transpiler::glsl::show_translation_unit(&mut ans_write, &ans);
    // println!("Rebuilt src: {:?}", out_write.0);
    // println!("Correct: {:?}", ans_write.0);
    // // assert_eq!(rebuilt, ans);
    // assert_eq!(rebuilt, ans);
}
// #[test]
// pub fn test_download_shadertoy(){
//     download("tsXBzS").unwrap();
// }
//

static ref_shader : &str = "#version 450
precision lowp float;

in vec2 fragCoord;

uniform sampler2D Texture;
uniform float iTime;
uniform vec2 iResolution;
out vec4 fragColor;

void main() {
    //gl_FragColor = texture2D(Texture, uv);
    //fragColor = vec4(iTime / 255.0, 0.0, 0.0, 1.0);

    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;


    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Output to screen
    fragColor = vec4(uv,0.0, 1.0);
}";



