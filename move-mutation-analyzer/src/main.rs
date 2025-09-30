// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use mutator_common::report::{OperatorStats, Report};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use tabled::{builder::Builder, settings::Style};
use walkdir::WalkDir;

extern crate log;
extern crate pretty_env_logger;

#[derive(Parser)]
#[command(name = "move-mutation-analyzer")]
#[command(about = "Analyze mutation operator effectiveness across Move projects")]
#[command(version, propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze mutation operator effectiveness across aptos-move projects
    Analyze {
        /// Root directory to search for Move projects.
        #[arg(long)]
        root_dir: Option<PathBuf>,

        /// Maximum number of projects to analyze.
        #[arg(long)]
        max_projects: Option<usize>,

        /// Skip projects that don't have tests
        #[arg(long)]
        skip_no_tests: bool,
    },
    /// Display aggregated statistics from a saved analysis
    Display {
        /// Path to the saved analysis file
        #[arg(long, default_value = "operator-analysis.json")]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    match args.command {
        Commands::Analyze {
            root_dir,
            max_projects,
            skip_no_tests,
        } => {
            let root = root_dir
                .or_else(|| {
                    // Try to expand home directory
                    home::home_dir().map(|home| {
                        home.join("projects/work/aptos/aptos-core/aptos-move/framework")
                    })
                })
                .ok_or_else(|| anyhow!("Could not determine root directory"))?;

            analyze_projects(&root, max_projects, skip_no_tests)?;
        },
        Commands::Display { input } => {
            display_saved_analysis(&input)?;
        },
    }

    Ok(())
}

fn analyze_projects(root: &Path, max_projects: Option<usize>, skip_no_tests: bool) -> Result<()> {
    if !root.exists() {
        return Err(anyhow!("Root directory does not exist: {}", root.display()));
    }

    println!("Searching for Move projects in: {}", root.display());
    let move_projects = find_move_projects(root)?;

    if move_projects.is_empty() {
        return Err(anyhow!("No Move projects found in {}", root.display()));
    }

    println!("Found {} Move projects", move_projects.len());

    let mut aggregated_stats = AggregatedStats::new();
    let mut successful_projects = 0;
    let mut failed_projects = Vec::new();

    for (idx, project) in move_projects.iter().enumerate() {
        if let Some(max) = max_projects {
            if successful_projects >= max {
                println!("\nReached maximum number of projects to analyze ({max})");
                break;
            }
        }

        println!(
            "\n[{}/{}] Analyzing: {}",
            idx + 1,
            move_projects.len(),
            project.display()
        );

        // Check if project has tests
        if skip_no_tests && !has_tests(project) {
            println!("  Skipping: No test files found");
            continue;
        }

        // Run coverage
        print!("  Running coverage generation... ");
        if let Err(e) = run_coverage_for_project(project) {
            println!("FAILED");
            println!("    Error: {}", e);
            failed_projects.push((project.clone(), format!("Coverage failed: {}", e)));
            continue;
        }
        println!("OK");

        // Run mutation testing
        print!("  Running mutation testing... ");
        match run_mutation_test_for_project(project) {
            Ok(report) => {
                println!("OK");
                let stats = extract_project_stats(&report);
                println!(
                    "    Mutants: {} tested, {} killed",
                    stats.total_tested, stats.total_killed
                );
                aggregated_stats.add_report(report);
                successful_projects += 1;
            },
            Err(e) => {
                println!("FAILED");
                println!("    Error: {}", e);
                failed_projects.push((project.clone(), format!("Mutation testing failed: {}", e)));
            },
        }
    }

    println!("\n");
    println!("{}", "=".repeat(80));
    println!("Analysis Complete");
    println!("{}", "=".repeat(80));
    println!("Projects analyzed successfully: {}", successful_projects);

    if !failed_projects.is_empty() {
        println!("\nFailed projects ({}):", failed_projects.len());
        for (path, reason) in &failed_projects {
            println!("  - {}: {}", path.display(), reason);
        }
    }

    if successful_projects > 0 {
        aggregated_stats.print_comprehensive_analysis();

        // Save the analysis
        let output_file = "operator-analysis.json";
        aggregated_stats.save(Path::new(output_file))?;
        println!("\nAnalysis saved to: {}", output_file);
        println!("Use 'move-mutation-analyzer display' to view it again");
    } else {
        println!("\nNo projects were successfully analyzed");
    }

    Ok(())
}

