// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    cli::{CLIOptions, OperatorModeArg},
    coverage::Coverage,
    operator_filter::OperatorMode,
};
use std::path::PathBuf;

/// Mutator configuration for the Move project.
#[derive(Debug)]
pub struct Configuration {
    /// Main project options. It's the same as the CLI options.
    pub project: CLIOptions,
    /// Path to the project.
    pub project_path: Option<PathBuf>,
    /// Coverage report where the optional unit test coverage data is stored.
    pub(crate) coverage: Coverage,
    /// Operator filter that determines which mutation operators are enabled.
    pub operator_mode: OperatorMode,
}

impl Configuration {
    /// Creates a new configuration using command line options.
    pub fn new(project: CLIOptions, project_path: Option<PathBuf>) -> anyhow::Result<Self> {
        // Parse and validate the operator mode from CLI options
        let operator_mode = Self::parse_operator_mode(&project)?;

        Ok(Self {
            project,
            project_path,
            // Coverage is disabled by default.
            coverage: Coverage::default(),
            operator_mode,
        })
    }

    fn parse_operator_mode(project: &CLIOptions) -> anyhow::Result<OperatorMode> {
        match (&project.mode, &project.operators) {
            // --operators specified
            (None, Some(operators)) => {
                let parsed_ops = OperatorMode::parse_operators(operators)?;
                Ok(OperatorMode::Custom(parsed_ops))
            },
            // --mode specified
            (Some(mode_arg), None) => {
                let mode = match mode_arg {
                    OperatorModeArg::Light => OperatorMode::Light,
                    OperatorModeArg::Medium => OperatorMode::Medium,
                    OperatorModeArg::Heavy => OperatorMode::Heavy,
                };
                Ok(mode)
            },
            // neither specified
            (None, None) => Ok(OperatorMode::default()),
            // both specified - this should be prevented by clap conflicts
            (Some(_), Some(_)) => {
                unreachable!("Both --mode and --operators specified")
            },
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            project: CLIOptions::default(),
            project_path: None,
            coverage: Coverage::default(),
            operator_mode: OperatorMode::default(),
        }
    }
}
