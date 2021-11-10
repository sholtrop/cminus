/*
_ _ _ ____ ____ _  _ _ _  _ ____     ___  ____    _  _ ____ ___
| | | |__| |__/ |\ | | |\ | | __ .   |  \ |  |    |\ | |  |  |
|_|_| |  | |  \ | \| | | \| |__] .   |__/ |__|    | \| |__|  |

____ _  _ ____ _  _ ____ ____    ____ _ _    ____   /
|    |__| |__| |\ | | __ |___    |___ | |    |___  /
|___ |  | |  | | \| |__] |___    |    | |___ |___ .
*/

#ifndef COCO_FRAMEWORK_TEST_SYNTAX_FIXTURE
#define COCO_FRAMEWORK_TEST_SYNTAX_FIXTURE

#include "testutil.h"
#include "gtest/gtest.h"
#include <test.h>

#include <logger.h>
#include <symboltable.h>
#include <syntaxtree.h>

#include <ostream>
#include <syntax.h>

class SyntaxTest : public CapturingLoggingTest {
    protected:
    SyntaxTest(std::ostream& infostream, std::ostream& warnstream, std::ostream& errorstream) : logger(infostream, warnstream, errorstream) {}
    ~SyntaxTest() override = default;

    /**
     * Simple function to test whether system is segfault-free. If we encounter a non-zero exit code, a segfault occurred.
     * @return Process exit code.
     */
    static int exitTest(const std::string& exepath, const std::string& testpath, bool no_warn=true, bool no_error=true) {
        std::stringstream ss;
        ss << exepath << " --no-print -f \"" << testpath << "\"";
        if (no_warn)
            ss << " --no-warn";
        if (no_error)
            ss << " --no-error";
        int exitCode = std::system(ss.str().c_str());
        return exitCode;
    }
    
    Logger logger;
    SymbolTable table{};
    SyntaxTree tree{};
};

class SyntaxCorrectTest : public SyntaxTest {
    protected:
    SyntaxCorrectTest() : SyntaxTest(std::cerr, std::cerr, std::cerr) {}
};

class SyntaxErrorTest : public SyntaxTest {
protected:
    SyntaxErrorTest() : SyntaxTest(std::cerr, std::cerr, NULL_STREAM) {}
};

class DynamicSyntaxTest : public SyntaxTest {
    protected:
    explicit DynamicSyntaxTest(std::ostream& infostream, std::ostream& warnstream, std::ostream& errorstream, const std::string& exepath, const std::string& testpath) : SyntaxTest(infostream, warnstream, errorstream), exepath(exepath), testpath(testpath){};
    const std::string& exepath; // Path to the regular syntax executable.
    const std::string& testpath; // Path to the file we are currently testing.
};

class DynamicSyntaxCorrectTest : public DynamicSyntaxTest {
    public:
    explicit DynamicSyntaxCorrectTest(const std::string& exepath, const std::string& testpath) : DynamicSyntaxTest(std::cerr, std::cerr, std::cerr, exepath, testpath) {}

    /**
     * Generic test body like present in many `TEST` and `TEST_F` cases.
     * This implementation checks whether the file pointed to by `path` can be converted to a syntaxtree without warnings, errors, segfaults.
     */
    void TestBody() override {
        ASSERT_EQ(exitTest(exepath, testpath), 0);
        int parseResult = syntax::generate(testpath, tree, table, logger);
        EXPECT_TRUE(test::has_success(tree, table, logger, parseResult));
        EXPECT_TRUE(test::function::root_available(tree, table, "main") || test::function::root_empty(tree, table, "main"));
    }
};

class DynamicSyntaxErrorTest : public DynamicSyntaxTest {
    public:
    explicit DynamicSyntaxErrorTest(const std::string& exepath, const std::string& testpath) : DynamicSyntaxTest(std::cerr, std::cerr, NULL_STREAM, exepath, testpath) {}

    /**
     * Generic test body like present in many `TEST` and `TEST_F` cases.
     * This implementation checks whether the file pointed to by `path` can be converted to a syntaxtree with errors, without segfaults.
     */
    void TestBody() override {
        ASSERT_EQ(exitTest(exepath, testpath, false, true), 0);
        int parseResult = syntax::generate(testpath, tree, table, logger);
        EXPECT_TRUE(test::has_errors(tree, table, logger, parseResult));
        EXPECT_EQ(parseResult, 0);
    }
};

class DynamicSyntaxWarnTest : public DynamicSyntaxTest {
    public:
    explicit DynamicSyntaxWarnTest(const std::string& exepath, const std::string& testpath) : DynamicSyntaxTest(std::cerr, std::cerr, std::cerr, exepath, testpath) {}

    /**
     * Generic test body like present in many `TEST` and `TEST_F` cases.
     * This implementation checks whether the file pointed to by `path` can be converted to a syntaxtree with, warnings, and without errors, segfaults.
     */
    void TestBody() override {
        ASSERT_EQ(exitTest(exepath, testpath, true, false), 0);
        int parseResult = syntax::generate(testpath, tree, table, logger);
        EXPECT_TRUE(test::has_warnings(tree, table, logger, parseResult));
    }
};
#endif