fn find_move_projects(root: &Path) -> Result<Vec<PathBuf>> {
    let mut projects = Vec::new();

    for entry in WalkDir::new(root).max_depth(50) {
        let entry = entry?;
        if entry.file_name() == "Move.toml" {
            if let Some(parent) = entry.path().parent() {
                projects.push(parent.to_path_buf());
            }
        }
    }

    projects.sort();
    Ok(projects)
}

fn has_tests(project: &Path) -> bool {
    // Check if there are any test files in the project
    WalkDir::new(project)
        .max_depth(50)
        .into_iter()
        .filter_map(|e| e.ok())
        .any(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "move")
                .unwrap_or(false)
                && entry
                    .path()
                    .to_str()
                    .map(|p| p.contains("test"))
                    .unwrap_or(false)
        })
}

fn run_coverage_for_project(project: &Path) -> Result<()> {
    let output = Command::new("aptos")
        .args(&[
            "move",
            "test",
            "--coverage",
            "--language-version",
            "2.2",
            "--ignore-compile-warnings",
        ])
        .current_dir(project)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("{}", stderr));
    }

    Ok(())
}

fn run_mutation_test_for_project(project: &Path) -> Result<Report> {
    let output_file = project.join("mutation-report.json");

    let output = Command::new("move-mutation-test")
        .args(&[
            "run",
            "--coverage",
            "--language-version",
            "2.2",
            "--output",
            output_file.to_str().unwrap(),
            "--show-operator-stats",
            "--ignore-compile-warnings",
        ])
        .current_dir(project)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("{}", stderr));
    }

    // Load and return the report
    Report::load_from_json_file(&output_file)
}

fn extract_project_stats(report: &Report) -> ProjectStats {
    let total_tested = report.operator_stats.values().map(|s| s.tested).sum();
    let total_killed = report.operator_stats.values().map(|s| s.killed).sum();

    ProjectStats {
        total_tested,
        total_killed,
    }
}

struct ProjectStats {
    total_tested: u32,
    total_killed: u32,
}

#[derive(Default, Serialize, Deserialize)]
struct AggregatedStats {
    total_projects: usize,
    operator_totals: BTreeMap<String, OperatorStats>,
    total_mutants_tested: u32,
    total_mutants_killed: u32,
}

impl AggregatedStats {
    fn new() -> Self {
        Self::default()
    }

    fn add_report(&mut self, report: Report) {
        self.total_projects += 1;

        for (op_name, stats) in report.operator_stats {
            let total_stats = self
                .operator_totals
                .entry(op_name.clone())
                .or_insert_with(|| OperatorStats::new(op_name));

            total_stats.tested += stats.tested;
            total_stats.killed += stats.killed;
            self.total_mutants_tested += stats.tested;
            self.total_mutants_killed += stats.killed;
        }
    }

    fn save(&self, path: &Path) -> Result<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn load(path: &Path) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    fn print_comprehensive_analysis(&self) {
        println!("\n");
        println!("{}", "=".repeat(80));
        println!("COMPREHENSIVE OPERATOR EFFECTIVENESS ANALYSIS");
        println!("{}", "=".repeat(80));

        println!("\nSummary:");
        println!("{}", "-".repeat(40));
        println!("Total projects analyzed: {}", self.total_projects);
        println!("Total mutants tested: {}", self.total_mutants_tested);
        println!("Total mutants killed: {}", self.total_mutants_killed);
        let overall_effectiveness = if self.total_mutants_tested > 0 {
            (self.total_mutants_killed as f64 / self.total_mutants_tested as f64) * 100.0
        } else {
            0.0
        };
        println!("Overall effectiveness: {:.2}%", overall_effectiveness);

        let mut sorted_ops: Vec<_> = self.operator_totals.values().collect();
        sorted_ops.sort_by(|a, b| b.effectiveness().partial_cmp(&a.effectiveness()).unwrap());

        // Print detailed operator statistics table
        println!("\n");
        println!("Operator Effectiveness Rankings:");
        println!("{}", "=".repeat(80));

        let mut builder = Builder::new();
        builder.push_record([
            "Rank",
            "Operator",
            "Tested",
            "Killed",
            "Effectiveness",
            "Kill Rate",
        ]);

        for (idx, op) in sorted_ops.iter().enumerate() {
            let kill_rate = format!("{}/{}", op.killed, op.tested);
            builder.push_record([
                format!("#{}", idx + 1),
                op.name.clone(),
                op.tested.to_string(),
                op.killed.to_string(),
                format!("{:.2}%", op.effectiveness()),
                kill_rate,
            ]);
        }

        let table = builder.build().with(Style::modern_rounded()).to_string();
        println!("{}", table);

        // Generate mode recommendations
        self.print_mode_recommendations(&sorted_ops);
    }

