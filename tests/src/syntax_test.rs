use std::io;
use syntax::{NodeType, SyntaxAnalysisResult};
use tests::{collect_tests_in_path, run_single_test, TestStats};

const PROGRAM_TEST_PATH: &str = "tests/testfiles/general/programs";
const UNIT_TEST_PATH: &str = "tests/testfiles/general/units";
const SYNTAX_TEST_PATH: &str = "tests/testfiles/syntax";

mod specific_tests {
    use itertools::{self, EitherOrBoth, Itertools};
    use syntax::SyntaxNode;

    const GLOBAL_PREFIX: &str = "tests/testfiles/general/units/";
    fn read_to_string(path: &str) -> String {
        std::fs::read_to_string(path).expect("Could not read file")
    }

    fn syntax_similar<'a>(
        expectation: impl IntoIterator<Item = &'a str>,
        actual: impl IntoIterator<Item = &'a SyntaxNode>,
    ) -> bool {
        for (idx, pair) in expectation
            .into_iter()
            .zip_longest(actual.into_iter())
            .enumerate()
        {
            match pair {
                EitherOrBoth::Both(l, r) => {
                    if l != r.to_string() {
                        log::error!(
                            "Nodes not similar at position {}. Expected: `{}` | Actual: `{}`",
                            idx + 1,
                            l,
                            r
                        );
                        return false;
                    }
                }
                EitherOrBoth::Right(r) => {
                    log::error!("Syntax was not similar: Actual had more nodes than expected tree. Last actual node: {}", r);
                    return false;
                }
                EitherOrBoth::Left(l) => {
                    log::error!("Syntax was not similar: Expectation had more nodes than actual tree. Last expected node: {}", l);
                    return false;
                }
            }
        }
        true
    }
    pub mod declaration {
        use super::*;
        const PREFIX: &str = "declarations/";
        pub fn simple_func() -> bool {
            let test_path = GLOBAL_PREFIX.to_owned() + PREFIX + "correct/simple_func.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let table = result.symbol_table;
            assert!(table.has_function("main"));
            true
        }
        pub fn simple_func_param() -> bool {
            let test_path =
                GLOBAL_PREFIX.to_owned() + PREFIX + "correct/simple_func_param/simple_func_param.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let table = result.symbol_table;
            assert!(table.has_function("main"));
            assert!(table.has_function("oof"));
            assert!(table.has_parameter("oof", "x"));
            assert!(table.has_parameter("oof", "y"));
            true
        }
        pub fn simple_var_assign() -> bool {
            let test_path =
                GLOBAL_PREFIX.to_owned() + PREFIX + "correct/simple_var_assign/simple_var_assign.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let table = result.symbol_table;
            assert!(table.has_local("main", "x"));
            true
        }
    }

    pub mod node_coercion {
        use super::*;
        const PREFIX: &str = "tests/testfiles/syntax/node/correct/coercion/";

        #[rustfmt::skip]
        pub fn if_coerce() -> bool {
            let test_path = PREFIX.to_owned() + "if_coercion.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            let expectation = [
                "statement_list - void",
                    "assignment - int",
                        "symbol - int",
                        "coercion - int",
                        "num - 1",

                "statement_list - void",
                    "if - void",
                        "coercion - bool",
                        "symbol - int",
                    "statement_list - void",
                    "assignment - int",
                        "symbol - int",
                        "coercion - int",
                        "num - 12",
                    "statement_list - void",
                    "function_call - void", // writeinteger
                        "symbol - void",
                        "expression_list - void",
                            "symbol - int",
                "statement_list - void",
                    "return - int",
                        "coercion - int",
                        "num - 0"
            ];
            assert!(syntax_similar(expectation, tree));
            true
        }

        #[rustfmt::skip]
        pub fn while_coerce() -> bool {
            let test_path = PREFIX.to_owned() + "while_coercion.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            let expectation = [
                "statement_list - void",
                    "assignment - int",
                        "symbol - int",
                        "coercion - int",
                            "num - 1",
                "statement_list - void",
                    "while - void",
                        "coercion - bool",
                            "symbol - int",
                        "statement_list - void",
                            "assignment - int",
                                "symbol - int",
                                "sub - int",
                                    "symbol - int",
                                    "coercion - int",
                                        "num - 1",
                        "statement_list - void",
                            "function_call - void",
                                "symbol - void", // writeinteger
                                "expression_list - void",
                                    "coercion - int",
                                        "num - 22",
                "statement_list - void",
                    "return - int",
                        "coercion - int",
                            "num - 0"
            ];
            assert!(syntax_similar(expectation, tree));
            true
        }
    }

    pub mod node {
        use super::*;
        const PREFIX: &str = "tests/testfiles/syntax/node/correct/";

        #[rustfmt::skip]
        pub fn statementlist_empty() -> bool {
            let test_path = PREFIX.to_owned() + "statementlist_empty.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            assert!(syntax_similar(["[EMPTY]"], tree));
            true
        }

        #[rustfmt::skip]
        pub fn statementlist_functioncall() -> bool {
            let test_path = PREFIX.to_owned() + "statementlist_funccall.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            let expectation = [
                "statement_list - void",
                    "function_call - void", // writeinteger
                        "symbol - void",
                        "expression_list - void",
                            "coercion - int",
                                "num - 10",
                "statement_list - void",
                    "return - int",
                        "coercion - int",
                            "num - 0"
            ];
            assert!(syntax_similar(expectation, tree));
            true
        }

        #[rustfmt::skip]
        pub fn assignment() -> bool {
            let test_path = PREFIX.to_owned() + "assignment.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            let expectation = [
                "statement_list - void",
                    "assignment - int",
                        "symbol - int",
                        "coercion - int",
                            "num - 42",
                "statement_list - void",
                    "return - int",
                        "coercion - int",
                            "num - 0"
            ];
            assert!(syntax_similar(expectation, tree));
            true
        }

        #[rustfmt::skip]
        pub fn if_targets() -> bool {
            let test_path = PREFIX.to_owned() + "if_targets.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            let expectation = [
                "statement_list - void",
                    "if - void",
                        "rel_gt - bool",
                            "num - 42",
                            "num - 0",
                        "if_targets - void",
                            // if-true
                            "statement_list - void",
                                "assignment - int",
                                    "symbol - int",
                                    "coercion - int",
                                        "num - 42",
                            // else-true
                            "statement_list - void",
                                "assignment - int",
                                    "symbol - int",
                                    "coercion - int",
                                        "num - 1",
                "statement_list - void",
                    "function_call - void",
                        "symbol - void", // writeinteger,
                        "expression_list - void",
                            "symbol - int",
                "statement_list - void",
                    "return - int",
                    "coercion - int",
                        "num - 0"
            ];
            assert!(syntax_similar(expectation, tree));
            true
        }

        #[rustfmt::skip]
        pub fn array() -> bool {
            let test_path = PREFIX.to_owned() + "array.c";
            log::info!("Running test {}", test_path);
            let input = read_to_string(&test_path);
            let result = syntax::generate(&input);
            assert!(result.is_ok());
            let result = result.unwrap();
            let main = result.tree.get_func_by_name("main").unwrap();
            assert!(main.tree.is_some());
            let tree = main.tree.as_ref().unwrap().preorder();
            let expectation = [
                "statement_list - void",
                    "assignment - int",
                        "l_array - int",
                            "symbol - int_array",
                            "num - 0",
                        "coercion - int",
                        "num - 42",
                "statement_list - void",
                    "return - int",
                        "coercion - int",
                            "num - 0"
            ];
            assert!(syntax_similar(expectation, tree));
            true
        }
    }

    pub const ALL_TESTS: [fn() -> bool; 10] = [
        declaration::simple_func,
        declaration::simple_func_param,
        declaration::simple_var_assign,
        node_coercion::if_coerce,
        node_coercion::while_coerce,
        node::statementlist_empty,
        node::statementlist_functioncall,
        node::assignment,
        node::if_targets,
        node::array,
    ];
}

