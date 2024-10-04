// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use move_mutation_test::{
    cli::{CLIOptions, TestBuildConfig},
    run_mutation_test,
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
    /// Runs the mutation test tool.
    Run {
        /// Command line options for the mutation tester.
        #[clap(flatten)]
        cli_options: CLIOptions,
        /// The configuration options for running the tests.
        #[clap(flatten)]
        test_build_config: TestBuildConfig,
    },

    /// Display the report in a more readable format.
    DisplayReport {
        /// Report location. The default file is "report.txt" under the same directory.
        #[clap(short = 'p', long, default_value = "report.txt")]
        path_to_report: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    match &opts.command {
        Commands::Run {
            cli_options,
            test_build_config,
        } => run_mutation_test(cli_options, test_build_config),
        Commands::DisplayReport { path_to_report: _ } => {
            unimplemented!("to be in the next PR soon")
        },
    }
}
