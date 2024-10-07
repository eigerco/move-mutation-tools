//! A module for displaying reports in a nice fashion.
// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::report::Report;
use crate::report::MutantStats;
use anyhow::{Context, Result};
use diffy::Line;
use prettytable::{color, format, Attr, Cell, Row, Table};
use std::{
    collections::{BTreeMap, HashSet},
    path::{Path, PathBuf},
    str::FromStr,
};

/// Filter for modules to include in the report.
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ModuleFilter {
    #[default]
    All,
    Selected(Vec<String>),
}

impl FromStr for ModuleFilter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" => Ok(ModuleFilter::All),
            _ => Ok(ModuleFilter::Selected(
                s.split(&[';', '-', ',']).map(String::from).collect(),
            )),
        }
    }
}

impl ModuleFilter {
    fn get_all_files_containing_the_modules(&self, report: &Report) -> HashSet<PathBuf> {
        match *self {
            Self::All => report.entries().keys().cloned().collect(),
            Self::Selected(ref modules) => {
                let mut files_to_print = HashSet::<PathBuf>::new();

                for module in modules {
                    for (file, mutants) in report.entries() {
                        if mutants.iter().any(|m| &m.get_module_name() == module) {
                            files_to_print.insert(file.clone());
                            break;
                        }
                    }
                }
                files_to_print
            },
        }
    }
}

/// Line stats for mutations.
#[derive(Default, Debug)]
struct MutatedLine {
    /// Number of total mutants.
    total_mutants: u32,

    /// Number of killed mutants.
    killed_mutants: u32,
}

impl From<&MutantStats> for MutatedLine {
    fn from(mutant_stats: &MutantStats) -> Self {
        Self {
            killed_mutants: mutant_stats.killed,
            total_mutants: mutant_stats.tested,
        }
    }
}

/// Line number. The first line is indexed from 1.
type LineNumber = usize;

/// File statistics about the mutated lines.
#[derive(Default, Debug)]
struct FileStats {
    /// Info about mutated lines.
    mutated_lines: BTreeMap<LineNumber, MutatedLine>,
}

impl FileStats {
    fn increment_killed_per_line(&mut self, line_number: LineNumber) {
        let mutated_line = self.mutated_lines.entry(line_number).or_default();
        mutated_line.total_mutants += 1;
        mutated_line.killed_mutants += 1;
    }

    fn increment_total_per_line(&mut self, line_number: LineNumber) {
        let mutated_line = self.mutated_lines.entry(line_number).or_default();
        mutated_line.total_mutants += 1;
    }
}

/// Displays a friendly readable report for given modules.
pub fn display_report_on_screen(path_to_report: &Path, modules: &ModuleFilter) -> Result<()> {
    let report = Report::load_from_json_file(path_to_report)?;
    let files_to_print = modules.get_all_files_containing_the_modules(&report);

    for file in files_to_print {
        let file_stats = calculate_file_stats(&file, &report)?;

        // Get the absolute file path.
        let abs_file_path = report.get_package_dir().to_path_buf().join(&file);
        let source_code = std::fs::read_to_string(&abs_file_path)?;

        print_nice_report(&file, source_code, file_stats)?;
    }

    Ok(())
}

fn print_nice_report(file: &Path, source_code: String, stats: FileStats) -> Result<()> {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let title = Cell::new_align(
        file.to_str().expect("invalid path"),
        format::Alignment::CENTER,
    )
    .with_hspan(2)
    .with_style(Attr::Bold);
    table.set_titles(Row::new(vec![title]));

    // Line numbers are indexed from 1, not from 0.
    for (line_no, line) in (1..).zip(source_code.lines()) {
        let (mut stat_cell, line_color) = if let Some(m) = stats.mutated_lines.get(&line_no) {
            let style_color = Attr::ForegroundColor(match m.killed_mutants {
                0 => color::GREEN,
                n if n == m.total_mutants => color::RED,
                _ => color::BRIGHT_YELLOW,
            });

            (
                Cell::new_align(
                    &format!("{}/{}", m.killed_mutants, m.total_mutants),
                    format::Alignment::RIGHT,
                ),
                Some(style_color),
            )
        } else {
            (Cell::new(""), None)
        };

        let mut line_cell = Cell::new(line);
        if let Some(color) = line_color {
            line_cell.style(color);
            stat_cell.style(color);
        }

        table.add_row(Row::new(vec![stat_cell, line_cell]));
    }

    table.printstd();
    Ok(())
}

fn calculate_file_stats(file: &Path, report: &Report) -> Result<FileStats> {
    let mut file_stats = FileStats::default();

    let Some(mutants) = report.entries().get(&file.to_path_buf()) else {
        return Ok(file_stats);
    };

    for mutant in mutants {
        for patch_str in &mutant.mutants_alive_diffs {
            let mutated_line_no = find_mutated_line_number(patch_str)?;
            file_stats.increment_total_per_line(mutated_line_no);
        }
        for patch_str in &mutant.mutants_killed_diff {
            let mutated_line_no = find_mutated_line_number(patch_str)?;
            file_stats.increment_killed_per_line(mutated_line_no);
        }
    }

    Ok(file_stats)
}

fn find_mutated_line_number(file_diff: &str) -> Result<usize> {
    let patch = diffy::Patch::from_str(file_diff)?;
    let hunk = patch
        .hunks()
        .first()
        .context("invalid diff in the report")?;

    let mut current_line_no = hunk.old_range().start();
    let mut lines = hunk.lines().iter();

    // Loop until Line::Deleted or Line::Insert.
    while let Some(Line::Context(_)) = lines.next() {
        current_line_no += 1;
    }

    Ok(current_line_no)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    #[test]
    fn reading_report_from_file_works() {
        let package_dir = tempfile::tempdir().unwrap().into_path();

        let mut report = Report::new(package_dir.clone());
        let path1 = package_dir.join("src_file1");
        let path2 = package_dir.join("src_file2");
        let module_name = "module";
        report.increment_mutants_tested(&path1, module_name);
        report.increment_mutants_tested(&path2, module_name);

        let report_path = package_dir.join("report.txt");
        report
            .save_to_json_file(&report_path)
            .expect("failed to save the file to a disk");

        // Files also need to exist.
        fs::File::create(path1).unwrap();
        fs::File::create(path2).unwrap();

        let modules = ModuleFilter::All;
        let ret = display_report_on_screen(&report_path, &modules);
        assert!(ret.is_ok());
    }

    #[test]
    fn report_file_not_found() {
        let path = PathBuf::from("/path/to/non/existing/file");
        let modules = ModuleFilter::All;
        let ret = display_report_on_screen(&path, &modules);
        assert!(ret.is_err());
    }
}