    fn print_mode_recommendations(&self, sorted_ops: &[&OperatorStats]) {
        println!("\n");
        println!("{}", "=".repeat(80));
        println!("RECOMMENDED MODE CONFIGURATIONS");
        println!("{}", "=".repeat(80));

        if sorted_ops.is_empty() {
            println!("No operators found");
            return;
        }

        let total = sorted_ops.len();
        let light_count = ((total as f32) * 0.3).ceil().min(total as f32) as usize;
        let medium_count = ((total as f32) * 0.6).ceil().min(total as f32) as usize;

        println!("\nLight Mode (Top 30% - Fastest execution):");
        println!("{}", "-".repeat(40));
        println!("Add these operators to OperatorFilter::light_operators():\n");
        println!("vec![");
        for op in sorted_ops.iter().take(light_count) {
            println!("    \"{}\".to_string(),", op.name);
        }
        println!("]");

        println!("\nMedium Mode (Top 60% - Balanced):");
        println!("{}", "-".repeat(40));
        println!("Add these operators to OperatorFilter::medium_operators():\n");
        println!("vec![");
        for op in sorted_ops.iter().take(medium_count) {
            println!("    \"{}\".to_string(),", op.name);
        }
        println!("]");

        println!("\nHeavy Mode (All operators - Most thorough):");
        println!("{}", "-".repeat(40));
        println!("Use None in OperatorFilter::heavy_operators() to include all operators");

        // Performance estimation
        println!("\n");
        println!("{}", "=".repeat(80));
        println!("EXPECTED PERFORMANCE IMPACT");
        println!("{}", "=".repeat(80));

        let light_mutants: u32 = sorted_ops
            .iter()
            .take(light_count)
            .map(|op| op.tested)
            .sum();
        let medium_mutants: u32 = sorted_ops
            .iter()
            .take(medium_count)
            .map(|op| op.tested)
            .sum();

        let light_reduction = if self.total_mutants_tested > 0 {
            ((self.total_mutants_tested - light_mutants) as f64 / self.total_mutants_tested as f64)
                * 100.0
        } else {
            0.0
        };

        let medium_reduction = if self.total_mutants_tested > 0 {
            ((self.total_mutants_tested - medium_mutants) as f64 / self.total_mutants_tested as f64)
                * 100.0
        } else {
            0.0
        };

        println!("\nLight Mode:");
        println!("  - Uses {} out of {} operators", light_count, total);
        println!(
            "  - Tests {} mutants (vs {} in heavy mode)",
            light_mutants, self.total_mutants_tested
        );
        println!(
            "  - Estimated speed improvement: {:.1}% faster",
            light_reduction
        );

        println!("\nMedium Mode:");
        println!("  - Uses {} out of {} operators", medium_count, total);
        println!(
            "  - Tests {} mutants (vs {} in heavy mode)",
            medium_mutants, self.total_mutants_tested
        );
        println!(
            "  - Estimated speed improvement: {:.1}% faster",
            medium_reduction
        );

        println!("\n");
        println!("{}", "=".repeat(80));
        println!("COPY THE OPERATOR LISTS ABOVE INTO YOUR PHASE 2 IMPLEMENTATION");
        println!("{}", "=".repeat(80));
    }
}

fn display_saved_analysis(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Analysis file not found: {}", path.display()));
    }

    println!("Loading analysis from: {}", path.display());
    let stats = AggregatedStats::load(path)?;
    stats.print_comprehensive_analysis();

    Ok(())
}

