use std::ffi::{OsStr, OsString};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum DispatchTarget {
    Gui,
    Cli(Vec<OsString>),
    HeadlessHelp,
    Error(String, i32),
}

fn has_active_display(
    wayland_display: Option<&OsStr>,
    x11_display: Option<&OsStr>,
) -> bool {
    let check = |opt: Option<&OsStr>| {
        opt.and_then(|s| s.to_str()).map(|s| !s.trim().is_empty()).unwrap_or(false)
    };
    check(wayland_display) || check(x11_display)
}

pub fn determine_dispatch_target<I, T>(
    exec_path: Option<&Path>,
    args: I,
    wayland_display: Option<&OsStr>,
    x11_display: Option<&OsStr>,
) -> DispatchTarget
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    let mut collected_args: Vec<OsString> = args.into_iter().map(Into::into).collect();

    // 1. Detect executable stem from invocation path
    let exec_stem = exec_path
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str());

    let mut is_symlink_gui = false;

    if let Some(subcmd) = exec_stem.and_then(|stem| stem.strip_prefix("tb-")) {
        if subcmd == "gui" {
            is_symlink_gui = true;
        } else if !subcmd.is_empty() {
            collected_args.insert(0, OsString::from(subcmd));
        }
    }

    let display_active = has_active_display(wayland_display, x11_display);

    // If symlinked directly as tb-gui
    if is_symlink_gui {
        #[cfg(feature = "gui")]
        if display_active {
            return DispatchTarget::Gui;
        }
        #[cfg(not(feature = "gui"))]
        if display_active {
            return DispatchTarget::Error(
                "GUI support is disabled in this headless build.".to_string(),
                2,
            );
        }
        return DispatchTarget::Error(
            "No graphical display server detected.".to_string(),
            2,
        );
    }

    // 2. Inspect remaining arguments
    if collected_args.is_empty() {
        #[cfg(feature = "gui")]
        if display_active {
            return DispatchTarget::Gui;
        }
        return DispatchTarget::HeadlessHelp;
    }

    // Check if first argument is "gui"
    if collected_args.first().and_then(|s| s.to_str()) == Some("gui") {
        #[cfg(feature = "gui")]
        if display_active {
            return DispatchTarget::Gui;
        }
        #[cfg(not(feature = "gui"))]
        if display_active {
            return DispatchTarget::Error(
                "GUI support is disabled in this headless build.".to_string(),
                2,
            );
        }
        return DispatchTarget::Error(
            "No graphical display server detected.".to_string(),
            2,
        );
    }

    DispatchTarget::Cli(collected_args)
}

