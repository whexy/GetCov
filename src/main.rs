use analyzer::{Analyzer, ExtractFunctionsAnalyzer, UncoveredAnalyzer};
use collector::{check_covmap, get_coverage_report_json};
use error::GetCovError;
use llvm_cov_json::CoverageReport;

mod analyzer;
mod cli;
mod collector;
mod config;
mod error;

fn main() -> Result<(), GetCovError> {
    env_logger::init();

    let options = cli::parse_arguments()?;
    check_covmap(&options.running_options.binary)?;

    let coverage_json = get_coverage_report_json(&options.running_options)?;
    let coverage_report: CoverageReport = serde_json::from_str(&coverage_json)?;

    let mut analyzer: Box<dyn Analyzer> = if options.analysis_options.extract_all_functions {
        Box::new(ExtractFunctionsAnalyzer::new())
    } else {
        Box::new(UncoveredAnalyzer::new())
    };

    analyzer.analyze(&coverage_report)?;

    match options.analysis_options.output_format {
        config::OutputFormat::Json => analyzer.output_json()?,
        config::OutputFormat::Text => analyzer.output_text(),
    }

    Ok(())
}
