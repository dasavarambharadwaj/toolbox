fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Ensure libfontconfig can be found if the distro only installs the runtime .so.1
    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        let out_path = std::path::Path::new(&out_dir);
        let link_target = out_path.join("libfontconfig.so");
        let candidates = [
            "/usr/lib64/libfontconfig.so.1",
            "/usr/lib/x86_64-linux-gnu/libfontconfig.so.1",
            "/usr/lib/aarch64-linux-gnu/libfontconfig.so.1",
            "/usr/lib/libfontconfig.so.1",
        ];
        if !link_target.exists() {
            let _ = std::fs::remove_file(&link_target);
            for candidate in candidates {
                if std::path::Path::new(candidate).exists() {
                    let _ = std::os::unix::fs::symlink(candidate, &link_target);
                    break;
                }
            }
        }
        if link_target.exists() {
            println!("cargo:rustc-link-search=native={}", out_dir);
        }
    }

    slint_build::compile("ui/main.slint").expect("Failed to compile Slint UI markup");
}
