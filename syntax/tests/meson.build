gtest_dep = gtest.get_variable('gtest_dep')

libsyntax_test_depends = [gtest_dep, libtestutils_dep]
libsyntax_test_depends += libsyntax_depends
libsyntax_test_depends += libsyntax_dep

libsyntax_test_files = []
libsyntax_test_files += files('cpp/main.cpp', 'cpp/units/declarations.cpp', 'cpp/units/node/node.cpp', 'cpp/units/node/node_operator.cpp', 'cpp/units/node/node_coercion.cpp')

libsyntax_test_exe = executable('libsyntax_test', libsyntax_test_files, dependencies: libsyntax_test_depends, build_rpath : build_rpath, install_rpath : install_rpath, install : true)