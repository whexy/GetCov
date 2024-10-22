mod args;
mod coverage;
mod error;
mod execution;

use coverage::check_covmap;
use error::GetCovError;
use llvm_cov_json::CoverageReport;

fn main() -> Result<(), GetCovError> {
    // enable logger
    env_logger::init();

    let options = args::parse_arguments()?;
    check_covmap(&options.binary)?;

    let coverage_run = execution::coverage_run(&options)?;
    let coverage_report = execution::generate_coverage_report_json(&options, &coverage_run)?;

    let report: CoverageReport = serde_json::from_str(&coverage_report)?;
    let program_report = &report.data[0]; // in practice, there should be only one element in the data array

    println!(
        "Branch Coverage: {} / {}, {}%",
        program_report.summary.branches.covered,
        program_report.summary.branches.count,
        program_report.summary.branches.percent
    );

    println!(
        "Function Coverage: {} / {}, {}%",
        program_report.summary.functions.covered,
        program_report.summary.functions.count,
        program_report.summary.functions.percent
    );

    Ok(())
}
