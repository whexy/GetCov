use std::process::Command;

#[test]
fn test_llvm_cov_integration() {
    // Compile the C program
    let status = Command::new("make")
        .current_dir("tests/c_code")
        .status()
        .expect("Failed to compile C program");
    assert!(status.success());

    // Run 'getcov' to execute the binary and process coverage
    let output = Command::new("cargo")
        .args(&["run", "--", "--", "tests/c_code/main"])
        .output()
        .expect("Failed to run 'getcov'");
    assert!(output.status.success());

    // Check the output
    let stdout = String::from_utf8(output.stdout).expect("Failed to convert stdout to string");
    println!("{}", stdout);
}
