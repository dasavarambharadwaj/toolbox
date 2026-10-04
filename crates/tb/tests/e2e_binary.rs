use std::process::Command;
use std::time::Instant;

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
fn test_e2e_tb_headless_zero_args_exits_2() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .output()
        .expect("Failed to execute tb binary in headless mode");

    assert_eq!(
        output.status.code(),
        Some(2),
        "tb with 0 args without display must exit with code 2"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Usage:") || stderr.contains("tb") || stderr.contains("Toolbox"),
        "stderr should display formatted CLI help, got: {}",
        stderr
    );
}

#[test]
fn test_e2e_tb_whitespace_display_zero_args_exits_2() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .env("DISPLAY", "   ")
        .env("WAYLAND_DISPLAY", "")
        .output()
        .expect("Failed to execute tb binary with empty/whitespace display vars");

    assert_eq!(
        output.status.code(),
        Some(2),
        "tb with empty/whitespace display must exit with code 2"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Usage:") || stderr.contains("tb") || stderr.contains("Toolbox"),
        "stderr should display formatted CLI help, got: {}",
        stderr
    );
}

#[test]
fn test_e2e_tb_gui_headless_exits_2() {
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .arg("gui")
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .output()
        .expect("Failed to execute tb gui in headless mode");

    assert_eq!(
        output.status.code(),
        Some(2),
        "tb gui without display must exit with code 2"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("No graphical display server detected"),
        "stderr should state that no display was detected, got: {}",
        stderr
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

#[test]
fn test_e2e_tb_symlink_dispatch() {
    struct TempSymlink(std::path::PathBuf);
    impl Drop for TempSymlink {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    let tb_exe = env!("CARGO_BIN_EXE_tb");
    let temp_dir = std::env::temp_dir();
    let symlink_path = temp_dir.join(format!("tb-image-test-{}", std::process::id()));

    let _ = std::fs::remove_file(&symlink_path);
    std::os::unix::fs::symlink(tb_exe, &symlink_path).expect("Failed to create symlink for test");
    let _guard = TempSymlink(symlink_path.clone());

    // Invoking symlink with --help should print help (and clap sees arguments including injected subcommand)
    let output = Command::new(&symlink_path)
        .arg("--help")
        .output()
        .expect("Failed to execute symlink");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}\n{}", stdout, stderr);
    assert!(
        combined.contains("image-test"),
        "Execution should dispatch through multi-call injecting image-test subcommand, got: {}",
        combined
    );
}

#[test]
fn test_e2e_tb_cli_latency_budget() {
    let start = Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_tb"))
        .arg("--help")
        .output()
        .expect("Failed to execute tb binary");
    let duration = start.elapsed();

    assert!(output.status.success());
    assert!(
        duration.as_millis() < 100, // Budget is <50ms in release, allow <100ms in debug test runs
        "CLI invocation took too long: {}ms",
        duration.as_millis()
    );
}

