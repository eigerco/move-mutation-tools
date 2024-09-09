// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use clap::Parser;
use move_mutation_test::{
    cli::{CLIOptions, TestBuildConfig},
    run_mutation_test,
};

#[derive(Parser)]
pub struct Opts {
    /// Command line options for the mutation tester.
    #[clap(flatten)]
    pub cli_options: CLIOptions,
    /// The configuration options for running the tests.
    #[clap(flatten)]
    pub test_build_config: TestBuildConfig,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    run_mutation_test(&opts.cli_options, &opts.test_build_config)
}