pub fn test_function(input: &str) -> Result<(), &str> {
    let result = syntax::generate(input);
    if let Err(err) = result {
        log::error!("{}", err);
        return Err("Error occurred");
    }
    let SyntaxAnalysisResult { tree, errors, .. } = result.unwrap();
    if !errors.is_empty() {
        return Err("Errors present");
    }
    for (id, func) in tree.functions {
        for node in func
            .tree
            .ok_or_else(|| {
                log::error!("Rootless function {}", id);
                "Error occurred"
            })?
            .preorder()
        {
            if node.node_type() == NodeType::Error {
                return Err("Error node found");
            }
        }
    }

    Ok(())
}

pub fn run() -> io::Result<()> {
    let mut stats = TestStats {
        total: 0,
        success: 0,
    };
    let unit_tests = collect_tests_in_path(UNIT_TEST_PATH)?.into_iter();
    let program_tests = collect_tests_in_path(PROGRAM_TEST_PATH)?.into_iter();
    let lex_tests = collect_tests_in_path(SYNTAX_TEST_PATH)?.into_iter();

    for test in unit_tests.chain(program_tests).chain(lex_tests) {
        stats.total += 1;
        if run_single_test(test, test_function).is_ok() {
            stats.success += 1;
        }
    }
    log::info!("[{} / {}] GENERAL TESTS PASSED", stats.success, stats.total);
    stats.success = 0;
    stats.total = 0;
    println!();
    log::info!("Running specific tests...");

    for test in specific_tests::ALL_TESTS {
        stats.total += 1;
        if test() {
            stats.success += 1;
            log::info!("↪    PASSED");
        } else {
            log::error!("↪    FAILED");
        }
    }

    log::info!(
        "[{} / {}] SPECIFIC TESTS PASSED",
        stats.success,
        stats.total
    );
    Ok(())
}
