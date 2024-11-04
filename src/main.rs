mod args;
mod coverage;
mod error;
mod execution;
mod uncovered;
mod report;

use coverage::check_covmap;
use error::GetCovError;
use llvm_cov_json::CoverageReport;
use report::print_uncovered;
use std::fs;

fn main() -> Result<(), GetCovError> {
    // enable logger
    env_logger::init();

    let options = args::parse_arguments()?;
    check_covmap(&options.binary)?;

    let coverage_run = execution::coverage_run(&options)?;
    let coverage_json = execution::generate_coverage_report_json(&options, &coverage_run)?;

    // save coverage report to file "report.json"
    fs::write("report.json", &coverage_json)?;

    let coverage_report: CoverageReport = serde_json::from_str(&coverage_json)?;
    let program_report = &coverage_report.data[0]; // in practice, there should be only one element in the data array

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

    let uncovered_functions = uncovered::get_uncovered(&coverage_report);    
    print_uncovered(&uncovered_functions);

    // also, print the json representation of the uncovered_functions
    println!("{}", serde_json::to_string_pretty(&uncovered_functions)?);

    Ok(())
}
