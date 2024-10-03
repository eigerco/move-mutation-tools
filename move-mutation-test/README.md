# Move Mutation Tester tool

## Summary

The tool is used to test the quality of the test suite and the source code.

## Overview

The program logic is quite simple, the tool works using the following principles:
1. Runs tests on the original source code to ensure the original tests are valid.
2. Internally runs the _Move Mutator_ tool to generate mutants.
3. Runs the tests for all mutants to check if the mutants are killed by the original test suite.

If the mutants are not killed, it might indicate the quality of the test suite could be improved, or in some rare cases, it might indicate an error in the original source code.

**Move Mutation Tester** tool can be used on Move packages (projects) which can compile successfully and have valid tests that are passing.
Using filters, it is possible to run the tool only on certain mutants filtered by:
 - Module name (`--mutate-modules` argument)
 - Function name (`--mutate-functions` argument)

The tool cannot be used with single Move files since, to run tests, the whole Move project structure with the manifest file is required.

The tool generates a report in a JSON format. The report contains information
about the number of mutants tested and killed and also the differences between
the original and modified code.

## Setup check

Build the whole repository first:
```bash
cargo build --release
```

Check if the tool is working properly by running its tests:
```bash
cargo test -p move-mutation-test
```

## Usage

To start the mutation test, run the following command from the repo directory:
```bash
./target/release/move-mutation-test run --package-dir move-mutator/tests/move-assets/simple -o report.txt
```
The above command will store the report in a file `report.txt`.
A shortened sample output for the above command will look as follows:
```text
Total mutants tested: 229
Total mutants killed: 203

╭────────────────────────────────────────────────┬────────────────┬────────────────┬────────────╮
│ Module                                         │ Mutants tested │ Mutants killed │ Percentage │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/Operators.move::Operators::and         │ 2              │ 2              │ 100.00%    │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/Operators.move::Operators::div         │ 5              │ 5              │ 100.00%    │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/Operators.move::Operators::eq          │ 5              │ 5              │ 100.00%    │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/Operators.move::Operators::gt          │ 6              │ 6              │ 100.00%    │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/StillSimple.move::StillSimple::sample1 │ 24             │ 16             │ 66.67%     │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/StillSimple.move::StillSimple::sample2 │ 11             │ 10             │ 90.91%     │
├────────────────────────────────────────────────┼────────────────┼────────────────┼────────────┤
│ sources/Sum.move::Sum::sum                     │ 4              │ 4              │ 100.00%    │
╰────────────────────────────────────────────────┴────────────────┴────────────────┴────────────╯
```

You should see different results for different modules as it depends on the
quality of the source code and the test suites. Some modules, like `Sum`, have good
tests and all mutants are killed, while some others, like `Operators`
may not and some mutants remain alive.

It's recommended to generate a report in a JSON format and analyze it to see
which mutants are not killed, and what the differences are between the original
and modified code. This can help improve the test suite, or it may indicate
an error in the original source code.

The sample `report.txt` generated for the above command will look as follows:
```json
{
  "files": {
    "sources/Operators.move": [
      {
        "module_func": "Operators::and",
        "tested": 3,
        "killed": 2,
        "mutants_alive_diffs": [
          "--- original\n+++ modified\n@@ -108,7 +108,7 @@\n     }\n\n     fun and(x: u64, y: u64): u64 {\n-        x & y\n+        y&x\n     }\n\n     // Info: we won't kill a mutant that swaps places (false-positive)\n"
        ]
      },
      {
        "module_func": "Operators::div",
        "tested": 5,
        "killed": 5,
        "mutants_alive_diffs": []
      },
      [...]
```

