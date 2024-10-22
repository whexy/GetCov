use crate::{args::RunningOptions, error::GetCovError};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Struct to store coverage run information
#[derive(Debug)]
pub struct CoverageRun {
    /// Path to the profdata file generated during the run
    profdata_file: String,
    /// Temporary directory used for storing profraw files
    _temp_dir: TempDir,
}

/// Run the binary with the given options.
/// Coverage run will execute the binary with the given arguments and collect coverage data.
pub fn coverage_run(options: &RunningOptions) -> Result<CoverageRun, GetCovError> {
    let temp_dir = TempDir::new()?;
    let coverage_id = uuid::Uuid::new_v4().to_string();
    let profraw_prefix = format!("getcov_{}_", coverage_id);
    let profraw_file = temp_dir
        .path()
        .join(format!("{}%m.profraw", profraw_prefix));

    for args in options.args_list.iter() {
        Command::new(&options.binary)
            .args(args)
            .env("LLVM_PROFILE_FILE", profraw_file.to_str().unwrap())
            .spawn()?
            .wait()?;
    }

    // after running, we need to match the prefix of the profraw file
    let profraw_files: Vec<_> = fs::read_dir(temp_dir.path())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .map_or(false, |file_name| file_name.starts_with(&profraw_prefix))
        })
        .collect();

    // there should be only one profraw file after filtering
    if profraw_files.len() != 1 {
        return Err(GetCovError::Coverage(format!(
            "Expected 1 profraw file, found {}",
            profraw_files.len()
        )));
    }

    let the_profraw_file = &profraw_files[0];

    // "merge" profraw file into a single profdata file
    let profdata_file = temp_dir
        .path()
        .join(format!("getcov_{}.profdata", coverage_id));
    Command::new("llvm-profdata")
        .arg("merge")
        .arg("-sparse")
        .arg(the_profraw_file)
        .arg("-o")
        .arg(&profdata_file)
        .spawn()?
        .wait()?;

    Ok(CoverageRun {
        profdata_file: profdata_file.to_str().unwrap().to_string(),
        _temp_dir: temp_dir,
    })
}

pub fn generate_coverage_report_json(
    options: &RunningOptions,
    coverage_run: &CoverageRun,
) -> Result<String, GetCovError> {
    let output = Command::new("llvm-cov")
        .arg("export")
        .arg(format!("--instr-profile={}", coverage_run.profdata_file))
        .arg(&options.binary)
        .output()?;

    Ok(String::from_utf8(output.stdout)?)
}
