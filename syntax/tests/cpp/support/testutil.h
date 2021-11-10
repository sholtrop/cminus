/*
_ _ _ ____ ____ _  _ _ _  _ ____     ___  ____    _  _ ____ ___
| | | |__| |__/ |\ | | |\ | | __ .   |  \ |  |    |\ | |  |  |
|_|_| |  | |  \ | \| | | \| |__] .   |__/ |__|    | \| |__|  |

____ _  _ ____ _  _ ____ ____    ____ _ _    ____   /
|    |__| |__| |\ | | __ |___    |___ | |    |___  /
|___ |  | |  | | \| |__] |___    |    | |___ |___ .
*/

#ifndef COCO_FRAMEWORK_SYNTAX_TESTUTIL
#define COCO_FRAMEWORK_SYNTAX_TESTUTIL

#include "gtest/gtest.h"

#include "testutil_internal.h"
#include <test.h>
#include <limits>
#include <numeric>
#include <memory>

#include <algorithm>
#include <hacks/template_hacks.h>
#include <logger.h>
#include <node.h>
#include <symboltable.h>
#include <syntaxtree.h>
#include <types.h>

namespace test {

    namespace function {
        /**
         * Determines whether a function exists.
         * @param table Symboltable after parsing.
         * @param func_name Name of function to check for.
         * @return `testing::AssertionSuccess()` on success, `testing::AssertionFailure()` otherwise.
         */
        inline testing::AssertionResult exists(const SymbolTable& table, const std::string& func_name) {
            if (internal::function::exists(table, func_name))
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Function with name '" << func_name << "' not found";
        }

        inline testing::AssertionResult exists_id(const SymbolTable& table, const size_t& id) {
            if (internal::function::exists_id(table, id))
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Function with id '" << id << "' not found";
        }

        inline size_t get(const SymbolTable& table, const std::string& func_name) {
            for (const auto& x: table.getFunctions()) {
                const auto* sym = table.getSymbol(x);
                if (sym->getName() == func_name)
                    return x;
            }
            return std::numeric_limits<size_t>::max();
        }

        inline BinaryNode* get_root(const SyntaxTree& tree, const SymbolTable& table, const std::string& func_name) {
            size_t id = get(table, func_name);
            if (id == std::numeric_limits<size_t>::max())
                return nullptr;
            Node* root = tree.getRoot(id);
            return dynamic_cast<BinaryNode*>(root);
        }

        inline testing::AssertionResult root_empty(const SyntaxTree& tree, const SymbolTable& table, const std::string& func_name) {
            size_t id = get(table, func_name);
            if (id == std::numeric_limits<size_t>::max())
                return testing::AssertionFailure() << "Function with name '" << func_name << "' not found";
            Node* root = tree.getRoot(id);
            if (root == nullptr)
                return testing::AssertionFailure() << "Could not find function '" << func_name << "' (id " << id << ") in the tree";
            if (root->getNodeType() == NODE_EMPTY)
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Provided node has incorrect type '" << util::to_string(root->getNodeType()) << "' (expected '" << util::to_string(NODE_EMPTY) << "')";
        }

        inline testing::AssertionResult root_available(const SyntaxTree& tree, const SymbolTable& table, const std::string& func_name) {
            size_t id = get(table, func_name);
            if (id == std::numeric_limits<size_t>::max())
                return testing::AssertionFailure() << "Function with name '" << func_name << "' not found";
            Node* root = tree.getRoot(id);
            if (root == nullptr)
                return testing::AssertionFailure() << "Could not find function '" << func_name << "' (id " << id << ") in the tree";
            if (root->getNodeType() == NODE_STATEMENT_LIST)
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Provided node has incorrect type '" << util::to_string(root->getNodeType()) << "' (expected '" << util::to_string(NODE_STATEMENT_LIST) << "')";
        }
    }

