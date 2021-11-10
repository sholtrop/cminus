/*
_ _ _ ____ ____ _  _ _ _  _ ____     ___  ____    _  _ ____ ___
| | | |__| |__/ |\ | | |\ | | __ .   |  \ |  |    |\ | |  |  |
|_|_| |  | |  \ | \| | | \| |__] .   |__/ |__|    | \| |__|  |

____ _  _ ____ _  _ ____ ____    ____ _ _    ____   /
|    |__| |__| |\ | | __ |___    |___ | |    |___  /
|___ |  | |  | | \| |__] |___    |    | |___ |___ .
*/

#ifndef COCO_FRAMEWORK_SYNTAX_TEST_TESTUTIL_INTERNAL
#define COCO_FRAMEWORK_SYNTAX_TEST_TESTUTIL_INTERNAL

#include <algorithm>
#include <symbol.h>

#include <symboltable.h>

namespace test {
    namespace internal {
        namespace function {
            inline size_t get_id(const SymbolTable& table, const std::string& func_name) {
                for (const auto& entry: table.getFunctions()) {
                    const auto* sym = table.getSymbol(entry);
                    if (sym->getName() == func_name)
                        return entry;
                }
                return std::numeric_limits<size_t>::max();
            }

            inline bool exists(const SymbolTable& table, const std::string& func_name) {
                return get_id(table, func_name) != std::numeric_limits<size_t>::max();
            }

            inline bool exists_id(const SymbolTable& table, size_t id) {
                const auto& functions = table.getFunctions();
                return std::find(functions.begin(), functions.end(), id) != functions.end();
            }
        }

        namespace variable {
            // Fetches local variables and parameters. Anything 'function-local'.
            inline const Symbol* local_get(const SymbolTable& table, const std::string& func_name, const std::string& var_name) {
                std::vector<Symbol*> symbols;
                if (!table.getVariables(test::internal::function::get_id(table, func_name), symbols))
                    return nullptr;
                for (const auto* var: symbols)
                    if (var->getName() == var_name)
                        return var;
                return nullptr;
            }

            inline const Symbol* parameter_get(const SymbolTable& table, const std::string& func_name, const std::string& var_name) {
                std::vector<Symbol*> symbols;
                if (!table.getParameters(test::internal::function::get_id(table, func_name), symbols))
                    return nullptr;
                for (const auto* var: symbols)
                    if (var->getName() == var_name)
                        return var;
                return nullptr;
            }

            inline const Symbol* global_get(const SymbolTable& table, const std::string& var_name) {
                auto globals = table.getGlobals();
                auto found = std::find_if(globals.begin(), globals.end(), [&var_name](const std::pair<size_t, Symbol*>& global) {
                    return var_name == global.second->getName();
                });
                if (found == globals.end())
                    return nullptr;
                return found->second;
            }
        }
    }
}

#endif