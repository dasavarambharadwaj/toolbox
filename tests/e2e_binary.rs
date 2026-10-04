use std::process::Command;

#[test]
fn test_e2e_tb_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .arg("--help")
        .output()
        .expect("Failed to execute tb binary");

    assert!(output.status.success(), "tb --help should exit with code 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Toolbox multi-call utility"),
        "stdout should display toolbox description, got: {}",
        stdout
    );
}

#[test]
fn test_e2e_tb_invalid_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .arg("--invalid-flag-that-does-not-exist")
        .output()
        .expect("Failed to execute tb binary");

    assert!(
        !output.status.success(),
        "tb with invalid flag should fail with non-zero exit code"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unexpected argument") || stderr.contains("error"),
        "stderr should display clap argument error, got: {}",
        stderr
    );
}

#[test]
fn test_e2e_tb_headless_mode() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .output()
        .expect("Failed to execute tb binary in headless mode");

    assert!(
        output.status.success(),
        "tb in headless mode should exit cleanly via CLI without display server error"
    );
}

#[test]
fn test_e2e_tb_json_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .arg("--json")
        .output()
        .expect("Failed to execute tb binary with --json");

    assert!(
        output.status.success(),
        "tb --json should exit cleanly with code 0"
    );
}
