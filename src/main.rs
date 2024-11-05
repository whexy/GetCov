mod args;
mod coverage;
mod error;
mod execution;
mod functions;
mod report;
mod uncovered;

use coverage::check_covmap;
use error::GetCovError;
use llvm_cov_json::CoverageReport;
use report::print_uncovered;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Coverage {
    covered_branches: u64,
    total_branches: u64,
    covered_functions: u64,
    total_functions: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Output {
    coverage: Coverage,
    uncovered_functions: Vec<uncovered::PartiallyCoveredFunction>,
}

fn main() -> Result<(), GetCovError> {
    // enable logger
    env_logger::init();

    let options = args::parse_arguments()?;
    check_covmap(&options.running_options.binary)?;

    let coverage_run = execution::coverage_run(&options.running_options)?;
    let coverage_json =
        execution::generate_coverage_report_json(&options.running_options, &coverage_run)?;

    let coverage_report: CoverageReport = serde_json::from_str(&coverage_json)?;
    let program_report = &coverage_report.data[0]; // in practice, there should be only one element in the data array

    if options.extract_all_functions {
        let all_functions = functions::get_all_functions(&coverage_report);
        serde_json::to_writer_pretty(std::io::stdout(), &all_functions)?;
        return Ok(());
    }

    let uncovered_functions = uncovered::get_uncovered(&coverage_report);
    let coverage = Coverage {
        covered_branches: program_report.summary.branches.covered,
        total_branches: program_report.summary.branches.count,
        covered_functions: program_report.summary.functions.covered,
        total_functions: program_report.summary.functions.count,
    };

    match options.output_format {
        args::OutputFormat::Json => {
            let output = Output {
                coverage,
                uncovered_functions,
            };
            serde_json::to_writer_pretty(std::io::stdout(), &output)?;
        }
        args::OutputFormat::Text => print_uncovered(&uncovered_functions),
    }

    Ok(())
}
