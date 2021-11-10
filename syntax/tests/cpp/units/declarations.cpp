#include "../support/fixture.h"
#include "../support/testutil.h"
#include "gtest/gtest.h"

#include <ghc/filesystem.h>
#include <string>
#include <syntax.h>
#include <cpp/support/globals.h>

class DeclarationsTest : public SyntaxCorrectTest {};

/**
 * Tests to check simple declarations of functions, globals, parameters, variables, returns.
 */

static ghc::filesystem::path get_root() {
    return ghc::filesystem::absolute(my_argv[0]);
}

static ghc::filesystem::path get_execpath() {
    return get_root().parent_path().parent_path().parent_path() / "coco_compiler_syntax";
}

static ghc::filesystem::path get_prefix() {
    return get_root().parent_path().parent_path().parent_path().parent_path().parent_path().parent_path() / "test" / "c-minus" / "units" / "declarations";
}

TEST_F(DeclarationsTest, simple_func) {
    const auto testpath = get_prefix() / "correct" / "simple_func.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);
    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));
    EXPECT_TRUE(test::function::exists(table, "main"));
}

TEST_F(DeclarationsTest, simple_func_param) {
    const auto testpath = get_prefix() / "correct" / "simple_func_param" / "simple_func_param.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);

    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));
    EXPECT_TRUE(test::function::exists(table, "main"));
    ASSERT_TRUE(test::function::exists(table, "oof"));
    EXPECT_TRUE(test::variable::param_exists(table, "oof", "x"));
    EXPECT_TRUE(test::variable::param_exists(table, "oof", "y"));
}

TEST_F(DeclarationsTest, simple_var_assign) {
    const auto testpath = get_prefix() / "correct" / "simple_var_assign" / "simple_var_assign.c";
    ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
    int parseResult = syntax::generate(testpath, tree, table, logger);

    EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));
    ASSERT_TRUE(test::variable::local_exists(table, "main", "x"));
}