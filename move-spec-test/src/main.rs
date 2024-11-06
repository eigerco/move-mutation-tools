// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use move_mutator::cli::PackagePathCheck;
use move_package::BuildConfig;
use move_spec_test::{cli::CLIOptions, run_spec_test};
use mutator_common::display_report::{
    display_coverage_on_screen, display_mutants_on_screen, DisplayReportCmd, DisplayReportOptions,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the specification test tool.
    Run {
        /// The path to the target Move package.
        #[clap(long, short, value_parser)]
        package_path: Option<PathBuf>,

        /// Command line options for specification tester.
        #[clap(flatten)]
        cli_options: CLIOptions,

        /// The build configuration options.
        #[clap(flatten)]
        build_config: BuildConfig,
    },

    /// Display the report in a more readable format.
    DisplayReport(DisplayReportOptions),
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match opts.command {
        Commands::Run {
            package_path,
            cli_options,
            build_config,
        } => {
            let package_path = cli_options.resolve(package_path)?;
            run_spec_test(&cli_options, &build_config, &package_path)
        },
        Commands::DisplayReport(display_report) => {
            let path_to_report = &display_report.path_to_report;
            let modules = &display_report.modules;

            match &display_report.cmds {
                DisplayReportCmd::Coverage => display_coverage_on_screen(path_to_report, modules),
                DisplayReportCmd::Mutants { functions, mutants } => {
                    display_mutants_on_screen(path_to_report, modules, functions, mutants)
                },
            }
        },
    }
}
