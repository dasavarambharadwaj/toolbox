use slint::Global;
use std::fs;
use std::path::Path;
use tb_ui::{create_main_window, Spacing, Theme};

fn color_hex(c: slint::Color) -> String {
    format!("#{:02X}{:02X}{:02X}", c.red(), c.green(), c.blue())
}

#[test]
fn test_ui_workbench_and_theme_suite() {
    // 0. Software backend initialization
    let backend_res = tb_ui::init_software_backend();
    assert!(
        backend_res.is_ok()
            || matches!(&backend_res, Err(slint::PlatformError::SetPlatformError(_)))
            || backend_res.as_ref().err().map(|e| e.to_string().to_lowercase().contains("already")).unwrap_or(false),
        "Software rendering backend should initialize or report already set"
    );

    let window = create_main_window().expect("Failed to create main window");

    // 1. Spacing 0px border-radius compliance
    let spacing = Spacing::get(&window);
    assert_eq!(spacing.get_radius_none(), 0.0, "radius_none must be strictly 0px");
    assert_eq!(spacing.get_radius(), 0.0, "radius must be strictly 0px");

    // 2. Default theme Industrial Graphite
    let theme = Theme::get(&window);
    assert_eq!(theme.get_theme_index(), 0, "Default theme must be index 0 (Industrial Graphite)");
    let active = theme.get_active();
    assert_eq!(color_hex(active.canvas), "#141618", "Industrial Graphite canvas mismatch");
    assert_eq!(color_hex(active.surface), "#1E2124", "Industrial Graphite surface mismatch");
    assert_eq!(color_hex(active.border_prominent), "#32383E", "Industrial Graphite border mismatch");
    assert_eq!(color_hex(active.text_primary), "#E2E8F0", "Industrial Graphite text_primary mismatch");
    assert_eq!(color_hex(active.accent_primary), "#FFFFFF", "Industrial Graphite accent_primary mismatch");

    // 3. All 10 Omarchy palettes
    let expected = [
        (0, "#141618", "#FFFFFF"), // Industrial Graphite
        (1, "#000000", "#FFFFFF"), // Vantablack
        (2, "#1A1B26", "#7AA2F7"), // Tokyo Night
        (3, "#2E3440", "#88C0D0"), // Nord
        (4, "#1D2021", "#FE8019"), // Gruvbox Dark
        (5, "#080C08", "#33FF77"), // Hackerman
        (6, "#1F1F28", "#7E9CD8"), // Kanagawa
        (7, "#1E1E2E", "#CBA6F7"), // Catppuccin Mocha
        (8, "#191724", "#EBBCBA"), // Rose Pine
        (9, "#272E33", "#A7C080"), // Everforest
    ];
    for (idx, exp_canvas, exp_accent) in expected {
        theme.invoke_set_theme(idx);
        assert_eq!(theme.get_theme_index(), idx, "Theme index mismatch for palette {}", idx);
        let active = theme.get_active();
        assert_eq!(color_hex(active.canvas), exp_canvas, "Canvas mismatch for palette {}", idx);
        assert_eq!(color_hex(active.accent_primary), exp_accent, "Accent mismatch for palette {}", idx);
    }

    // 4. Clamping invalid index
    theme.invoke_set_theme(-1);
    assert_eq!(theme.get_theme_index(), 0, "Negative theme index must clamp to 0");
    assert_eq!(color_hex(theme.get_active().canvas), "#141618");

    theme.invoke_set_theme(99);
    assert_eq!(theme.get_theme_index(), 0, "Out of bounds theme index must clamp to 0");
    assert_eq!(color_hex(theme.get_active().canvas), "#141618");

    // 5. Sidebar and navigation defaults
    assert_eq!(window.get_sidebar_width(), 200.0, "Sidebar width must be exactly 200px");
    assert_eq!(window.get_active_category(), 0, "Default navigation category must be 0 (Image)");
    window.set_active_category(1);
    assert_eq!(window.get_active_category(), 1);
    window.set_active_category(2);
    assert_eq!(window.get_active_category(), 2);
    window.set_active_category(3);
    assert_eq!(window.get_active_category(), 3);
    window.set_active_category(4);
    assert_eq!(window.get_active_category(), 4);
    window.set_active_category(0);
    assert_eq!(window.get_active_category(), 0);

    // Theme font family token
    assert_eq!(
        theme.get_font_family(),
        "ui-monospace, \"JetBrains Mono\", \"Fira Code\", \"Courier New\", monospace",
        "font_family must match strict brutalist monospace stack"
    );

    // 6. Request close callback test
    let closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let closed_clone = closed.clone();
    window.on_request_close(move || {
        closed_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });
    window.invoke_request_close();
    assert!(closed.load(std::sync::atomic::Ordering::SeqCst), "request_close callback must trigger");

    // 7. Dynamic CLI dock command generation across presets and categories
    assert_eq!(
        window.get_cli_command(),
        "tb image compress --preset balanced input.png",
        "Initial CLI command should not have '$ ' prefix"
    );

    window.set_active_preset(0);
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb image compress --preset small input.png");

    window.set_active_preset(2);
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb image compress --preset best input.png");

    window.set_active_category(1);
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb pdf compress --preset best document.pdf");

    window.set_active_category(2);
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb archive pack --preset best archive.tar.gz");

    window.set_active_category(3);
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb dev format --preset best file.json");

    window.set_active_category(4);
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb config --theme 0");

    // Dynamic loaded file injection
    window.set_active_category(0);
    window.set_loaded_file("custom_photo.webp".into());
    window.invoke_update_cli_dock();
    assert_eq!(window.get_cli_command(), "tb image compress --preset best custom_photo.webp");

    // 8. Slintcn component creation
    let host = tb_ui::ComponentHost::new();
    assert!(
        host.is_ok(),
        "ComponentHost containing Button, Card, Input, Badge, Dropzone, Dialog must instantiate"
    );
}

