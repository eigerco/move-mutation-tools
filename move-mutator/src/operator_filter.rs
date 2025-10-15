// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Operator filtering module for mutation testing.
//!
//! This module provides functionality to filter mutation operators based on their effectiveness.
//!
//! It supports predefined modes (Light, Medium, Heavy) and custom operator selection,
//! where Light is the most effective (most killed mutants) and Heavy is mutating all operators.
//!
//! The way that the effectiveness was calculated is by running the tool on the biggest projects
//! in [Aptos' Move Framework](https://github.com/aptos-labs/aptos-core/tree/main/aptos-move/framework).
//!
//! These were the results:
//! Total mutants tested: 22597
//! Total mutants killed: 18535
//! Average effectiveness: 82.02%
//!
//! ╭──────┬─────────────────────────────┬────────┬────────┬───────────────┬───────────╮
//! │ Rank │ Operator                    │ Tested │ Killed │ Effectiveness │ Kill Rate │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #1   │ unary_operator_replacement  │ 219    │ 219    │ 100.00%       │ 219/219   │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #2   │ delete_statement            │ 909    │ 895    │ 98.46%        │ 895/909   │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #3   │ break_continue_replacement  │ 26     │ 23     │ 88.46%        │ 23/26     │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #4   │ binary_operator_replacement │ 7081   │ 6207   │ 87.66%        │ 6207/7081 │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #5   │ if_else_replacement         │ 5310   │ 4579   │ 86.23%        │ 4579/5310 │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #6   │ literal_replacement         │ 8781   │ 6498   │ 74.00%        │ 6498/8781 │
//! ├──────┼─────────────────────────────┼────────┼────────┼───────────────┼───────────┤
//! │ #7   │ binary_operator_swap        │ 271    │ 114    │ 42.07%        │ 114/271   │
//! ╰──────┴─────────────────────────────┴────────┴────────┴───────────────┴───────────╯

use crate::operators::binary::OPERATOR_NAME as BINARY_OPERATOR_NAME;
use crate::operators::binary_swap::OPERATOR_NAME as BINARY_SWAP_NAME;
use crate::operators::break_continue::OPERATOR_NAME as BREAK_CONTINUE_NAME;
use crate::operators::delete_stmt::OPERATOR_NAME as DELETE_STATEMENT_NAME;
use crate::operators::ifelse::OPERATOR_NAME as IF_ELSE_NAME;
use crate::operators::literal::OPERATOR_NAME as LITERAL_NAME;
use crate::operators::unary::OPERATOR_NAME as UNARY_OPERATOR_NAME;
use std::str::FromStr;

/// Enum representing all available mutation operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    UnaryOperatorReplacement,
    DeleteStatement,
    BreakContinueReplacement,
    BinaryOperatorReplacement,
    IfElseReplacement,
    LiteralReplacement,
    BinaryOperatorSwap,
}

impl Operator {
    const fn as_str(self) -> &'static str {
        match self {
            Self::UnaryOperatorReplacement => UNARY_OPERATOR_NAME,
            Self::DeleteStatement => DELETE_STATEMENT_NAME,
            Self::BreakContinueReplacement => BREAK_CONTINUE_NAME,
            Self::BinaryOperatorReplacement => BINARY_OPERATOR_NAME,
            Self::IfElseReplacement => IF_ELSE_NAME,
            Self::LiteralReplacement => LITERAL_NAME,
            Self::BinaryOperatorSwap => BINARY_SWAP_NAME,
        }
    }

    const fn all() -> [Operator; 7] {
        [
            Operator::UnaryOperatorReplacement,
            Operator::DeleteStatement,
            Operator::BreakContinueReplacement,
            Operator::BinaryOperatorReplacement,
            Operator::IfElseReplacement,
            Operator::LiteralReplacement,
            Operator::BinaryOperatorSwap,
        ]
    }
}

impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            UNARY_OPERATOR_NAME => Ok(Self::UnaryOperatorReplacement),
            DELETE_STATEMENT_NAME => Ok(Self::DeleteStatement),
            BREAK_CONTINUE_NAME => Ok(Self::BreakContinueReplacement),
            BINARY_OPERATOR_NAME => Ok(Self::BinaryOperatorReplacement),
            IF_ELSE_NAME => Ok(Self::IfElseReplacement),
            LITERAL_NAME => Ok(Self::LiteralReplacement),
            BINARY_SWAP_NAME => Ok(Self::BinaryOperatorSwap),
            _ => anyhow::bail!("Unknown operator: {}", s),
        }
    }
}

/// Mutation operator mode that determines which operators are enabled.
///
/// Based on effectiveness analysis:
/// - Light: Top 3 operators
/// - Medium: Top 5 operators
/// - Heavy: All 7 operators
#[derive(Debug, Clone, PartialEq)]
pub enum OperatorMode {
    /// Light mode: Only the most effective operators (fastest execution).
    /// Uses 3 operators, approximately 95% faster than heavy mode.
    Light,

