# C-Minus Compiler

## Introduction

A simple compiler written in Rust for the C-minus language; a very limited subset of C. Tested with `rustc` version 1.56.1.

## How to use

The compiler consists of four stages, each with their own crate. Every compiler stage crate is both a library and a binary. The binary can be conveniently run by the provided `run.sh` script at the root of every crate. To run a full compilation, use the `machine_code` crate.

The log level for each binary can be set through an environment variable `RUST_LOG`. Options are: `error`, `warning`, `info`, `debug`, `trace`.

## Overview

- `lexical` includes only the lexical parsing aspect of the compiler.
- `syntax` transforms a parse tree into an abstract syntax tree.
  - Use `-s` to obtain partial output. The (incomplete) syntax tree and symbol table will be printed even in case of an error.
- `intermediate_code` produces 3-address code for a given AST. Also performs live time analysis and can make a control flow graph.
  - Use `-a` to annotate the produced three-address code with the original variable names rather than the symbol ids, for easier reading.
  - Use `-g <filename>.png` to save the control flow graph as a PNG image. This requires Graphviz (`dot`) on your system.
- `machine_code` - produces x86 assembly for the given 3-address code.
  - Use `-o` to set the name of the output file.
  - WARNING: Machine code is WIP and will barely compile anything yet.

There are two additional crates:

- `general` includes components used across multiple other crates, such as logging.
- `tests` includes tests for all crates (WIP)
  Use the `tests.sh` script to run one or more test suites. Provide all the tests you want to run, e.g.

```bash
./tests.sh lexical syntax intermediate
```

Available test suites are `lexical`, `syntax`, `intermediate`. Tests for `machine_code` is WIP.
