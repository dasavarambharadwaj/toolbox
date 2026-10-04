use std::process::Command;

#[test]
fn test_e2e_tb_core_isolation() {
    let output = Command::new("cargo")
        .args(["tree", "-p", "tb-core"])
        .output()
        .expect("Failed to execute cargo tree -p tb-core");

    assert!(
        output.status.success(),
        "cargo tree -p tb-core should exit with 0"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("slint"),
        "tb-core must never have transitive slint dependency"
    );
    assert!(
        !stdout.contains("clap"),
        "tb-core must never have transitive clap dependency"
    );
}

#[test]
fn test_e2e_tb_headless_features() {
    let output = Command::new("cargo")
        .args(["tree", "-p", "tb", "--no-default-features"])
        .output()
        .expect("Failed to execute cargo tree -p tb --no-default-features");

    assert!(
        output.status.success(),
        "cargo tree -p tb --no-default-features should exit with 0"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("slint"),
        "tb without default features must not depend on slint"
    );
    assert!(
        !stdout.contains("tb-ui"),
        "tb without default features must not depend on tb-ui"
    );
}
