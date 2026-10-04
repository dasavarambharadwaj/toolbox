#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Gui,
    Cli,
}

pub fn determine_mode(arg_count: usize, has_display: bool) -> Mode {
    #[cfg(feature = "gui")]
    if arg_count <= 1 && has_display {
        return Mode::Gui;
    }
    let _ = (arg_count, has_display);
    Mode::Cli
}

fn main() -> anyhow::Result<()> {
    let arg_count = std::env::args_os().count();
    let has_display = std::env::var_os("WAYLAND_DISPLAY").is_some_and(|v| !v.is_empty())
        || std::env::var_os("DISPLAY").is_some_and(|v| !v.is_empty());

    match determine_mode(arg_count, has_display) {
        #[cfg(feature = "gui")]
        Mode::Gui => tb_ui::run(),
        _ => tb_cli::run(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_mode_gui() {
        #[cfg(feature = "gui")]
        assert_eq!(determine_mode(1, true), Mode::Gui);
        #[cfg(not(feature = "gui"))]
        assert_eq!(determine_mode(1, true), Mode::Cli);
    }

    #[test]
    fn test_determine_mode_cli_with_args() {
        assert_eq!(determine_mode(2, true), Mode::Cli);
    }

    #[test]
    fn test_determine_mode_headless() {
        assert_eq!(determine_mode(1, false), Mode::Cli);
    }

    #[test]
    fn test_entrypoint_wiring() {
        assert!(tb_cli::run_with_args(["tb"]).is_ok());
    }
}