The tool respects `RUST_LOG` variable, and it will print out as much information as the variable allows.
There is possibility to enable logging only for the specific modules.
Please refer to the [env_logger](https://docs.rs/env_logger/latest/env_logger/) documentation for more details.

You can try to run the tool using other examples from the `move-mutator` tests like:
```bash
./target/release/move-mutation-test run --package-dir move-mutator/tests/move-assets/breakcontinue
```

## Command-line options

To check possible options, run:
```bash
./target/release/move-mutation-test run --help
Runs the mutation test tool

Usage: move-mutation-test run [OPTIONS]

Options:
      --include-modules <INCLUDE_MODULES>              Work only over specified modules [default: all]
      --mutator-conf <MUTATOR_CONF>                    Optional configuration file for mutator tool
  -o, --output <OUTPUT>                                Save report to a JSON file
  -u, --use-generated-mutants <USE_GENERATED_MUTANTS>  Use previously generated mutants
      --package-dir <PACKAGE_DIR>                      Path to a move package (the folder with a Move.toml file).  Defaults to current directory
      --output-dir <OUTPUT_DIR>                        Path to save the compiled move package
      --named-addresses <NAMED_ADDRESSES>              Named addresses for the move binary [default: ]
      --override-std <OVERRIDE_STD>                    Override the standard library version by mainnet/testnet/devnet [possible values: mainnet, testnet, devnet]
      --skip-fetch-latest-git-deps                     Skip pulling the latest git dependencies
      --skip-attribute-checks                          Do not complain about unknown attributes in Move code
      --dev                                            Enables dev mode, which uses all dev-addresses and dev-dependencies
      --check-test-code                                Do apply extended checks for Aptos (e.g. `#[view]` attribute) also on test code. NOTE: this behavior will become the default in the future. See <https://github.com/aptos-labs/aptos-core/issues/10335> [env: APTOS_CHECK_TEST_CODE=]
      --optimize <OPTIMIZE>                            Select optimization level.  Choices are "none", "default", or "extra". Level "extra" may spend more time on expensive optimizations in the future. Level "none" does no optimizations, possibly leading to use of too many runtime resources. Level "default" is the recommended level, and the default if not
                                                       provided
      --bytecode-version <BYTECODE_VERSION>            ...or --bytecode BYTECODE_VERSION
                                                       Specify the version of the bytecode the compiler is going to emit.
                                                       Defaults to `6`, or `7` if language version 2 is selected
                                                       (through `--move-2` or `--language_version=2`), .
      --compiler-version <COMPILER_VERSION>            ...or --compiler COMPILER_VERSION
                                                       Specify the version of the compiler.
                                                       Defaults to `1`, or `2` if `--move-2` is selected.
      --language-version <LANGUAGE_VERSION>            ...or --language LANGUAGE_VERSION
                                                       Specify the language version to be supported.
                                                       Currently, defaults to `1`, unless `--move-2` is selected.
      --move-2                                         Select bytecode, language version, and compiler to support Move 2:
                                                       Same as `--bytecode_version=7 --language_version=2.0 --compiler_version=2.0`
      --dump                                           Dump storage state on failure
  -f, --filter <FILTER>                                A filter string to determine which unit tests to run
      --ignore-compile-warnings                        A boolean value to skip warnings
  -h, --help                                           Print help (see more with '--help')
```

### Examples

_In below examples, the `RUST_LOG` flag is used to provide a more informative output._

To use the tool on only the `Operators` module for the project `simple`, run:
```bash
RUST_LOG=info ./target/release/move-mutation-test run --package-dir move-mutator/tests/move-assets/simple -o report.txt --move-2 --mutate-modules Operators
```
------------------------------------------------------------------------------------------------------------
To use the tool only on functions called `sum` for the project `simple`, run:
```bash
RUST_LOG=info ./target/release/move-mutation-test run --package-dir move-mutator/tests/move-assets/simple -o report.txt --move-2 --mutate-functions sum
```
In the output for the above command, the tool will mutate both the `Operators::sum` and `Sum::sum` functions.

If the user wants to mutate only the `sum` function in the `Sum` module, the user can use this command:
```bash
RUST_LOG=info ./target/release/move-mutation-test run --package-dir move-mutator/tests/move-assets/simple -o report.txt --move-2 --mutate-functions sum --mutate-modules Sum
```

[aptos-core]: https://github.com/aptos-labs/aptos-core/
