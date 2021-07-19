use atory;
use glsl::syntax::Statement;
use glsl::syntax::{ExternalDeclaration, NonEmpty};
use macroquad::prelude::*;
use std::collections::HashMap;
use glsl::syntax::TranslationUnit;

#[macroquad::main("Quadtoy")]
async fn main() {
    let mut expression = String::new();
    let shader_ast: NonEmpty<ExternalDeclaration> = atory::load_glsl_shader("test.frag");
    let shader_ast_tu = TranslationUnit(shader_ast.clone());

    let mut enabled_ast_nodes: HashMap<String, bool> = HashMap::new();

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Glsl AST")
                .scroll(true)
                .show(egui_ctx, |ui| {
                    for d in shader_ast.clone().0.iter() {
                        println!("Node");
                        match d {
                            ExternalDeclaration::Preprocessor(pre) => {
                                ui.label("Preprocessor");
                                ui.selectable_label(true, &format!("{:?}", pre)[0..5]);
                            }
                            ExternalDeclaration::FunctionDefinition(def) => {
                                let mut func_def = String::new();
                                // glsl::transpiler::glsl::show_function_definition(&mut func_def, def);

                                let functype = &def.prototype.ty;
                                glsl::transpiler::glsl::show_fully_specified_type(
                                    &mut func_def,
                                    functype,
                                );
                                func_def.push(' ');

                                let name = &def.prototype.name;
                                glsl::transpiler::glsl::show_identifier(&mut func_def, name);
                                let params = &def.prototype.parameters;

                                func_def.push(' ');
                                func_def.push('(');

                                for p in params {
                                    glsl::transpiler::glsl::show_function_parameter_declaration(
                                        &mut func_def,
                                        p,
                                    );
                                    func_def.push(' ');
                                    func_def.push(',');
                                }

                                func_def.push(')');

                                // ui.selectable_label(true, &format!("{:?} \n {:?} \n {:?}", functype, name ,params));
                                let label = &format!("{}", func_def);
                                let new_val = if let Some(true) = enabled_ast_nodes.get(&label.to_string()) {true} else {false};
                                if ui.selectable_label(new_val, label).clicked() {
                                    enabled_ast_nodes.insert(label.to_string(),  !new_val);
                                }

                                if let Some(true) = enabled_ast_nodes.get(&label.to_string()) {
                                    let func_body = String::new();
                                    for s in def.statement.statement_list.iter() {
                                        let mut out = String::new();
                                        glsl::transpiler::glsl::show_statement(&mut out, s);
                                        ui.selectable_label(false, &format!("     {}", out));
                                        // match s {
                                        //     Statement::Simple(a) => {
                                        //     },
                                        //     Statement::Compound(a) => {},
                                        // }
                                    }
                                }
                            }
                            ExternalDeclaration::Declaration(dec) => {
                                //     ui.label("Declarations");
                                //     ui.selectable_label(true, &format!("{:?}", dec)[0..100]);
                            }
                        }
                    }
                });

            let mut ast_txt = String::new();
            glsl::transpiler::glsl::show_translation_unit(&mut ast_txt,&shader_ast_tu);
            egui::Window::new("AST Expression").scroll(true).show(egui_ctx, |ui| {
                ui.code_editor(&mut ast_txt);
            });
        });

        // Draw things before egui

        egui_macroquad::draw();
        next_frame().await;
    }
}