    namespace variable {
        /**
         * Determines whether a local variable exists.
         * @param table Symboltable after parsing.
         * @param func_name Name of function to check for.
         * @return `testing::AssertionSuccess()` on success, `testing::AssertionFailure()` otherwise.
         */
        inline testing::AssertionResult local_exists(const SymbolTable& table, const std::string& func_name, const std::string& var_name) {
            const auto* sym = internal::variable::local_get(table, func_name, var_name);
            if (!sym) {
                if (!internal::function::exists(table, func_name))
                    return testing::AssertionFailure() << "Function with name '" << func_name << "' not found";
                else
                    return testing::AssertionFailure() << "Function '" << func_name << "' has no variable '" << var_name << "'";
            }
            if (sym->getSymbolType() == ST_VARIABLE)
                return testing::AssertionSuccess();
            else
                return testing::AssertionFailure() << "Function '" << func_name << "' has no variable '" << var_name << "'. Found only equivalently named variable, with type " << util::to_string(sym->getSymbolType());
        }

        inline testing::AssertionResult param_exists(const SymbolTable& table, const std::string& func_name, const std::string& var_name) {
            const auto* sym = internal::variable::parameter_get(table, func_name, var_name);
            if (!sym) {
                if (!internal::function::exists(table, func_name))
                    return testing::AssertionFailure() << "Function with name '" << func_name << "' not found";
                else
                    return testing::AssertionFailure() << "Function '" << func_name << "' has no parameter '" << var_name << "'";
            }
            if (sym->getSymbolType() == ST_PARAMETER)
                return testing::AssertionSuccess();
            else
                return testing::AssertionFailure() << "Function '" << func_name << "' has no parameter '" << var_name << "'. Found only equivalently named variable, with type " << util::to_string(sym->getSymbolType());
        }

        inline testing::AssertionResult global_exists(const SymbolTable& table, const std::string& var_name) {
            const auto* sym = internal::variable::global_get(table, var_name);
            if (!sym)
                return testing::AssertionFailure() << "Global scope has no variable '" << var_name << "'";
            if (sym->getSymbolType() == ST_VARIABLE)
                return testing::AssertionSuccess();
            else
                return testing::AssertionFailure() << "Global scope has no variable '" << var_name << "'";
        }
    }

    namespace build {
        class FunctionTreeBuilder {
            public:
            struct TreeHandle {
                Node* source;
                SymbolTable* tableRef;
                size_t functionRef = 0;

                TreeHandle() : source(nullptr), tableRef(nullptr) {}
                TreeHandle(SymbolTable* table, size_t function) : source(nullptr), tableRef(table), functionRef(function) {}
                TreeHandle(Node* source, SymbolTable* table, size_t function) : source(source), tableRef(table), functionRef(function) {}

                /**
                 * Adds a unary node to the tree.
                 * @param nodeType NodeType for the new binary node.
                 * @param returnType ReturnType for the new binary node.
                 * @return TreeHandle for added node on success, Treehandle with nullptr source on failure.
                 */
                inline TreeHandle add_unary(NodeType nodeType, ReturnType returnType) const {
                    if (auto* unaryNode = dynamic_cast<UnaryNode*>(source)) {
                        unaryNode->setChild(new UnaryNode(nodeType, returnType));
                        return {unaryNode->getChild(), tableRef, functionRef};
                    } else if (auto* binaryNode = dynamic_cast<BinaryNode*>(source)) {
                        auto* returnNode = new UnaryNode(nodeType, returnType);
                        (!binaryNode->getLeftChild() ? binaryNode->setLeftChild(returnNode) : binaryNode->setRightChild(returnNode));
                        return {returnNode, tableRef, functionRef};
                    }
                    return {};
                }

                /**
                 * Adds a binary node to the tree.
                 * @see test::build::FunctionTreeBuilder::TreeHandle::add_unary(Nodetype, ReturnType).
                 */
                inline TreeHandle add_binary(NodeType nodeType, ReturnType returnType) const {
                    if (auto* unaryNode = dynamic_cast<UnaryNode*>(source)) {
                        unaryNode->setChild(new BinaryNode(nodeType, returnType));
                        return {unaryNode->getChild(), tableRef, functionRef};
                    } else if (auto* binaryNode = dynamic_cast<BinaryNode*>(source)) {
                        auto* returnNode = new BinaryNode(nodeType, returnType);
                        (!binaryNode->getLeftChild() ? binaryNode->setLeftChild(returnNode) : binaryNode->setRightChild(returnNode));
                        return {returnNode, tableRef, functionRef};
                    }
                    return {};
                }

