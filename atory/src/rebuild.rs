use glsl::syntax::{Declaration, ExternalDeclaration, NonEmpty, ShaderStage, TranslationUnit};
use glsl::parser::Parse;

pub fn get_shadertoy_uniform_ast() -> NonEmpty<ExternalDeclaration> {
    let glsl = "
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

    let stage = ShaderStage::parse(glsl).unwrap();
    let TranslationUnit(decs) = stage;
    decs
}

pub fn parse_declarations(declarations : NonEmpty<ExternalDeclaration>) -> NonEmpty<ExternalDeclaration>{
    // let NonEmpty(decs) = declarations;
    // decs.push();
    declarations
}

pub fn parse(root : TranslationUnit) -> TranslationUnit{
    match root {
        TranslationUnit(declarations) => {
            TranslationUnit(parse_declarations(declarations))
        },
        _ => {panic!("wot???")}
    }
}
