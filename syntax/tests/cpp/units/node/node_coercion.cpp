#include "../../support/fixture.h"
#include "../../support/testutil.h"
#include "gtest/gtest.h"

#include <string>
#include <syntax.h>
#include <types.h>
#include <cpp/support/globals.h>

class NodeCoercionTest : public SyntaxCorrectTest {};

static ghc::filesystem::path get_root() {
    return ghc::filesystem::absolute(my_argv[0]);
}

static ghc::filesystem::path get_execpath() {
    return get_root().parent_path().parent_path().parent_path() / "coco_compiler_syntax";
}

static ghc::filesystem::path get_prefix() {
    return get_root().parent_path().parent_path().parent_path().parent_path().parent_path().parent_path() / "src" / "syntax" / "src" / "test" / "c-minus" / "units" / "node" / "correct" / "coercion";
}

const bool verbose = false;

TEST_F(NodeCoercionTest, if_coerce) {
    //TODO: finish test
    const auto testpath = get_prefix() / "if_coercion.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    // assignment
    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    size_t a_id = assignop.add_symbol(NODE_ID, RT_INT, new Symbol("a", 4, RT_INT, ST_VARIABLE));
    assignop.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 1);

    // if block
    auto if_stmt = builder.add_statement().add_binary(NODE_IF, RT_VOID);
    if_stmt.add_unary(NODE_COERCION, RT_BOOL).add_symbol(NODE_ID, RT_INT, a_id);
    auto if_stmt1 = if_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    auto if_assign = if_stmt1.add_binary(NODE_ASSIGNMENT, RT_VOID);
    auto super_id = if_assign.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    if_assign.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 12);
    auto if_stmt2 = if_stmt1.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    auto writecall = if_stmt2.add_binary(NODE_FUNCTIONCALL, RT_VOID);
    writecall.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    writecall.add_symbol(NODE_ID, RT_INT, super_id);
    if_stmt2.add_empty();

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeCoercionTest, while_coerce) {
    const auto testpath = get_prefix() / "while_coercion.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  1);

    // assignment
    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    size_t a_id = assignop.add_symbol(NODE_ID, RT_INT, new Symbol("a", 4, RT_INT, ST_VARIABLE));
    assignop.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 1);

    // while block
    auto while_stmt = builder.add_statement().add_binary(NODE_WHILE, RT_VOID);
    while_stmt.add_unary(NODE_COERCION, RT_BOOL).add_symbol(NODE_ID, RT_INT, a_id);
    auto while_stmt1 = while_stmt.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    auto while_assign = while_stmt1.add_binary(NODE_ASSIGNMENT, RT_VOID);
    while_assign.add_symbol(NODE_ID, RT_INT, a_id);
    auto while_sub = while_assign.add_binary(NODE_SUB, RT_VOID);
    while_sub.add_symbol(NODE_ID, RT_INT, a_id);
    while_sub.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 1);
    auto while_stmt2 = while_stmt1.add_binary(NODE_STATEMENT_LIST, RT_VOID);
    auto writecall = while_stmt2.add_binary(NODE_FUNCTIONCALL, RT_VOID);
    writecall.add_symbol(NODE_ID, RT_VOID, builder.getFunctionIdByName("writeinteger"));
    writecall.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 22);
    while_stmt2.add_empty();

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}