                inline void add_empty() const {
                    if (auto* unaryNode = dynamic_cast<UnaryNode*>(source)) {
                        unaryNode->setChild(new Node(NODE_EMPTY, RT_VOID));
                    } else if (auto* binaryNode = dynamic_cast<BinaryNode*>(source)) {
                        auto* returnNode = new Node(NODE_EMPTY, RT_VOID);
                        (!binaryNode->getLeftChild() ? binaryNode->setLeftChild(returnNode) : binaryNode->setRightChild(returnNode));
                    }
                }

                /**
                 * Adds a symbol node to the tree. Builds a new symbol and inserts it into the symboltable.
                 * @param sym Symbol pointer to add. This function handles cleaning up.
                 * @return The added symbol id.
                 * @see test::build::FunctionTreeBuilder::TreeHandle::add_unary(NodeType, ReturnType).
                 */
                inline size_t add_symbol(NodeType nodeType, ReturnType returnType, Symbol* sym) const {
                    auto symId = tableRef->addSymbol(sym, functionRef);
                    return add_symbol(nodeType, returnType, symId);
                }

                /**
                 * Adds a symbol node to the tree. References an existing symbol in the symboltable.
                 * @return `id` of the created symbol on success, `std::numeric_limits<size_t>::max()` on failure.
                 * @note The caller is responsible to add the symbol with `id` to the symboltable.
                 * @see test::build::FunctionTreeBuilder::TreeHandle::add_symbol(NodeType, ReturnType, Symbol*)
                 */
                inline size_t add_symbol(NodeType nodeType, ReturnType returnType, const size_t id) const {
                    if (auto* unaryNode = dynamic_cast<UnaryNode*>(source)) {
                        unaryNode->setChild(new SymbolNode(nodeType, returnType, id));
                        return id;
                    } else if (auto* binaryNode = dynamic_cast<BinaryNode*>(source)) {
                        auto* returnNode = new SymbolNode(nodeType, returnType, id);
                        (!binaryNode->getLeftChild() ? binaryNode->setLeftChild(returnNode) : binaryNode->setRightChild(returnNode));
                        return id;
                    }
                    return std::numeric_limits<size_t>::max();
                }

                /**
                 * Adds a const node to the tree.
                 * @tparam T Type of the value of the const node to add.
                 * @param value Value of the const node to add.
                 * @see test::build::FunctionTreeBuilder::TreeHandle::add_unary(Nodetype, ReturnType).
                 */
                template<typename T>
                inline TreeHandle add_const(NodeType nodeType, ReturnType returnType, T value) {
                    if (auto* unaryNode = dynamic_cast<UnaryNode*>(source)) {
                        unaryNode->setChild(new ConstantNode<T>(nodeType, returnType, value));
                        return {unaryNode->getChild(), tableRef, functionRef};
                    } else if (auto* binaryNode = dynamic_cast<BinaryNode*>(source)) {
                        auto* returnNode = new ConstantNode<T>(nodeType, returnType, value);
                        (binaryNode->getLeftChild() ? binaryNode->setRightChild(returnNode) : binaryNode->setLeftChild(returnNode));
                        return {returnNode, tableRef, functionRef};
                    }
                    return {};
                }
            };

            struct Function {
                std::shared_ptr<SymbolTable> table = nullptr;
                std::unique_ptr<Node> treeRoot = nullptr;
                Function(std::shared_ptr<SymbolTable>&& table, std::unique_ptr<Node>&& treeRoot) : table(std::move(table)), treeRoot(std::move(treeRoot)) {}
            };

