// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos::common::types::MovePackageOptions;
use aptos_framework::extended_checks;
use clap::Parser;
use move_model::metadata::{CompilerVersion, LanguageVersion};
use move_mutator::cli::{FunctionFilter, ModuleFilter, OperatorModeArg};
use move_package::CompilerConfig;
use std::path::PathBuf;

/// Command line options for mutation test tool.
#[derive(Parser, Default, Debug, Clone)]
pub struct CLIOptions {
    /// Work only over specified modules.
    #[clap(
        long,
        value_parser,
        default_value = "all",
        conflicts_with = "use_generated_mutants"
    )]
    pub mutate_modules: ModuleFilter,

    /// Work only over specified functions (these are not qualified functions).
    #[clap(
        long,
        value_parser,
        default_value = "all",
        conflicts_with = "use_generated_mutants"
    )]
    pub mutate_functions: FunctionFilter,

    /// Save report to a JSON file.
    #[clap(long, value_parser)]
    pub output: Option<PathBuf>,

    /// Use previously generated mutants.
    #[clap(long, value_parser)]
    pub use_generated_mutants: Option<PathBuf>,

    /// Remove averagely given percentage of mutants. See the doc for more details.
    #[clap(long, conflicts_with = "use_generated_mutants")]
    pub downsampling_ratio_percentage: Option<usize>,

    /// Mutation operator mode to balance speed and test gap detection.
    ///
    /// - light: binary_operator_swap, break_continue_replacement, delete_statement
    /// - medium: light + literal_replacement
    /// - medium-only: literal_replacement (only what's added in medium)
    /// - heavy (default): all 7 operators
    /// - heavy-only: unary_operator_replacement, binary_operator_replacement, if_else_replacement (only what's added in heavy)
    #[clap(
        long,
        value_enum,
        conflicts_with = "operators",
        conflicts_with = "use_generated_mutants"
    )]
    pub mode: Option<OperatorModeArg>,

    /// Custom operator selection to run mutations on (comma-separated).
    ///
    /// Available operators: unary_operator_replacement, delete_statement, break_continue_replacement, binary_operator_replacement, if_else_replacement, literal_replacement, binary_operator_swap
    #[clap(
        long,
        value_parser,
        value_delimiter = ',',
        conflicts_with = "mode",
        conflicts_with = "use_generated_mutants"
    )]
    pub operators: Option<Vec<String>>,
}

/// This function creates a mutator CLI options from the given mutation-test options.
#[must_use]
pub fn create_mutator_options(
    options: &CLIOptions,
    apply_coverage: bool,
) -> move_mutator::cli::CLIOptions {
    move_mutator::cli::CLIOptions {
        mutate_functions: options.mutate_functions.clone(),
        mutate_modules: options.mutate_modules.clone(),
        downsampling_ratio_percentage: options.downsampling_ratio_percentage,
        apply_coverage,
        // To run tests, compilation must succeed
        mode: options.mode,
        operators: options.operators.clone(),
        ..Default::default()
    }
}

/// The configuration options for running the tests.
// Info: this set struct is based on TestPackage in `aptos-core/crates/aptos/src/move_tool/mod.rs`.
#[derive(Parser, Debug, Clone)]
pub struct TestBuildConfig {
    /// A filter string to determine which unit tests to run
    #[clap(long, short)]
    pub filter: Option<String>,

    /// A boolean value to skip warnings.
    #[clap(long)]
    pub ignore_compile_warnings: bool,

    #[clap(flatten)]
    pub move_options: MovePackageOptions,

    /// Collect coverage information for later use with the various `aptos move coverage` subcommands
    #[clap(long = "coverage")]
    pub compute_coverage: bool,

    /// Dump storage state on failure.
    #[clap(long = "dump")]
    pub dump_state: bool,

    /// The maximum gas limit for each test.
    ///
    /// Used mainly for disabling mutants with infinite loops.
    /// The default value is large enough for all normal tests in most projects.
    #[clap(long, default_value_t = 1_000_000)]
    pub gas_limit: u64,

    /// Whether to stop testing upon the first failure.
    #[clap(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub fail_fast: bool,

    /// Whether to test the mutant module first. This takes precedence over the `filter` option.
    #[clap(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub mutant_module_first: bool,
}

impl TestBuildConfig {
    /// Create a [`CompilerConfig`] from the [`TestBuildConfig`].
    pub fn compiler_config(&self) -> CompilerConfig {
        let known_attributes = extended_checks::get_all_attribute_names().clone();
        CompilerConfig {
            known_attributes: known_attributes.clone(),
            skip_attribute_checks: self.move_options.skip_attribute_checks,
            bytecode_version: get_bytecode_version(
                self.move_options.bytecode_version,
                self.move_options.language_version,
            ),
            compiler_version: self
                .move_options
                .compiler_version
                .or_else(|| Some(CompilerVersion::latest_stable())),
            language_version: self
                .move_options
                .language_version
                .or_else(|| Some(LanguageVersion::latest_stable())),
            experiments: self.move_options.compute_experiments(),
            print_errors: false,
        }
    }
}

fn get_bytecode_version(
    bytecode_version_in: Option<u32>,
    language_version: Option<LanguageVersion>,
) -> Option<u32> {
    bytecode_version_in.or_else(|| language_version.map(|lv| lv.infer_bytecode_version(None)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_options_starts_empty() {
        let options = CLIOptions::default();
        assert_eq!(ModuleFilter::All, options.mutate_modules);
        assert_eq!(FunctionFilter::All, options.mutate_functions);
        assert!(options.output.is_none());
    }

    #[test]
    fn create_mutator_options_copies_fields() {
        let options = crate::cli::CLIOptions {
            mutate_modules: ModuleFilter::Selected(vec!["mod1".to_string(), "mod2".to_string()]),
            mutate_functions: FunctionFilter::Selected(vec![
                "func1".to_string(),
                "func2".to_string(),
            ]),
            ..Default::default()
        };

        let mutator_options = create_mutator_options(&options, false);

        assert_eq!(mutator_options.mutate_modules, options.mutate_modules);
    }
}