    /// Medium mode: Balanced selection of effective operators.
    /// Uses 5 operators, approximately 40% faster than heavy mode.
    Medium,

    /// Heavy mode: All available operators (maximum coverage).
    /// Uses all 7 operators, default mode.
    Heavy,

    /// Custom mode: User-specified set of operators.
    /// The vector contains validated operators.
    Custom(Vec<Operator>),
}

impl OperatorMode {
    /// Returns the list of enabled operator names for this mode.
    pub fn get_operators(&self) -> Vec<&str> {
        self.operators_enum().iter().map(|op| op.as_str()).collect()
    }

    /// Returns the list of enabled operators as an enum (internal use).
    fn operators_enum(&self) -> Vec<Operator> {
        match self {
            OperatorMode::Light => Self::light_operators(),
            OperatorMode::Medium => Self::medium_operators(),
            OperatorMode::Heavy => Self::heavy_operators(),
            OperatorMode::Custom(ops) => ops.clone(),
        }
    }

    /// Returns operators for Light mode.
    /// Top 3 most effective operators based on effectiveness analysis.
    fn light_operators() -> Vec<Operator> {
        vec![
            Operator::UnaryOperatorReplacement,
            Operator::DeleteStatement,
            Operator::BreakContinueReplacement,
        ]
    }

    /// Returns operators for Medium mode.
    /// Top 5 most effective operators based on effectiveness analysis.
    fn medium_operators() -> Vec<Operator> {
        vec![
            Operator::UnaryOperatorReplacement,
            Operator::DeleteStatement,
            Operator::BreakContinueReplacement,
            Operator::BinaryOperatorReplacement,
            Operator::IfElseReplacement,
        ]
    }

    /// Returns operators for Heavy mode.
    /// All available operators.
    fn heavy_operators() -> Vec<Operator> {
        Operator::all().to_vec()
    }

    /// Checks if the specified operator should be applied in this mode.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator to check.
    ///
    /// # Returns
    ///
    /// `true` if the operator is enabled in this mode, `false` otherwise.
    pub fn should_apply(&self, operator: Operator) -> bool {
        self.operators_enum().contains(&operator)
    }

    /// Gets a display-friendly name for this mode.
    pub fn display_name(&self) -> String {
        match self {
            OperatorMode::Light => "LIGHT".to_string(),
            OperatorMode::Medium => "MEDIUM".to_string(),
            OperatorMode::Heavy => "HEAVY".to_string(),
            OperatorMode::Custom(_) => "CUSTOM".to_string(),
        }
    }

    /// Validates a list of operator names and returns an error if any are invalid.
    ///
    /// # Arguments
    ///
    /// * `operators` - Vector of operator names to validate.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all operators are valid, `Err` with details of invalid operators.
    pub fn validate_operators(operators: &[String]) -> anyhow::Result<()> {
        let mut invalid_ops = Vec::new();

        for op in operators {
            if Operator::from_str(op).is_err() {
                invalid_ops.push(op.clone());
            }
        }

        if !invalid_ops.is_empty() {
            anyhow::bail!(
                "Invalid operator name(s): {}. Valid operators are: {}",
                invalid_ops.join(", "),
                Self::list_all_operators()
            );
        }

        Ok(())
    }

    /// Parses a list of operator name strings into a vector of Operator enums.
    ///
    /// # Arguments
    ///
    /// * `operator_names` - Vector of operator name strings.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Operator>)` if all operators are valid, `Err` otherwise.
    pub fn parse_operators(operator_names: &[String]) -> anyhow::Result<Vec<Operator>> {
        Self::validate_operators(operator_names)?;
        Ok(operator_names
            .iter()
            .map(|s| Operator::from_str(s).expect("already validated"))
            .collect())
    }

