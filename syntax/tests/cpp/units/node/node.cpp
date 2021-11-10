#include "../../support/fixture.h"
#include "../../support/testutil.h"
#include "../../support/globals.h"
#include "gtest/gtest.h"

#include <ghc/filesystem.h>
#include <string>
#include <syntax.h>
#include <types.h>

#include <node.h>

/**
 * Tests to check syntax tree correctness. Every nodetype is tested. Note: Operators have been moved to `node_operator.cpp`, to make this file a bit more accessible.
 */
class NodeTest : public SyntaxCorrectTest {};

static ghc::filesystem::path get_root() {
    return ghc::filesystem::absolute(my_argv[0]);
}

static ghc::filesystem::path get_execpath() {
    return get_root().parent_path().parent_path().parent_path() / "coco_compiler_syntax";
}

static ghc::filesystem::path get_prefix() {
    return get_root().parent_path().parent_path().parent_path().parent_path().parent_path().parent_path() / "src" / "syntax" / "src" / "test" / "c-minus" / "units" / "node";
}

const bool verbose = false;

TEST_F(NodeTest, statementlist_empty) {
    const auto testpath = get_prefix() / "correct" / "statementlist_empty.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_empty(tree, table, "main"));
}

TEST_F(NodeTest, statementlist_funccall) {
    const auto testpath = get_prefix() / "correct" / "statementlist_funccall.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT, 1);
    auto funccall_stmt = builder.add_statement().add_binary(NODE_FUNCTIONCALL, RT_VOID);
    funccall_stmt.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    auto args_stmt = funccall_stmt.add_binary(NODE_EXPRLIST, RT_VOID);
    args_stmt.add_unary(NODE_COERCION, RT_INT)
             .add_const<int8_t>(NODE_NUM, RT_INT8, 10);
    args_stmt.add_empty();
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, assignment) {
    const auto testpath = get_prefix() / "correct" / "assignment.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));

    test::build::FunctionTreeBuilder builder("main", RT_INT, 3);
    auto assignment_stmt = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignment_stmt.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    assignment_stmt.add_unary(NODE_COERCION, RT_INT)
                   .add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, if) {
    const auto testpath = get_prefix() / "correct" / "if.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));

    test::build::FunctionTreeBuilder builder("main", RT_INT, 3);
    // Stage 0: Variables
    auto superglobal_var_id = builder.getTable()->addSymbol(new Symbol("superglobal", 1, RT_INT, ST_VARIABLE), 0 /* means that this var is a global var */);

    // Stage 1: The if-statement
    auto if_stmt = builder.add_statement().add_binary(NODE_IF, RT_VOID);
    auto boolexpr_stmt = if_stmt.add_binary(NODE_REL_GT, RT_BOOL);
    boolexpr_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    boolexpr_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    // Stage 2: Inside the if-statement
    auto list_first_stmt = if_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    // Stage 2a: The assignment statement
    auto assignment_stmt = list_first_stmt.add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignment_stmt.add_symbol(NODE_ID, RT_INT, superglobal_var_id);
    assignment_stmt.add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 12);
    //Stage 2b: The functioncall statement
    auto list_second_stmt = list_first_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    auto funccall_stmt = list_second_stmt.add_binary(NODE_FUNCTIONCALL, RT_VOID);
    funccall_stmt.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    auto args_stmt = funccall_stmt.add_binary(NODE_EXPRLIST, RT_VOID);
    args_stmt.add_symbol(NODE_ID, RT_INT, superglobal_var_id);
    args_stmt.add_empty();
    list_second_stmt.add_empty();

    builder.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 0);


    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, if_targets) {
    const auto testpath = get_prefix() / "correct" / "if_targets.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    test::build::FunctionTreeBuilder builder("main", RT_INT, 3);
    // Stage 0: The variables
    auto superglobal_var_id = builder.getTable()->addSymbol(new Symbol("superglobal", 1, RT_INT, ST_VARIABLE), 0 /* means that this var is a global var */);

    // Stage 1: The if-statement
    auto if_stmt = builder.add_statement().add_binary(NODE_IF, RT_VOID);
    auto boolexpr_stmt = if_stmt.add_binary(NODE_REL_GT, RT_BOOL);
    boolexpr_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    boolexpr_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto target_stmt = if_stmt.add_binary(NODE_IF_TARGETS, RT_VOID);

    auto target_stmt_true = target_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    // Stage 2: Inside the if-statement
    auto assignment_stmt_true = target_stmt_true.add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignment_stmt_true.add_symbol(NODE_ID, RT_INT, superglobal_var_id);
    assignment_stmt_true.add_unary(NODE_COERCION, RT_INT)
        .add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    target_stmt_true.add_empty();

    // Stage 2b: Inside the else-statement
    auto target_stmt_false = target_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    auto assignment_stmt_false = target_stmt_false.add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignment_stmt_false.add_symbol(NODE_ID, RT_INT, superglobal_var_id);
    assignment_stmt_false.add_unary(NODE_COERCION, RT_INT)
        .add_const<int8_t>(NODE_NUM, RT_INT8, 1);
    target_stmt_false.add_empty();

    // Stage 3: The functioncall statement
    auto funccall_stmt = builder.add_statement().add_binary(NODE_FUNCTIONCALL, RT_VOID);
    funccall_stmt.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    auto args_stmt = funccall_stmt.add_binary(NODE_EXPRLIST, RT_VOID);
    args_stmt.add_symbol(NODE_ID, RT_INT, superglobal_var_id);
    args_stmt.add_empty();

    // Stage 4: The return statement
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID)
        .add_unary(NODE_COERCION, RT_INT)
        .add_const<int8_t>(NODE_NUM, RT_INT8, 0);


    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, while) {
    const auto testpath = get_prefix() / "correct" / "while.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));


    test::build::FunctionTreeBuilder builder("main", RT_INT, 3);
    // Stage 1: The while-statement
    auto while_stmt = builder.add_statement().add_binary(NODE_WHILE, RT_VOID);
    auto boolexpr_stmt = while_stmt.add_binary(NODE_REL_LT, RT_BOOL);
    boolexpr_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    boolexpr_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto target_stmt = while_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);

    // Stage 2: Inside the while-statement
    auto funccall_stmt = target_stmt.add_binary(NODE_FUNCTIONCALL, RT_VOID);
    funccall_stmt.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    auto args_stmt = funccall_stmt.add_binary(NODE_EXPRLIST, RT_VOID);
    args_stmt.add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 22);
    args_stmt.add_empty();
    target_stmt.add_empty();

    // Stage 3: The return statement
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, array) {
    const auto testpath = get_prefix() / "correct" / "array.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    test::build::FunctionTreeBuilder builder("main", RT_INT, 3);
    // Stage 0: The variables
    auto superarray_var_id = builder.getTable()->addSymbol(new ArraySymbol("superarray", 1, RT_INT_ARRAY, ST_VARIABLE, /* array size */ 42), 0 /* means that this var is a global var */);

    // Stage 1: The assignment-statement
    auto assignment_stmt = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    auto larry_stmt = assignment_stmt.add_binary(NODE_LARRAY, RT_INT);
    larry_stmt.add_symbol(NODE_ID, RT_INT_ARRAY, superarray_var_id);
    larry_stmt.add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    assignment_stmt.add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 42);

    // Stage 2: The return statement
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 0);
    auto referenceFunction = builder.build();

    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, return) {
    const auto testpath = get_prefix() / "correct" / "return.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    test::build::FunctionTreeBuilder builder_t1("t1", RT_VOID, 3);
    // Stage 0: The variables
    auto superglobal_var_id = builder_t1.getTable()->addSymbol(new Symbol("superglobal", 1, RT_INT, ST_VARIABLE), 0 /* means that this var is a global var */);
    // Stage 1: the return statement
    builder_t1.add_statement().add_unary(NODE_RETURN, RT_VOID).add_empty();


    test::build::FunctionTreeBuilder builder_t2(builder_t1.getTable(), "t2", RT_INT, 7);
    // Stage 1: the return statement
    builder_t2.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_symbol(NODE_ID, RT_INT, superglobal_var_id);


    test::build::FunctionTreeBuilder builder_main(builder_t1.getTable(),"main", RT_INT, 10);
    // Stage 1: The assignment-statement
    auto assignment_stmt = builder_main.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignment_stmt.add_symbol(NODE_ID, RT_INT, superglobal_var_id);

    assignment_stmt.add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 42);

    // Stage 2: t1() funccall
    auto func_t1 = builder_main.add_statement().add_binary(NODE_FUNCTIONCALL, RT_VOID);
    func_t1.add_symbol(NODE_ID, RT_VOID, builder_main.getFunctionIdByName("t1"));
    func_t1.add_empty();

    // Stage 3: t2() funccall
    auto func_t2 = builder_main.add_statement().add_binary(NODE_FUNCTIONCALL, RT_INT);
    func_t2.add_symbol(NODE_ID, RT_INT, builder_main.getFunctionIdByName("t2"));
    func_t2.add_empty();

    // Stage 4: The return statement
    builder_main.add_statement().add_unary(NODE_RETURN, RT_VOID)
            .add_unary(NODE_COERCION, RT_INT)
            .add_const<int8_t>(NODE_NUM, RT_INT8, 0);
    auto referenceFunction = builder_main.build();

    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
    ////////////////
