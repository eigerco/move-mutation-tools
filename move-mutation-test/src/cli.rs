// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::too_long_first_doc_paragraph)]

use aptos::common::types::{MovePackageDir, OptimizationLevel};
use aptos_framework::extended_checks;
use clap::Parser;
use move_compiler_v2::Experiment;
use move_model::metadata::LanguageVersion;
use move_mutator::cli::ModuleFilter;
use move_package::CompilerConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Command line options for mutation test tool.
#[derive(Parser, Default, Debug, Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CLIOptions {
    /// Work only over specified modules.
    #[clap(long, value_parser, default_value = "all")]
    pub include_modules: ModuleFilter,

    /// Optional configuration file for mutator tool.
    #[clap(long, value_parser)]
    pub mutator_conf: Option<PathBuf>,

    /// Save report to a JSON file.
    #[clap(short, long, value_parser)]
    pub output: Option<PathBuf>,

    /// Use previously generated mutants.
    #[clap(long, short, value_parser)]
    pub use_generated_mutants: Option<PathBuf>,
}

/// This function creates a mutator CLI options from the given mutation-test options.
#[must_use]
pub fn create_mutator_options(options: &CLIOptions) -> move_mutator::cli::CLIOptions {
    move_mutator::cli::CLIOptions {
        // TODO: add an option to mutate functions (in general, not tied to any module)
        mutate_modules: options.include_modules.clone(),
        configuration_file: options.mutator_conf.clone(),
        // To run tests, compilation must succeed
        verify_mutants: true,
        ..Default::default()
    }
}

/// This function checks if the mutator output path is provided in the configuration file.
/// We don't need to check if the mutator output path is provided in the options as they were created
/// from the mutation-test options which does not allow setting it.
#[must_use]
pub fn check_mutator_output_path(options: &move_mutator::cli::CLIOptions) -> Option<PathBuf> {
    options
        .configuration_file
        .as_ref()
        .and_then(|conf| move_mutator::configuration::Configuration::from_file(conf).ok())
        .and_then(|c| c.project.out_mutant_dir)
}

/// Runs Move unit tests for a package
///
/// This will run Move unit tests against a package with debug mode
/// turned on.  Note, that move code warnings currently block tests from running.
#[derive(Parser)]
pub struct TestPackage {}

// Info: this set struct is based on TestPackage in `aptos-core/crates/aptos/src/move_tool/mod.rs`.
/// The configuration options for running the tests.
#[derive(Parser, Debug)]
pub struct TestBuildConfig {
    /// Options for compiling a move package dir.
    // We might move some options out and have our own option struct here - not all options are
    // needed for mutation testing.
    #[clap(flatten)]
    pub move_pkg: MovePackageDir,

    /// Dump storage state on failure.
    #[clap(long = "dump")]
    pub dump_state: bool,

    /// A filter string to determine which unit tests to run.
    #[clap(long, short)]
    pub filter: Option<String>,

    /// A boolean value to skip warnings.
    #[clap(long)]
    pub ignore_compile_warnings: bool,
    // TODO: Unused in aptos-core:
    ///// The maximum number of instructions that can be executed by a test
    /////
    ///// If set, the number of instructions executed by one test will be bounded
    //#[clap(long = "instructions", default_value_t = 100000)]
    //pub instruction_execution_bound: u64,

    // TODO: There is no sense in enabling coverage - we'll have another option in the future
    // 'use-coverage-data' or something like that - that is going to be passed to the mutator
    // tool
    ///// Collect coverage information for later use with the various `aptos move coverage` subcommands
    //#[clap(long = "coverage")]
    //pub compute_coverage: bool,
}

impl TestBuildConfig {
    pub fn compiler_config(&self) -> CompilerConfig {
        let known_attributes = extended_checks::get_all_attribute_names().clone();
        CompilerConfig {
            known_attributes,
            skip_attribute_checks: self.move_pkg.skip_attribute_checks,
            bytecode_version: get_bytecode_version(
                self.move_pkg.bytecode_version,
                self.move_pkg.language_version,
            ),
            compiler_version: self.move_pkg.compiler_version,
            language_version: self.move_pkg.language_version,
            experiments: experiments_from_opt_level(&self.move_pkg.optimize),
        }
    }
}

/// Get bytecode version.
fn get_bytecode_version(
    bytecode_version_in: Option<u32>,
    language_version: Option<LanguageVersion>,
) -> Option<u32> {
    bytecode_version_in.or_else(|| language_version.map(|lv| lv.infer_bytecode_version(None)))
}

/// Get a list of stringified [`Experiment`] from the optimization level.
fn experiments_from_opt_level(optlevel: &Option<OptimizationLevel>) -> Vec<String> {
    match optlevel {
        None | Some(OptimizationLevel::Default) => {
            vec![format!("{}=on", Experiment::OPTIMIZE.to_string())]
        },
        Some(OptimizationLevel::None) => vec![format!("{}=off", Experiment::OPTIMIZE.to_string())],
        Some(OptimizationLevel::Extra) => vec![
            format!("{}=on", Experiment::OPTIMIZE_EXTRA.to_string()),
            format!("{}=on", Experiment::OPTIMIZE.to_string()),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn cli_options_starts_empty() {
        let options = CLIOptions::default();
        assert_eq!(ModuleFilter::All, options.include_modules);
        assert!(options.mutator_conf.is_none());
        assert!(options.output.is_none());
    }

    #[test]
    fn create_mutator_options_copies_fields() {
        let options = crate::cli::CLIOptions {
            include_modules: ModuleFilter::Selected(vec!["test1".to_string(), "test2".to_string()]),
            mutator_conf: Some(PathBuf::from("path/to/mutator/conf")),
            ..Default::default()
        };
        let mutator_options = create_mutator_options(&options);

        assert_eq!(mutator_options.mutate_modules, options.include_modules);
        assert_eq!(mutator_options.configuration_file, options.mutator_conf);
    }

    #[test]
    fn check_mutator_output_path_returns_none_when_no_conf() {
        let options = move_mutator::cli::CLIOptions::default();
        assert!(check_mutator_output_path(&options).is_none());
    }

    #[test]
    fn check_mutator_output_path_returns_path_when_conf_exists() {
        let json_content = r#"
            {
                "project": {
                    "out_mutant_dir": "path/to/out_mutant_dir"
                },
                "project_path": "/path/to/project",
                "individual": []
            }
        "#;

        fs::write("test_mutator_conf.json", json_content).unwrap();

        let options = move_mutator::cli::CLIOptions {
            configuration_file: Some(PathBuf::from("test_mutator_conf.json")),
            ..Default::default()
        };

        let path = check_mutator_output_path(&options);
        fs::remove_file("test_mutator_conf.json").unwrap();

        assert!(path.is_some());
        assert_eq!(path.unwrap(), PathBuf::from("path/to/out_mutant_dir"));
    }
}
