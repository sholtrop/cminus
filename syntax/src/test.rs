// TEST_F(DeclarationsTest, simple_func) {
//     const auto testpath = get_prefix() / "correct" / "simple_func.c";
//     ASSERT_EQ(exitTest(get_execpath(), testpath), 0);
//     int parseResult = syntax::generate(testpath, tree, table, logger);
//     EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));
//     EXPECT_TRUE(test::function::exists(table, "main"));
// }
#[test]
fn test_simple_func() {}
