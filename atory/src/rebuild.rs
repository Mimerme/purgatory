use glsl::syntax::{Declaration, ExternalDeclaration, NonEmpty, ShaderStage, TranslationUnit};

pub fn get_shadertoy_uniform_ast() -> Vec<Declaration> {
    
}

pub fn parse_declarations(declarations : NonEmpty) -> NonEpty{
    let NonEmpty(decs) = declarations;
    decs.push();
    

    declarations
}

pub fn parse(root : TranslationUnit) -> TranslationUnit{
    match root {
        TranslationUnit(NonEmpty(declarations)) => {

        },
        _ => {panic!("wot???")}
    }
}
