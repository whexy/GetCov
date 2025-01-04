use crate::config::RunningOptions;
use crate::error::GetCovError;
use std::process::{Command, Stdio};
use tempfile::TempDir;
use uuid::Uuid;

pub struct CoverageRun {
    profdata_file: String,
    _temp_dir: TempDir,
}

pub fn get_coverage_report_json(options: &RunningOptions) -> Result<String, GetCovError> {
    let coverage_run = run_with_coverage(options)?;
    generate_coverage_report_json(options, &coverage_run)
}

fn run_with_coverage(options: &RunningOptions) -> Result<CoverageRun, GetCovError> {
    let temp_dir = TempDir::new()?;
    let coverage_id = Uuid::new_v4().to_string();
    let profraw_prefix = format!("getcov_{}_", coverage_id);
    let profraw_file = temp_dir
        .path()
        .join(format!("{}%m.profraw", profraw_prefix));

    for args in options.args_list.iter() {
        Command::new(&options.binary)
            .args(args)
            .env("LLVM_PROFILE_FILE", profraw_file.to_str().unwrap())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
    }

    let profraw_files: Vec<_> = std::fs::read_dir(temp_dir.path())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .map_or(false, |file_name| file_name.starts_with(&profraw_prefix))
        })
        .collect();

    if profraw_files.len() != 1 {
        return Err(GetCovError::Coverage(format!(
            "Expected 1 profraw file, found {}",
            profraw_files.len()
        )));
    }

    let profdata_file = temp_dir
        .path()
        .join(format!("getcov_{}.profdata", coverage_id));

    Command::new("llvm-profdata")
        .arg("merge")
        .arg("-sparse")
        .arg(&profraw_files[0])
        .arg("-o")
        .arg(&profdata_file)
        .spawn()?
        .wait()?;

    Ok(CoverageRun {
        profdata_file: profdata_file.to_str().unwrap().to_string(),
        _temp_dir: temp_dir,
    })
}

fn generate_coverage_report_json(
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
