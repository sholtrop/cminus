#include "../../support/fixture.h"
#include "../../support/testutil.h"
#include "gtest/gtest.h"

#include <string>
#include <syntax.h>
#include <types.h>
#include <cpp/support/globals.h>

class NodeOperatorTest : public SyntaxCorrectTest {};
class NodeOperatorBinaryTest : public NodeOperatorTest {};
class NodeOperatorUnaryTest : public NodeOperatorTest {};

static ghc::filesystem::path get_root() {
    return ghc::filesystem::absolute(my_argv[0]);
}

static ghc::filesystem::path get_execpath() {
    return get_root().parent_path().parent_path().parent_path() / "coco_compiler_syntax";
}

static ghc::filesystem::path get_prefix() {
    return get_root().parent_path().parent_path().parent_path().parent_path().parent_path().parent_path() / "src" / "syntax" / "src" / "test" / "c-minus" / "units" / "node" / "correct" / "operators";
}

const bool verbose = false;


TEST_F(NodeOperatorBinaryTest, rel_equal) {
    const auto testpath = get_prefix() / "logical" / "rel_equal.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    // equal operator
    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_eq = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_REL_EQUAL, RT_BOOL);
    rel_eq.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_eq.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return statement
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, rel_lt) {
    const auto testpath = get_prefix() / "logical" / "rel_lt.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // less than operator
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_REL_LT, RT_BOOL);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 11);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 99);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, rel_gt) {
    const auto testpath = get_prefix() / "logical" / "rel_gt.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // greater than operator
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_REL_GT, RT_BOOL);
    rel_lt.add_const<int>(NODE_NUM, RT_INT, 4000);
    rel_lt.add_const<int>(NODE_NUM, RT_INT, 400);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, rel_lte) {
    const auto testpath = get_prefix() / "logical" / "rel_lte.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // less than or equal operator
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_REL_LTE, RT_BOOL);
    rel_lt.add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 3);
    rel_lt.add_const<int>(NODE_NUM, RT_INT, 3000);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, rel_gte) {
    const auto testpath = get_prefix() / "logical" / "rel_gte.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // greater than or equal operator
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_REL_GTE, RT_BOOL);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 2);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 1);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, rel_notequal) {
    const auto testpath = get_prefix() / "logical" / "rel_notequal.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // not equal
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_REL_NOTEQUAL, RT_BOOL);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 120);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, add) {
    const auto testpath = get_prefix() / "math" / "add.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // addition
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_ADD, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}


TEST_F(NodeOperatorBinaryTest, sub) {
    const auto testpath = get_prefix() / "math" / "sub.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // subtraction
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_SUB, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, or) {
    const auto testpath = get_prefix() / "logical" / "or.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // or
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_OR, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, mul) {
    const auto testpath = get_prefix() / "math" / "mul.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // multiplication
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_MUL, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, div) {
    const auto testpath = get_prefix() / "math" / "div.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // divide
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_DIV, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, mod) {
    const auto testpath = get_prefix() / "math" / "mod.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // modulo
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_MOD, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorBinaryTest, and) {
    const auto testpath = get_prefix() / "logical" / "and.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // and
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    auto rel_lt = assignop.add_unary(NODE_COERCION, RT_INT).add_binary(NODE_AND, RT_INT8);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 42);
    rel_lt.add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorUnaryTest, signplus) {
    const auto testpath = get_prefix() / "math" / "signplus.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // sign plus
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    assignop.add_unary(NODE_COERCION, RT_INT).add_unary(NODE_SIGNPLUS, RT_INT8).add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorUnaryTest, signminus) {
    const auto testpath = get_prefix() / "math" / "signminus.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // sign minus
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    assignop.add_unary(NODE_COERCION, RT_INT).add_unary(NODE_SIGNMINUS, RT_INT8).add_const<int8_t>(NODE_NUM, RT_INT8, 4);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}

TEST_F(NodeOperatorUnaryTest, not ) {
    const auto testpath = get_prefix() / "logical" / "not.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));

    ASSERT_TRUE(test::function::root_available(tree, table, "main"));
    test::build::FunctionTreeBuilder builder("main", RT_INT,  3);

    auto assignop = builder.add_statement().add_binary(NODE_ASSIGNMENT, RT_VOID);

    // not operator
    assignop.add_symbol(NODE_ID, RT_INT, new Symbol("superglobal", 1, RT_INT, ST_VARIABLE));
    assignop.add_unary(NODE_COERCION, RT_INT).add_unary(NODE_NOT, RT_BOOL).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    // return
    builder.add_statement().add_unary(NODE_RETURN, RT_VOID).add_unary(NODE_COERCION, RT_INT).add_const<int8_t>(NODE_NUM, RT_INT8, 0);

    auto referenceFunction = builder.build();
    EXPECT_TRUE(test::syntax::syntax_similar(tree, table, referenceFunction, verbose));
}