    /// Returns a formatted string listing all available operators with their effectiveness.
    pub fn list_all_operators() -> String {
        Operator::all()
            .iter()
            .map(|op| format!("  - {}", op.as_str()))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for OperatorMode {
    fn default() -> Self {
        // Default to Heavy mode for backward compatibility
        OperatorMode::Heavy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_as_str() {
        assert_eq!(
            Operator::UnaryOperatorReplacement.as_str(),
            "unary_operator_replacement"
        );
        assert_eq!(Operator::DeleteStatement.as_str(), "delete_statement");
        assert_eq!(
            Operator::BinaryOperatorSwap.as_str(),
            "binary_operator_swap"
        );
    }

    #[test]
    fn test_operator_from_str() {
        assert_eq!(
            Operator::from_str("unary_operator_replacement").unwrap(),
            Operator::UnaryOperatorReplacement
        );
        assert_eq!(
            Operator::from_str("delete_statement").unwrap(),
            Operator::DeleteStatement
        );
        assert!(Operator::from_str("invalid").is_err());
    }

    #[test]
    fn test_operator_all() {
        let all = Operator::all();
        assert_eq!(all.len(), 7);
    }

    #[test]
    fn test_light_mode_operators() {
        let mode = OperatorMode::Light;
        let ops = mode.get_operators();
        assert_eq!(ops.len(), 3);
        assert!(ops.contains(&Operator::UnaryOperatorReplacement.as_str()));
        assert!(ops.contains(&Operator::DeleteStatement.as_str()));
        assert!(ops.contains(&Operator::BreakContinueReplacement.as_str()));
    }

    #[test]
    fn test_medium_mode_operators() {
        let mode = OperatorMode::Medium;
        let ops = mode.get_operators();
        assert_eq!(ops.len(), 5);
        assert!(ops.contains(&Operator::UnaryOperatorReplacement.as_str()));
        assert!(ops.contains(&Operator::DeleteStatement.as_str()));
        assert!(ops.contains(&Operator::BreakContinueReplacement.as_str()));
        assert!(ops.contains(&Operator::BinaryOperatorReplacement.as_str()));
        assert!(ops.contains(&Operator::IfElseReplacement.as_str()));
    }

    #[test]
    fn test_heavy_mode_operators() {
        let mode = OperatorMode::Heavy;
        let ops = mode.get_operators();
        assert_eq!(ops.len(), 7);
        // All operators should be present
        assert!(ops.contains(&Operator::UnaryOperatorReplacement.as_str()));
        assert!(ops.contains(&Operator::DeleteStatement.as_str()));
        assert!(ops.contains(&Operator::BreakContinueReplacement.as_str()));
        assert!(ops.contains(&Operator::BinaryOperatorReplacement.as_str()));
        assert!(ops.contains(&Operator::IfElseReplacement.as_str()));
        assert!(ops.contains(&Operator::LiteralReplacement.as_str()));
        assert!(ops.contains(&Operator::BinaryOperatorSwap.as_str()));
    }

    #[test]
    fn test_custom_mode() {
        let mode = OperatorMode::Custom(vec![
            Operator::DeleteStatement,
            Operator::BinaryOperatorReplacement,
        ]);
        let ops = mode.get_operators();
        assert_eq!(ops.len(), 2);
        assert!(ops.contains(&Operator::DeleteStatement.as_str()));
        assert!(ops.contains(&Operator::BinaryOperatorReplacement.as_str()));
    }

    #[test]
    fn test_should_apply() {
        let mode = OperatorMode::Light;
        assert!(mode.should_apply(Operator::UnaryOperatorReplacement));
        assert!(mode.should_apply(Operator::DeleteStatement));
        assert!(!mode.should_apply(Operator::LiteralReplacement));
        assert!(!mode.should_apply(Operator::BinaryOperatorSwap));
    }

    #[test]
    fn test_validate_operators_valid() {
        let operators = vec![
            Operator::DeleteStatement.as_str().to_string(),
            Operator::BinaryOperatorReplacement.as_str().to_string(),
        ];
        assert!(OperatorMode::validate_operators(&operators).is_ok());
    }

    #[test]
    fn test_validate_operators_invalid() {
        let operators = vec![
            "invalid_operator".to_string(),
            "another_invalid".to_string(),
        ];
        let result = OperatorMode::validate_operators(&operators);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("invalid_operator"));
        assert!(err_msg.contains("another_invalid"));
    }

    #[test]
    fn test_validate_operators_mixed() {
        let operators = vec![
            Operator::DeleteStatement.as_str().to_string(),
            "invalid_operator".to_string(),
        ];
        let result = OperatorMode::validate_operators(&operators);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        // Error message should mention the invalid operator
        assert!(err_msg.contains("invalid_operator"));
        // Error message will also list all valid operators (including delete_statement)
        assert!(err_msg.contains("Invalid operator name"));
    }

    #[test]
    fn test_parse_operators() {
        let operators = vec![
            Operator::DeleteStatement.as_str().to_string(),
            Operator::BinaryOperatorReplacement.as_str().to_string(),
        ];
        let result = OperatorMode::parse_operators(&operators);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], Operator::DeleteStatement);
        assert_eq!(parsed[1], Operator::BinaryOperatorReplacement);
    }

    #[test]
    fn test_default_mode() {
        let mode = OperatorMode::default();
        assert_eq!(mode, OperatorMode::Heavy);
    }

    #[test]
    fn test_display_name() {
        assert_eq!(OperatorMode::Light.display_name(), "LIGHT");
        assert_eq!(OperatorMode::Medium.display_name(), "MEDIUM");
        assert_eq!(OperatorMode::Heavy.display_name(), "HEAVY");
        assert_eq!(OperatorMode::Custom(vec![]).display_name(), "CUSTOM");
    }
}
