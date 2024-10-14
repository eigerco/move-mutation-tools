// Copyright © Eiger
// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

mod benchmark;
pub mod cli;
mod prover;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::{
    benchmark::{Benchmark, Benchmarks},
    prover::prove,
};
use anyhow::anyhow;
use move_package::{source_package::layout::SourcePackageLayout, BuildConfig};
use mutator_common::report::{MiniReport, MutantStatus, Report};
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

/// This function runs the specification testing, which is a combination of the mutator tool and the prover tool.
///
/// It takes the CLI options and constructs appropriate options for the
/// Move Mutator tool and Move Prover tool. Then it mutates the code storing
/// results in a temporary directory. Then it runs the prover on the mutated
/// code and remember the results, using them to generate the report at the end.
///
/// # Arguments
///
/// * `options` - A `cli::Options` representing the options for the spec test.
/// * `config` - A `BuildConfig` representing the build configuration.
/// * `package_path` - A `PathBuf` representing the path to the package.
///
/// # Errors
///
/// Errors are returned as `anyhow::Result`.
///
/// # Returns
///
/// * `anyhow::Result<()>` - The result of the spec test.
pub fn run_spec_test(
    options: &cli::CLIOptions,
    config: &BuildConfig,
    package_path: &Path,
) -> anyhow::Result<()> {
    // We need to initialize logger using try_init() as it might be already initialized in some other tool
    // (e.g. move-mutator). If we use init() instead, we will get an abort.
    let _ = pretty_env_logger::try_init();

    // Check if package is correctly structured.
    let package_path = SourcePackageLayout::try_find_root(&package_path.canonicalize()?)?;

    info!("Found package path: {package_path:?}");
    info!("Running specification tester with the following options: {options:?}");

    // Always create and use benchmarks.
    // Benchmarks call only time getting functions, so it's safe to use them in any case and
    // they are not expensive to create (won't hit the performance).
    let mut benchmarks = Benchmarks::new();
    benchmarks.spec_test.start();

    let prover_conf = cli::generate_prover_options(options)?;

    let mut error_writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);

    let result = prove(config, &package_path, &prover_conf, &mut error_writer);

    if let Err(e) = result {
        let msg = format!("Original code verification failed! Prover failed with error: {e}");
        error!("{msg}");
        return Err(anyhow!(msg));
    }

    // Setup temporary directory structure.
    let outdir = tempfile::tempdir()?.into_path();
    let outdir_original = outdir.join("base");

    fs::create_dir_all(&outdir_original)?;

    // We can skip fetching the latest deps for generating mutants and proving those mutants
    // since the original prover verification already fetched the latest dependencies.
    let mut quick_config = config.clone();
    quick_config.skip_fetch_latest_git_deps = true;

    let outdir_mutant = if let Some(mutant_path) = &options.use_generated_mutants {
        mutant_path.clone()
    } else {
        benchmarks.mutator.start();

        let outdir_mutant = run_mutator(options, &quick_config, &package_path, &outdir)?;
        benchmarks.mutator.stop();
        outdir_mutant
    };

    let report =
        move_mutator::report::Report::load_from_json_file(&outdir_mutant.join("report.json"))?;

    // Proving part.
    move_mutator::compiler::copy_dir_all(&package_path, &outdir_original)?;

    benchmarks.prover.start();
    let (proving_benchmarks, mini_reports): (Vec<Benchmark>, Vec<MiniReport>) = report
        .get_mutants()
        .par_iter()
        .map(|elem| {
            let mut benchmark = Benchmark::new();

            let mutant_file = elem.mutant_path();
            let rayon_thread_id =
                rayon::current_thread_index().expect("failed to fetch rayon thread id");
            info!(
                "job_{rayon_thread_id}: Running prover for mutant {}",
                mutant_file.display()
            );

            // Strip prefix to get the path relative to the package directory (or take that path if it's already relative).
            let original_file = elem
                .original_file_path()
                .strip_prefix(&package_path)
                .unwrap_or(elem.original_file_path());
            let job_work_dir = format!("prover_{rayon_thread_id}");
            let outdir = outdir.join(job_work_dir);

            let _ = fs::remove_dir_all(&outdir);
            move_mutator::compiler::copy_dir_all(&package_path, &outdir)
                .expect("copying directory failed");

            trace!(
                "Copying mutant file {mutant_file:?} to the package directory {:?}",
                outdir.join(original_file)
            );

            // Should never fail, since files will always exists.
            let _ = fs::copy(mutant_file, outdir.join(original_file));

            if let Err(e) =
                move_mutator::compiler::rewrite_manifest_for_mutant(&package_path, &outdir)
            {
                panic!("rewriting manifest for mutant failed: {e}");
            }

            benchmark.start();
            let mut error_writer = std::io::sink();
            let result = prove(&quick_config, &outdir, &prover_conf, &mut error_writer);
            benchmark.stop();

            let mutant_status = if let Err(e) = result {
                trace!("Mutant killed! Prover failed with error: {e}");
                MutantStatus::Killed
            } else {
                trace!("Mutant {} hasn't been killed!", mutant_file.display());
                MutantStatus::Alive
            };

            let diff = elem.get_diff().to_owned();

            let mut qname = elem.get_module_name().to_owned();
            qname.push_str("::");
            qname.push_str(elem.get_function_name());

            (
                benchmark,
                MiniReport::new(original_file.to_path_buf(), qname, mutant_status, diff),
            )
        })
        .collect::<Vec<(_, _)>>()
        .into_iter()
        .unzip();

    benchmarks.prover.stop();
    benchmarks.prover_results = proving_benchmarks;

    // Prepare a report.
    let mut test_report = Report::new(package_path.to_owned());
    for MiniReport {
        original_file,
        qname,
        mutant_status,
        diff,
    } in mini_reports
    {
        test_report.increment_mutants_tested(&original_file, &qname);
        if let MutantStatus::Alive = mutant_status {
            test_report.add_mutants_alive_diff(&original_file, &qname, &diff);
        } else {
            test_report.increment_mutants_killed(&original_file, &qname);
            test_report.add_mutants_killed_diff(&original_file, &qname, &diff);
        }
    }

    if let Some(outfile) = &options.output {
        test_report.save_to_json_file(outfile)?;
    }

    println!("\nTotal mutants tested: {}", test_report.mutants_tested());
    println!("Total mutants killed: {}\n", test_report.mutants_killed());
    test_report.print_table();

    benchmarks.spec_test.stop();
    benchmarks.display();

    Ok(())
}

/// This function runs the Move Mutator tool.
fn run_mutator(
    options: &cli::CLIOptions,
    config: &BuildConfig,
    package_path: &Path,
    outdir: &Path,
) -> anyhow::Result<PathBuf> {
    debug!("Running the move mutator tool");
    let mut mutator_conf = cli::create_mutator_options(options);

    let outdir_mutant = if let Some(path) = cli::check_mutator_output_path(&mutator_conf) {
        path
    } else {
        mutator_conf.out_mutant_dir = Some(outdir.join("mutants"));
        mutator_conf.out_mutant_dir.clone().unwrap()
    };

    fs::create_dir_all(&outdir_mutant)?;
    move_mutator::run_move_mutator(mutator_conf, config, package_path)?;

    Ok(outdir_mutant)
}
