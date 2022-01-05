use std::cell::RefCell;
use std::rc::Rc;

use crate::output::OutStream;
use code_generator::CodeGenerator;
use intermediate_code::ic_generator::Intermediate;
use intermediate_code::ic_generator::OptLevel;
use syntax::SymbolTable;
use syntax::SyntaxAnalysisResult;

pub mod assembly;
pub mod code_generator;
pub mod emitter;
pub mod global_alloc;
pub mod output;
pub mod reg_alloc;
pub mod register;

pub fn compile_file(input_path: &str, output_path: Option<&str>) -> Result<(), String> {
    let file = std::fs::read_to_string(input_path).unwrap();
    let SyntaxAnalysisResult {
        errors,
        warnings,
        tree,
        mut symbol_table,
    } = syntax::generate(&file).unwrap();
    let has_errors = !errors.is_empty();
    if has_errors {
        syntax::display_errors(&errors);
        return Err("Syntax errors encountered".into());
    }
    syntax::display_warnings(&warnings);
    let intermediate =
        intermediate_code::generate(&tree, &mut symbol_table, OptLevel::None).unwrap();
    let out = if let Some(path) = output_path {
        std::fs::File::create(path).unwrap()
    } else {
        let path = input_path.split('.').next().unwrap();
        std::fs::File::create(format!("{}.S", path)).unwrap()
    };
    let out = Rc::new(RefCell::new(out)) as OutStream;
    generate(&intermediate, &symbol_table, out);
    Ok(())
}

pub fn generate(intermediate: &Intermediate, table: &SymbolTable, out: OutStream) {
    let mut cg = CodeGenerator::new(out, table);
    let Intermediate { graph, icode } = intermediate;
    cg.generate_header();
    cg.generate_global_decls(table);
    cg.generate_code(table, icode, graph);
    cg.generate_trailer();
}