            FunctionTreeBuilder(const std::string &name, ReturnType functionType, int line) : FunctionTreeBuilder(std::make_shared<SymbolTable>(), name, functionType, line) {}
            FunctionTreeBuilder(const std::shared_ptr<SymbolTable> &table, const std::string &name, ReturnType functionType, int line)
                    : table(table), root(nullptr), curStmt(nullptr), funcId(table->addFunction(new Symbol(name, line, functionType, ST_FUNCTION))) {
                table->addFunction(new Symbol("writeinteger", -1, RT_VOID, ST_FUNCTION), {}, {new Symbol("i", -1, RT_INT, ST_PARAMETER)});
                table->addFunction(new Symbol("readinteger", -1, RT_INT, ST_FUNCTION));
            }

            inline Function build() {
                curStmt->setRightChild(new Node(NODE_EMPTY, RT_VOID));
                return Function(std::move(table), std::move(root));
            }

            inline TreeHandle add_statement() {
                if (!root) {
                    root = std::make_unique<BinaryNode>(NODE_STATEMENT_LIST, RT_VOID);
                    curStmt = (BinaryNode*) root.get();
                } else {
                    auto* newNode = new BinaryNode(NODE_STATEMENT_LIST, RT_VOID);
                    curStmt->setRightChild(newNode);
                    curStmt = newNode;
                }
                return {curStmt, table.get(), funcId};
            }

            inline const std::shared_ptr<SymbolTable>& getTable() const {
                return table;
            }

            /**
             * @return the ID registered for the function we are building.
             */
            inline size_t getFunctionID() const {
                return funcId;
            }

            /** Fetches a function symbol by searching for its name. Do not ever use this except for tests. */
            inline size_t getFunctionIdByName(const std::string& name) const {
                for (const auto id: table->getFunctions())
                    if (table->getSymbol(id)->getName() == name)
                        return id;
                return -1;
            }

            inline static void printTree(std::ostream& stream, const Function& function) {
                function.treeRoot->doStream(stream, 4, 4, function.table.get());
            }

            protected:
            std::shared_ptr<SymbolTable> table;
            std::unique_ptr<Node> root;
            BinaryNode* curStmt; // points to the last valid statement in the statement list.
            const size_t funcId;
        };
    }
    namespace syntax {
        inline testing::AssertionResult nodetype_correct(const Node* const node, const nodetype type) {
            if (node->getNodeType() == type)
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Provided node has incorrect nodetype '" << util::to_string(node->getNodeType()) << "' (expected '" << util::to_string(type) << "')";
        }

        inline testing::AssertionResult returntype_correct(const Node* const node, const returntype type) {
            if (node->getReturnType() == type)
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Provided node has incorrect nodetype '" << util::to_string(node->getReturnType()) << "' (expected '" << util::to_string(type) << "')";
        }

        inline size_t node_num_errors(const Node* node) {
            if (node == nullptr)
                return 0;
            if (node->getReturnType() == RT_ERROR)
                return 1;

            const auto* binary = dynamic_cast<const BinaryNode*>(node);
            if (binary != nullptr)
                return node_num_errors(binary->getLeftChild()) + node_num_errors(binary->getRightChild());
            const auto* unary = dynamic_cast<const UnaryNode*>(node);
            if (unary != nullptr)
                return node_num_errors(unary->getChild());
            return 0;
        }

        inline size_t tree_num_errors(const SyntaxTree& tree, const SymbolTable& table) {
            std::vector<size_t> funcs = table.getFunctions();
            return std::accumulate(funcs.begin(), funcs.end(), 0, [&tree](size_t sum, size_t id) {
                return sum + syntax::node_num_errors(tree.getRoot(id));
            });
        }

        template<typename T>
        inline testing::AssertionResult const_node(const Node* const node, T expected_value) {
            if (node->getNodeType() != NODE_NUM)
                return testing::AssertionFailure() << "Provided node has incorrect type '" << util::to_string(node->getNodeType()) << "' (expected '" << util::to_string(NODE_NUM) << "')";
            const auto* const constnode = dynamic_cast<const ConstantNode<T>* const>(node);
            if (!constnode) { // We had a casting failure. Is this node a ConstantNode? Did the testcase provide the (exact, no promoting) right type T?
                return testing::AssertionFailure() << "Could not cast node with type " << util::to_string(node->getReturnType()) << " to ConstantNode of type " << hack::get_name<T>();
            }
            if (constnode->getValue() == expected_value)
                return testing::AssertionSuccess();
            return testing::AssertionFailure() << "Provided constant node has incorrect value '" << constnode->getValue() << "' (expected '" << expected_value << "')";
        }


