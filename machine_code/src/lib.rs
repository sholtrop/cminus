use allocator::OutStream;
use code_generator::CodeGenerator;
use intermediate_code::ic_generator::Intermediate;
use intermediate_code::ic_generator::OptLevel;
use intermediate_code::icode::IntermediateCode;
use syntax::SymbolTable;
use syntax::SyntaxAnalysisResult;

pub mod allocator;
pub mod code_emitter;
pub mod code_generator;
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
        Box::new(std::fs::File::create(path).unwrap())
    } else {
        let path = input_path.split('.').next().unwrap();
        Box::new(std::fs::File::create(path).unwrap())
    };
    generate(&intermediate, &symbol_table, out);
    Ok(())
}

pub fn generate(intermediate: &Intermediate, table: &SymbolTable, out: OutStream) {
    let mut cg = CodeGenerator::new(out);
    let Intermediate { graph, icode } = intermediate;
    cg.generate_header();
    cg.generate_global_decls(table);
    cg.generate_code(table, icode, graph);
    cg.generate_trailer();
}
