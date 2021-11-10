#include "support/fixture.h"
#include "gtest/gtest.h"
#include <dynamic_test.h>
#include <ghc/filesystem.h>

int my_argc;
char** my_argv;

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    my_argc = argc;
    my_argv = argv;

    ghc::filesystem::path root_path = ghc::filesystem::canonical(ghc::filesystem::absolute(argv[0])); // Distro-dependent method to get executable-path.
    std::cerr << "Test root path is: " << root_path << std::endl;

    ghc::filesystem::path project_root_path = root_path.parent_path().parent_path().parent_path().parent_path().parent_path().parent_path(); // Project root path.

    ghc::filesystem::path exe_path = root_path.parent_path().parent_path().parent_path() / "coco_compiler_syntax"; // Absolute path to regular executable.
    if (!ghc::filesystem::exists(exe_path))
        throw std::runtime_error("Path does not exist: "+exe_path.string());
    ghc::filesystem::path test_path_general = project_root_path / "test" / "c-minus"; // Absolute path to general tests.
    if (!ghc::filesystem::exists(test_path_general))
        throw std::runtime_error("Path does not exist: "+test_path_general.string());
    ghc::filesystem::path test_path_specific = project_root_path / "src" / "syntax" / "src" / "test" / "c-minus"; // Absolute path to syntax tests.
    if (!ghc::filesystem::exists(test_path_specific))
        throw std::runtime_error("Path does not exist: "+test_path_specific.string());


    ////////////////////// Execute global tests //////////////////////
    test::support::register_tests<SyntaxTest>(project_root_path, exe_path, test_path_general, "DynamicSyntaxCorrectTest", {"incorrect", "warn"}, [](const std::string& exe_path, const std::string& test_path) -> SyntaxTest* {
        return new DynamicSyntaxCorrectTest(exe_path, test_path);
    });

    ////////////////////// Execute syntax-specific tests //////////////////////
    test::support::register_tests<SyntaxTest>(project_root_path, exe_path, test_path_specific, "DynamicSyntaxCorrectTest", {"incorrect", "warn"}, [](const std::string& exe_path, const std::string& test_path) -> SyntaxTest* {
        return new DynamicSyntaxCorrectTest(exe_path, test_path);
    });

    test::support::register_tests<SyntaxTest>(project_root_path, exe_path, test_path_specific, "DynamicSyntaxErrorTest", {"correct", "warn"}, [](const std::string& exe_path, const std::string& test_path) -> SyntaxTest* {
        return new DynamicSyntaxErrorTest(exe_path, test_path);
    });

    test::support::register_tests<SyntaxTest>(project_root_path, exe_path, test_path_specific, "DynamicSyntaxWarnTest", {"correct", "incorrect"}, [](const std::string& exe_path, const std::string& test_path) -> SyntaxTest* {
        return new DynamicSyntaxWarnTest(exe_path, test_path);
    });
    return RUN_ALL_TESTS();
}