#[test]
fn test_slintcn_components_declarations() {
    // Verify all 6 slintcn component files exist and declare their components
    let slintcn_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("ui/slintcn");
    let components = [
        ("button.slint", "component Button"),
        ("card.slint", "component Card"),
        ("input.slint", "component Input"),
        ("badge.slint", "component Badge"),
        ("dropzone.slint", "component Dropzone"),
        ("dialog.slint", "component Dialog"),
    ];
    for (filename, decl) in components {
        let file_path = slintcn_dir.join(filename);
        assert!(file_path.exists(), "Missing slintcn file: {}", filename);
        let content = fs::read_to_string(&file_path).expect("Failed to read slintcn component file");
        assert!(content.contains(decl), "{} must contain '{}'", filename, decl);
    }
}

#[test]
fn test_slint_source_0px_radius_compliance() {
    let ui_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("ui");
    assert!(ui_dir.exists(), "ui/ directory must exist in tb-ui");

    fn check_dir(dir: &Path) {
        for entry in fs::read_dir(dir).expect("Failed to read directory") {
            let entry = entry.expect("Invalid entry");
            let path = entry.path();
            if path.is_dir() {
                check_dir(&path);
            } else if path.extension().and_then(|s| s.to_str()) == Some("slint") {
                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Failed to read {}", path.display()));
                for (line_idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("//") {
                        continue;
                    }
                    if trimmed.contains("border-radius") || trimmed.contains("border_radius") {
                        assert!(
                            trimmed.contains("0px")
                                || trimmed.contains("radius_none")
                                || trimmed.contains("radius: 0px")
                                || trimmed.contains("radius;"),
                            "Violated 0px border-radius in {}:{} -> '{}'",
                            path.display(),
                            line_idx + 1,
                            trimmed
                        );
                    }
                }
            }
        }
    }

    check_dir(&ui_dir);
}
