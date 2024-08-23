# move-mutator

## Summary

The `move-mutator` tool is a mutation testing tool for the Move language.

## Overview

The Move mutator is a tool that mutates Move source code. It can be used to
help test the robustness of Move specifications and tests by generating 
different code versions (mutants).

Please refer to the design document for more details: [Move Mutator Design](doc/design.md).

## Setup check

Please build the whole repository first:
```bash
cargo build -r
```

Check if the tool is working properly by running its tests:
```bash
cargo test -p move-mutator
```

## Usage

The `move-mutator` tool can be run using the `move-cli` tool or the `aptos`
tool. The command line options are slightly different for both tools. Last 
section of this document describes the command line options for both tools. For
the rest of this document, we will use the `aptos` tool.

```bash
./target/release/move-mutator -m move-mutator/tests/move-assets/simple/sources/Sum.move
```

By default, the output shall be stored in the `mutants_output` directory unless
otherwise specified.

The mutator tool respects `RUST_LOG` variable, and it will print out as much
information as the variable allows. To see all the logs run:
```bash
RUST_LOG=trace ./target/release/move-mutator -m move-mutator/tests/move-assets/simple/sources/Sum.move
```
There is a possibility of enabling logging only for specific modules. Please
refer to the [env_logger](https://docs.rs/env_logger/latest/env_logger/) documentation for more details.

There are also good tests in the Move Prover repository that can be used to
check the tool. To run them, execute:
```
git clone https://github.com/aptos-labs/aptos-core.git;
./target/release/move-mutator -m aptos-core/third_party/move/move-prover/tests/sources/functional/arithm.move;
./target/release/move-mutator -m aptos-core/third_party/move/move-prover/tests/sources/functional/bitwise_operators.move;
./target/release/move-mutator -m aptos-core/third_party/move/move-prover/tests/sources/functional/nonlinear_arithm.move;
./target/release/move-mutator -m aptos-core/third_party/move/move-prover/tests/sources/functional/shift.move;
```
and observe `mutants_output` directory after each single command.
Please note that each call overwrites the previous output.

To generate mutants for all files within a test project (for the whole Move package) run:
```bash
./target/release/move-mutator --package-path move-mutator/tests/move-assets/simple/
```

You can also examine reports made inside the output directory.

It's also possible to generate mutants for a specific module by using the `--mutate-modules` option:
```bash
./target/release/move-mutator --package-path move-mutator/tests/move-assets/simple/ --mutate-modules "Sum"
```

The mutator tool generates:
- mutants (modified move source code)
- reports about mutants in JSON and text format.

Generating mutants for the whole package can be time-consuming. To speed up the
process, mutant verification is disabled by default. To enable it, use the
`--verify-mutants

` option:
```bash
./target/release/move-mutator --package-path move-mutator/tests/move-assets/simple/ --verify-mutants
```
Mutants verification is done by compiling them. If the compilation fails,
the mutant is considered invalid. It's highly recommended to enable this option
as it helps to filter out invalid mutants, which would be a waste of time to
prove.

There are several test projects under `move-mutator/tests/move-assets/`
directory. They can be used to check the mutator tool as well.

## Command-line options

To check possible options run:
```bash
./target/release/move-mutator --help
Package and build system for Move code

Usage: move-mutator [OPTIONS]

Options:
  -p, --package-path <PACKAGE_PATH>
          The path where to put the output files
  -m, --move-sources <MOVE_SOURCES>
          The paths to the Move sources
      --mutate-modules <MUTATE_MODULES>
          Module names to be mutated [default: all]
  -o, --out-mutant-dir <OUT_MUTANT_DIR>
          The path where to put the output files
      --verify-mutants
          Indicates if mutants should be verified and made sure mutants can compile
  -n, --no-overwrite
          Indicates if the output files should be overwritten
      --downsampling-ratio-percentage <DOWNSAMPLING_RATIO_PERCENTAGE>
          Remove averagely given percentage of mutants. See the doc for more details
  -c, --configuration-file <CONFIGURATION_FILE>
          Optional configuration file. If provided, it will override the default configuration
  -d, --dev
          Compile in 'dev' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used if this flag is set. This flag is useful for development of packages that expose named addresses that are not set to a specific value
      --test
          Compile in 'test' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used along with any code in the 'tests' directory
      --override-std <OVERRIDE_STD>
          Whether to override the standard library with the given version [possible values: mainnet, testnet, devnet]
      --doc
          Generate documentation for packages
      --abi
          Generate ABIs for packages
      --install-dir <INSTALL_DIR>
          Installation directory for compiled artifacts. Defaults to current directory
      --force
          Force recompilation of all packages
      --arch <ARCHITECTURE>

      --fetch-deps-only
          Only fetch dependency repos to MOVE_HOME
      --skip-fetch-latest-git-deps
          Skip fetching latest git dependencies
      --bytecode-version <BYTECODE_VERSION>
          Bytecode version to compile move code
      --skip-attribute-checks
          Do not complain about an unknown attribute in Move code
      --compiler-version <COMPILER_VERSION>
          Compiler version to use
      --language-version <LANGUAGE_VERSION>
          Language version to support
      --experiments <EXPERIMENTS>
          Experiments for v2 compiler to set to true
  -h, --help
          Print help
  -V, --version
          Print version
```
