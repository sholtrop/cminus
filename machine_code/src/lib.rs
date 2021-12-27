use intermediate_code::ic_generator::Intermediate;
use intermediate_code::ic_generator::OptLevel;
use syntax::SyntaxAnalysisResult;

pub mod allocator;
pub mod code_emitter;
pub mod code_generator;
pub mod register;

pub fn compile_file(input_path: &str) -> Result<String, &str> {
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
        return Err("Syntax errors encountered");
    }
    syntax::display_warnings(&warnings);
    let icode = intermediate_code::generate(&tree, &mut symbol_table, OptLevel::None).unwrap();
    let machine_code = generate(&icode);
    Ok(machine_code)
}

pub fn generate(icode: &Intermediate) -> String {
    "".into()
}
