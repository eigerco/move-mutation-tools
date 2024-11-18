use log::info;
use move_model::metadata::LanguageVersion;
use move_package::{BuildConfig, CompilerConfig};
use move_spec_test::{cli::CLIOptions, run_spec_test};
use mutator_common::report::Report;
use std::{
    fs,
    path::{Path, PathBuf},
};

// Arbitrarily chosen after manual testing.
// Tweaking only changes the overall duration of the test a little.
const RED_ZONE: usize = 128 * 1024; // 128 KiB
const STACK_SIZE: usize = 32 * RED_ZONE; // 4 MiB

fn test_run_spec_test(path: &Path, expected_report: String) -> datatest_stable::Result<()> {
    let expected_report =
        Report::load_from_str(expected_report).expect("failed to load the report");
    let package_path = path.parent().expect("package path not found");

    let report_file = PathBuf::from("report.txt");
    let cli_opts = CLIOptions {
        output: Some(report_file.clone()),
        ..Default::default()
    };

    // By default, let's run the tests with move 2 features enabled.
    let build_cfg = BuildConfig {
        compiler_config: CompilerConfig {
            language_version: Some(LanguageVersion::latest_stable()),
            ..Default::default()
        },
        ..Default::default()
    };

    stacker::maybe_grow(RED_ZONE, STACK_SIZE, || {
        run_spec_test(&cli_opts, &build_cfg, package_path)
            .expect("running the mutation test failed");
        info!(
            "remaining stack size is {}",
            stacker::remaining_stack().expect("failed to get the remaining stack size")
        );
    });

    let generated_report = Report::load_from_json_file(&report_file).expect("report not found");

    // Let's make sure the reports are equal.
    let Report {
        files: mut expected_entries,
        ..
    } = expected_report;
    let Report {
        files: generated_report_files,
        ..
    } = generated_report;

    // Unfortunately, we cannot compare the files directly since the `package_path` is an absolute
    // path and would differ on different machines depending on the package location.
    for (file, mutant_stats) in generated_report_files {
        let (expected_file, expected_mutant_stats) = expected_entries
            .pop_first()
            .expect("reports are not the same");
        assert_eq!(file, expected_file);
        assert_eq!(mutant_stats, expected_mutant_stats);
    }
    assert!(expected_entries.is_empty());

    // Make sure we remove the file since these tests are executed serially - it makes no sense to
    // run these tests in parallel since every test spawns the maximum number of threads.
    fs::remove_file(report_file).expect("failed to remove the report file");

    Ok(())
}

const MOVE_ASSETS: &str = "../move-mutator/tests/move-assets";

datatest_stable::harness!(test_run_spec_test, MOVE_ASSETS, r".*\.spec-exp",);