//    ASSERT_TRUE(test::function::root_available(tree, table, "t1"));
//    auto* t1_stmt_list = test::function::get_root(tree, table, "t1");
//
//    EXPECT_TRUE(test::syntax::nodetype_correct(t1_stmt_list->getRightChild(), NODE_EMPTY));
//    auto* t1_return = t1_stmt_list->getLeftChild();
//    ASSERT_TRUE(test::syntax::nodetype_correct(t1_return, NODE_RETURN));
//    EXPECT_TRUE(test::syntax::nodetype_correct(dynamic_cast<UnaryNode*>(t1_return)->getChild(), NODE_EMPTY));
//
//    ASSERT_TRUE(test::function::root_available(tree, table, "t2"));
//    auto* t2_stmt_list = test::function::get_root(tree, table, "t2");
//
//    EXPECT_TRUE(test::syntax::nodetype_correct(t2_stmt_list->getRightChild(), NODE_EMPTY));
//    auto* t2_return = t2_stmt_list->getLeftChild();
//    ASSERT_TRUE(test::syntax::nodetype_correct(t2_return, NODE_RETURN));
//    auto* t2_return_expr = dynamic_cast<UnaryNode*>(t2_return)->getChild();
//
//    ASSERT_TRUE(test::syntax::nodetype_correct(t2_return_expr, NODE_ID));
//    size_t var_reference_id = dynamic_cast<SymbolNode*>(t2_return_expr)->getSymbolId();
//    auto* global_sym = table.getSymbol(var_reference_id);
//    ASSERT_NE(global_sym, nullptr);
//    EXPECT_EQ(table.getSymbol(var_reference_id)->getName(), "superglobal");
}

