use glsl::syntax::FunctionParameterDeclaration::Named;
use glsl::syntax::StorageQualifier::Out;
use glsl::syntax::TypeQualifierSpec::Storage;
use glsl::{
    parser::Parse,
    syntax::{
        Declaration, ExternalDeclaration, FullySpecifiedType, FunctionPrototype, Identifier,
        NonEmpty, ShaderStage, TranslationUnit, TypeQualifier, TypeSpecifier,
        TypeSpecifierNonArray,
    },
};
use ExternalDeclaration::FunctionDefinition;

/// Should rebuild a main defintion with its body as that in mainImage()
pub fn get_main(main_image: NonEmpty<ExternalDeclaration>) -> NonEmpty<ExternalDeclaration> {
    let mut definitions = vec![];
    for dec in main_image.into_iter() {
        match dec {
            FunctionDefinition(def) => {
                if def.prototype.name.0 != "mainImage" {
                    definitions.push(ExternalDeclaration::FunctionDefinition(def));
                    continue;
                }

                let proto = FunctionPrototype {
                    ty: FullySpecifiedType {
                        qualifier: None,
                        ty: TypeSpecifier {
                            ty: TypeSpecifierNonArray::Void,
                            array_specifier: None,
                        },
                    },
                    name: Identifier::new("main").unwrap(),
                    parameters: Vec::new(),
                };

                let mut def_clone = def.clone();

                def_clone.prototype = proto;
                definitions.push(ExternalDeclaration::FunctionDefinition(def_clone));
            }
            x => {
                definitions.push(x);
            },
        }
    }

    NonEmpty(definitions)
}

pub fn get_shadertoy_defs() -> NonEmpty<ExternalDeclaration> {
    let defs = "
    #version 450
    in vec2 fragCoord;
    out vec4 fragColor;

    uniform vec4 iMouse;
    uniform float iTime;
    uniform float iTimeDelta;
    uniform int iFrame;
    uniform vec4 iDate;
    uniform vec2 iResolution;
";

    let stage = ShaderStage::parse(defs).unwrap();
    let TranslationUnit(decs) = stage;
    decs
}

pub fn parse_declarations(
    mut begin: NonEmpty<ExternalDeclaration>,
    mut mid: Option<NonEmpty<ExternalDeclaration>>,
    mut end: Option<NonEmpty<ExternalDeclaration>>,
) -> NonEmpty<ExternalDeclaration> {
    // let NonEmpty(decs) = declarations;
    // decs.push();

    // shadertoy_declarations.into_iter().for_each(|d| {
    //     declarations.push(d);
    // });

    if let Some(mid) = mid {
        begin.extend(mid);
    }
    if let Some(end) = end {
        begin.extend(end);
    }

    begin
}

pub fn parse(root: TranslationUnit) -> TranslationUnit {
    match root {
        TranslationUnit(root_declarations) => {
            /// NOTE: this is not scalable at anything more compilcated than a single file i thinks
            /// Loop until we we find the main_image() def and then pass it to get its body yanked out
            let functions = Some(get_main(root_declarations.clone()));

            TranslationUnit(parse_declarations(
                get_shadertoy_defs(),
                None,
                functions,
            ))
        }
        _ => {
            panic!("wot???")
        }
    }
}
