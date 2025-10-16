# Move Mutator Tool

## Summary

The `move-mutator` tool is a tool that is used to mutate the Move language.

## Overview

The Move mutator is a tool that mutates Move source code.
It can help test the robustness of Move specifications and tests by generating different code versions (mutants).
The tool mutates only the source code. Unit tests and specification blocks are not affected by the tool.

Please refer to the design document for more details: [Move Mutator Design](doc/design.md).

## Install

To build the tools, run:
```bash
$ cargo install --git https://github.com/eigerco/move-spec-testing.git --locked move-mutator
```

That will install the tools into `~/.cargo/bin` directory (at least on MacOS and Linux).
Ensure to have this path in your `PATH` environment. This step can be done with the below command.
```bash
$ export PATH=~/.cargo/bin:$PATH
```

### Development setup

Please build the whole repository first:
```bash
cargo build --release
```

**Note:** _For any development purposes, we recommend using the `release` mode only. We don't recommend using the `debug` build since this tool is very resource-intensive._

Check if the tool is working properly by running its tests via the [`nextest`][nextest] tool:
```bash
# Using the release build for tests due to test duration
cargo nextest run -r -p move-mutator
```

## Usage

To run the tool, use the command:
```bash
./target/release/move-mutator --move-sources move-mutator/tests/move-assets/file_without_package/Sub.move
```

By default, the output shall be stored in the `mutants_output` directory unless
otherwise specified.

The mutator tool respects `RUST_LOG` variable, and it will print out as much
information as the variable allows. To see all the logs run:
```bash
RUST_LOG=trace ./target/release/move-mutator --move-sources move-mutator/tests/move-assets/file_without_package/Sub.move
```
There is a possibility of enabling logging only for specific modules. Please
refer to the [env_logger](https://docs.rs/env_logger/latest/env_logger/) documentation for more details.

There are also good tests in the `aptos-core` repository that can be used to check the tool. To run them, execute:
```
git clone https://github.com/aptos-labs/aptos-core.git;
./target/release/move-mutator --move-sources aptos-core/third_party/move/move-prover/tests/sources/functional/arithm.move;
./target/release/move-mutator --move-sources aptos-core/third_party/move/move-prover/tests/sources/functional/bitwise_operators.move;
./target/release/move-mutator --move-sources aptos-core/third_party/move/move-prover/tests/sources/functional/nonlinear_arithm.move;
./target/release/move-mutator --move-sources aptos-core/third_party/move/move-prover/tests/sources/functional/shift.move;
```
and observe `mutants_output` directory after each single command.
Please note that each call overwrites the previous output.

To generate mutants for all files within a test project (for the whole Move package) run:
```bash
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/
```

You can also examine reports made inside the output directory.

It's also possible to generate mutants for a specific module by using the `--mutate-modules` option:
```bash
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --mutate-modules Sum
```
Or use the tool to generate mutants for specific functions:
```bash
# This command will generate mutants only for functions named: 'or', 'and' and 'sum'
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --mutate-functions or,and,sum
```

The mutator tool generates:
- mutants (modified move source code)
- reports about mutants in JSON and text format.

Generating mutants for the whole package can be time-consuming. To speed up the
process, mutant verification is disabled by default. To enable it, use the
`--verify-mutants` option:
```bash
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --verify-mutants
```
Mutants verification is performed by compiling them. If the compilation fails,
the mutant is considered invalid. It's highly recommended to enable this option
as it helps to filter out invalid mutants, which would be a waste of time to
prove.

There are several test projects under `move-mutator/tests/move-assets/`
directory. They can be used to check the mutator tool as well.

To check possible options, use the `--help` option.

[nextest]: https://github.com/nextest-rs/nextest

### Operator modes

The mutator tool supports different operator modes to control which mutation operators are applied. This can significantly reduce mutation testing time by focusing on the most effective operators.

Use the `--mode` option to select a predefined operator set:
```bash
# Light mode: fastest, uses only the top 3 most effective operators
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --mode light

# Medium mode: balanced, uses the top 5 most effective operators
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --mode medium

# Heavy mode (default): uses all 7 available operators
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --mode heavy
```

The operator modes are based on [effectiveness analysis](doc/design.md#operator-effectiveness-analysis) where:
- **light**: `unary_operator_replacement`, `delete_statement`, `break_continue_replacement`
- **medium**: light + `binary_operator_replacement`, `if_else_replacement`
- **heavy**: medium + `literal_replacement`, `binary_operator_swap`

For fine-grained control, use the `--operators` option to specify exactly which operators to apply:
```bash
# Apply only specific operators
./target/release/move-mutator --package-dir move-mutator/tests/move-assets/simple/ --operators delete_statement,binary_operator_replacement,if_else_replacement
```

Available operators: `unary_operator_replacement`, `delete_statement`, `break_continue_replacement`, `binary_operator_replacement`, `if_else_replacement`, `literal_replacement`, `binary_operator_swap`.

**Note:** The `--mode` and `--operators` options are mutually exclusive.
