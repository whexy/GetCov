#[derive(Debug)]
pub enum OutputFormat {
    Json,
    Text,
}

#[derive(Debug)]
pub struct Config {
    pub running_options: RunningOptions,
    pub analysis_options: AnalysisOptions,
}

#[derive(Debug)]
pub struct RunningOptions {
    pub binary: String,
    pub args_list: Vec<Vec<String>>,
}

#[derive(Debug)]
pub struct AnalysisOptions {
    pub extract_all_functions: bool,
    pub output_format: OutputFormat,
}