fn main() -> anyhow::Result<()> {
    let mut raw_args = std::env::args_os();
    let exec_path = raw_args.next();
    let exec_path_ref = exec_path.as_deref().map(Path::new);

    let wayland = std::env::var_os("WAYLAND_DISPLAY");
    let x11 = std::env::var_os("DISPLAY");

    let target = determine_dispatch_target(
        exec_path_ref,
        raw_args,
        wayland.as_deref(),
        x11.as_deref(),
    );

    match target {
        #[cfg(feature = "gui")]
        DispatchTarget::Gui => tb_ui::run(),
        #[cfg(not(feature = "gui"))]
        DispatchTarget::Gui => {
            eprintln!("Error: GUI support is disabled in this headless build.");
            std::process::exit(2);
        }
        DispatchTarget::Cli(cli_args) => {
            let mut full_args = vec![exec_path.unwrap_or_else(|| OsString::from("tb"))];
            full_args.extend(cli_args);
            let exit_code = tb_cli::run_cli(full_args);
            std::process::exit(exit_code);
        }
        DispatchTarget::HeadlessHelp => {
            tb_cli::print_help_to_stderr()?;
            std::process::exit(2);
        }
        DispatchTarget::Error(msg, code) => {
            let (err_code, suggestion) = if msg.contains("disabled") || msg.contains("headless") {
                (
                    "FEATURE_UNAVAILABLE",
                    "Use CLI subcommands or install a build with GUI support enabled.",
                )
            } else if msg.contains("display") || msg.contains("Display") {
                (
                    "DISPLAY_NOT_FOUND",
                    "Run in a graphical desktop session (X11 or Wayland) or use CLI commands.",
                )
            } else {
                (
                    "INVALID_ARGUMENT",
                    "Check 'tb --help' for valid options and usage.",
                )
            };

            let is_json = std::env::args_os().any(|a| a.to_str() == Some("--json"));
            if is_json {
                let envelope: tb_cli::JsonEnvelope<()> = tb_cli::JsonEnvelope::error(
                    err_code,
                    &msg,
                    Some(suggestion.to_string()),
                );
                println!("{}", serde_json::to_string(&envelope).unwrap());
            } else {
                use std::io::IsTerminal;
                let use_color = tb_cli::should_use_color(std::io::stderr().is_terminal());
                eprintln!(
                    "{}",
                    tb_cli::format_error(&msg, Some(suggestion), use_color)
                );
            }
            std::process::exit(code);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{OsStr, OsString};
    use std::path::Path;

    #[test]
    fn test_zero_args_with_wayland_display() {
        let target = determine_dispatch_target(
            Some(Path::new("tb")),
            Vec::<OsString>::new(),
            Some(OsStr::new("wayland-0")),
            None,
        );
        #[cfg(feature = "gui")]
        assert_eq!(target, DispatchTarget::Gui);
        #[cfg(not(feature = "gui"))]
        assert_eq!(target, DispatchTarget::HeadlessHelp);
    }

    #[test]
    fn test_zero_args_with_x11_display() {
        let target = determine_dispatch_target(
            Some(Path::new("tb")),
            Vec::<OsString>::new(),
            None,
            Some(OsStr::new(":0")),
        );
        #[cfg(feature = "gui")]
        assert_eq!(target, DispatchTarget::Gui);
        #[cfg(not(feature = "gui"))]
        assert_eq!(target, DispatchTarget::HeadlessHelp);
    }

    #[test]
    fn test_zero_args_empty_or_whitespace_display() {
        let target1 = determine_dispatch_target(
            Some(Path::new("tb")),
            Vec::<OsString>::new(),
            Some(OsStr::new("")),
            Some(OsStr::new("   ")),
        );
        assert_eq!(target1, DispatchTarget::HeadlessHelp);

        let target2 = determine_dispatch_target(
            Some(Path::new("tb")),
            Vec::<OsString>::new(),
            None,
            None,
        );
        assert_eq!(target2, DispatchTarget::HeadlessHelp);
    }

    #[test]
    fn test_explicit_tb_gui_with_display() {
        let target = determine_dispatch_target(
            Some(Path::new("tb")),
            vec![OsString::from("gui")],
            None,
            Some(OsStr::new(":0")),
        );
        #[cfg(feature = "gui")]
        assert_eq!(target, DispatchTarget::Gui);
        #[cfg(not(feature = "gui"))]
        assert!(matches!(target, DispatchTarget::Error(..)));
    }

    #[test]
    fn test_explicit_tb_gui_headless() {
        let target = determine_dispatch_target(
            Some(Path::new("tb")),
            vec![OsString::from("gui")],
            None,
            None,
        );
        assert_eq!(
            target,
            DispatchTarget::Error("No graphical display server detected.".to_string(), 2)
        );
    }

    #[test]
    fn test_symlink_tb_prefix_injection() {
        // e.g. tb-image with args ["compress", "test.png"] -> Cli(["image", "compress", "test.png"])
        let target = determine_dispatch_target(
            Some(Path::new("/usr/local/bin/tb-image")),
            vec![OsString::from("compress"), OsString::from("test.png")],
            None,
            None,
        );
        assert_eq!(
            target,
            DispatchTarget::Cli(vec![
                OsString::from("image"),
                OsString::from("compress"),
                OsString::from("test.png")
            ])
        );
    }

    #[test]
    fn test_symlink_tb_pdf_merge() {
        let target = determine_dispatch_target(
            Some(Path::new("/usr/bin/tb-pdf")),
            vec![OsString::from("merge"), OsString::from("a.pdf")],
            None,
            None,
        );
        assert_eq!(
            target,
            DispatchTarget::Cli(vec![
                OsString::from("pdf"),
                OsString::from("merge"),
                OsString::from("a.pdf")
            ])
        );
    }

    #[test]
    fn test_symlink_tb_gui() {
        // tb-gui with display
        let target_display = determine_dispatch_target(
            Some(Path::new("tb-gui")),
            Vec::<OsString>::new(),
            Some(OsStr::new("wayland-0")),
            None,
        );
        #[cfg(feature = "gui")]
        assert_eq!(target_display, DispatchTarget::Gui);
        #[cfg(not(feature = "gui"))]
        assert_eq!(
            target_display,
            DispatchTarget::Error(
                "GUI support is disabled in this headless build.".to_string(),
                2
            )
        );

        // tb-gui without display
        let target_headless = determine_dispatch_target(
            Some(Path::new("tb-gui")),
            Vec::<OsString>::new(),
            None,
            None,
        );
        assert_eq!(
            target_headless,
            DispatchTarget::Error("No graphical display server detected.".to_string(), 2)
        );
    }

    #[test]
    fn test_symlink_with_zero_args_active_display() {
        let target = determine_dispatch_target(
            Some(Path::new("tb-image")),
            Vec::<OsString>::new(),
            Some(OsStr::new(":0")),
            None,
        );
        assert_eq!(target, DispatchTarget::Cli(vec![OsString::from("image")]));
    }

    #[test]
    fn test_standard_cli_args() {
        let target = determine_dispatch_target(
            Some(Path::new("tb")),
            vec![OsString::from("--help")],
            Some(OsStr::new(":0")),
            None,
        );
        assert_eq!(
            target,
            DispatchTarget::Cli(vec![OsString::from("--help")])
        );

        let target_ver = determine_dispatch_target(
            Some(Path::new("tb")),
            vec![OsString::from("--version")],
            Some(OsStr::new(":0")),
            None,
        );
        assert_eq!(
            target_ver,
            DispatchTarget::Cli(vec![OsString::from("--version")])
        );

        let target_dev = determine_dispatch_target(
            Some(Path::new("tb")),
            vec![OsString::from("dev"), OsString::from("json")],
            None,
            None,
        );
        assert_eq!(
            target_dev,
            DispatchTarget::Cli(vec![OsString::from("dev"), OsString::from("json")])
        );
    }
}