TEST_F(NodeTest, funccall) {
    const auto testpath = get_prefix() / "correct" / "funccall.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT, 1);

    // function call
    auto func = builder.add_statement().add_binary(NODE_FUNCTIONCALL, RT_VOID);
    func.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    auto expr_list = func.add_binary(NODE_EXPRLIST, RT_VOID);
    expr_list.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 10);
    expr_list.add_empty();

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, exprlist) {
    const auto testpath = get_prefix() / "correct" / "expression_list.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT, 1);

    // function call
    auto func = builder.add_statement().add_binary(NODE_FUNCTIONCALL, RT_VOID);
    func.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    auto expr_list = func.add_binary(NODE_EXPRLIST, RT_VOID);
    expr_list.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 10);
    expr_list.add_empty();

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, id) {
    const auto testpath = get_prefix() / "correct" / "assignment.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT, 3);

    // assignment
    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    assignop.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 42);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeTest, coercion) {
    const auto testpath = get_prefix() / "correct" / "coercion.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT, 2);

    // assignment
    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    size_t smallint = assignop.add_symbol(NODE_ID, RT_UINT8, new Symbol("smallint", 4, RT_UINT8, ST_VARIABLE));
    assignop.add_const<uint8_t>(NODE_NUM, RT_UINT8, 248);

    // coercion-assignment
    auto coerce_assign = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    coerce_assign.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    coerce_assign.add_unary(NODE_COERCION, RT_INT).add_symbol(NODE_ID, RT_UINT8, smallint);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}



/**
 * Tests to check syntax tree failures. For clarity: Every test here should succeed. Inside each test, we check if the called syntaxtree builder fails correctly.
 */
class NodeTestError : public SyntaxErrorTest {};

TEST_F(NodeTestError, global_call) {
    const auto testpath = get_prefix() / "incorrect" / "global_call.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath, false, true), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_errors(tree, table, logger, parseResult, 1));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT, 2);

    auto func = builder.add_statement().add_binary(NODE_FUNCTIONCALL, RT_ERROR);
    func.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    func.add_empty();

    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}