        /**
         * Checks whether 2 pairs of (syntaxtree, symboltable) are structurally equivalent.
         * @param tree Tree 1.
         * @param table  Table 1.
         * @param function (Tree 2, Table 2)
         * @param verbose If set, prints advanced debugging information that should help find error causes in code in many cases.
         * @return Whether or not structural equivalence is present.
         */
        inline testing::AssertionResult syntax_similar(const SyntaxTree& tree, const SymbolTable& table, const test::build::FunctionTreeBuilder::Function& function, bool verbose = false) {
            auto* stmt_list = ::test::function::get_root(tree, table, "main");
            if (!stmt_list)
                return testing::AssertionFailure() << "Function has no root\n";


            bool success;
            if (verbose) {
                std::stringstream ss_diff;
                success = function.treeRoot->similar_to_debug(*stmt_list, function.table.get(), &table, ss_diff, 4, 4);
                if (!success) {
                    const auto diff = ss_diff.str();
                    std::stringstream ss_expected;
                    test::build::FunctionTreeBuilder::printTree(ss_expected, function);
                    const auto expected = ss_expected.str();
                    std::stringstream ss_found;
                    tree.doStream(ss_found, 4, &table);
                    const auto found = ss_found.str();
                    return testing::AssertionFailure()
                    << "Structural difference with reference function detected. Please check the tree and differences below:\n"
                    << "1. Expected tree:\n"
                    << expected << "\n"
                    << "2. Found tree:\n"
                    << found << "\n"
                    << "3. Differences detected:\n"
                    << diff << "\n";
                }
            } else {
                success = function.treeRoot->similar_to(*stmt_list, function.table.get(), &table);
            }
            return testing::AssertionSuccess();
        }
    }

    /**
     * Verifies whether there are any errors.
     * @param num_node_errors If < 0, does not check the syntaxtree for number of errors. If >= 0, verifies whether the syntaxtree has exactly this many errors.
     */
    inline testing::AssertionResult has_errors(const SyntaxTree& tree, const SymbolTable& table, const Logger& logger, int lexcode, ssize_t num_node_errors=-1) {
        if (lexcode != 0)
            return testing::AssertionFailure() << "Lex exit code=" << lexcode << ", must be 0.";

        if (num_node_errors > 0) {
            auto num_errors = syntax::tree_num_errors(tree, table);
            if (num_errors != (size_t) num_node_errors)
                return testing::AssertionFailure() << "The final syntax tree contains " << num_errors << " error node(s), expected " << num_node_errors << "error(s).";
        }
        if (logger.n_errors() == 0)
            return testing::AssertionFailure() << "Found error nodes in tree, but no logger.error() was emitted.";
        return test::has_errors(logger);
    }

    inline testing::AssertionResult has_warnings(const SyntaxTree& tree, const SymbolTable& table, const Logger& logger, int lexcode) {
        if (lexcode != 0)
            return testing::AssertionFailure() << "Lex exit code=" << lexcode << ", must be 0.";
        if (syntax::tree_num_errors(tree, table) > 0)
            return testing::AssertionFailure() << "The final syntax tree contains error nodes, expected 0 error nodes.";
        return test::has_warnings(logger);
    }

    inline testing::AssertionResult has_success(const SyntaxTree& tree, const SymbolTable& table, const Logger& logger, int lexcode) {
        if (lexcode != 0)
            return testing::AssertionFailure() << "Lex exit code=" << lexcode << ", must be 0.";
        if (syntax::tree_num_errors(tree, table) > 0)
            return testing::AssertionFailure() << "The final syntax tree contains error nodes, expected 0 error nodes.";
        return test::has_success(logger);
    }
}

#endif
