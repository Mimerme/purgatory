use glsl::parser::Parse;
use glsl::syntax::{Declaration, ExternalDeclaration, NonEmpty, ShaderStage, TranslationUnit};

pub fn get_main() -> NonEmpty<ExternalDeclaration> {
    let main_def = "void main(){

    }";

    let stage = ShaderStage::parse(main_def).unwrap();
    let TranslationUnit(decs) = stage;
    decs
}

pub fn get_shadertoy_defs() -> NonEmpty<ExternalDeclaration> {
    let uniforms = "
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
";

    let stage = ShaderStage::parse(uniforms).unwrap();
    let TranslationUnit(decs) = stage;
    decs
}

pub fn parse_declarations(
    mut declarations: NonEmpty<ExternalDeclaration>,
    mut shadertoy_uniforms: NonEmpty<ExternalDeclaration>,
    mut function_defs : NonEmpty<ExternalDeclaration>,
) -> NonEmpty<ExternalDeclaration> {
    // let NonEmpty(decs) = declarations;
    // decs.push();

    // shadertoy_declarations.into_iter().for_each(|d| {
    //     declarations.push(d);
    // });

    shadertoy_uniforms.extend(declarations);
    shadertoy_uniforms.extend(function_defs);

    shadertoy_uniforms
}

pub fn parse(root: TranslationUnit) -> TranslationUnit {
    match root {
        TranslationUnit(declarations) => {
            TranslationUnit(parse_declarations(declarations, get_shadertoy_defs(), get_main()))
        }
        _ => {
            panic!("wot???")
        }
    }
}
