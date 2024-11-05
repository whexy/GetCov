use llvm_cov_json::CoverageReport;

/// Extract all functions from the coverage report (using llvm coverage mapping)
/// This feature is useful for getting all functions name and location in source code.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub file_path: String,
    pub start_line: u64,
    pub start_column: u64,
    pub end_line: u64,
    pub end_column: u64, // Exclusive
}

pub fn get_all_functions(coverage_report: &CoverageReport) -> Vec<FunctionInfo> {
    coverage_report.data[0]
        .functions
        .iter()
        .map(|f| FunctionInfo {
            name: f.name.to_string(),
            file_path: f.filenames[f.regions[0].file_id as usize].to_string(),
            start_line: f.regions[0].line_start,
            start_column: f.regions[0].column_start,
            end_line: f.regions[0].line_end,
            end_column: f.regions[0].column_end,
        })
        .collect()
}
