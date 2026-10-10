slint::include_modules!();

use slint::ComponentHandle;

/// Configures or initializes the software rendering backend.
pub fn init_software_backend() -> Result<(), slint::PlatformError> {
    slint::BackendSelector::new()
        .renderer_name("software".into())
        .select()
}

/// Creates a new instance of `MainWindow`.
pub fn create_main_window() -> anyhow::Result<MainWindow> {
    MainWindow::new().map_err(|e| anyhow::anyhow!("Failed to initialize MainWindow: {e}"))
}

/// Runs the Slint GUI desktop shell with automated software rendering fallback.
pub fn run() -> anyhow::Result<()> {
    // Attempt window initialization with hardware backend, fallback to software rendering if needed
    let window = match MainWindow::new() {
        Ok(w) => w,
        Err(err) => {
            let backend_env = std::env::var("SLINT_BACKEND").unwrap_or_default();
            if !backend_env.contains("software") {
                eprintln!(
                    "Slint GPU hardware initialization failed ({err}), falling back to software renderer..."
                );
                let _ = init_software_backend();
                unsafe {
                    std::env::set_var("SLINT_BACKEND", "software");
                }
                MainWindow::new().map_err(|e| {
                    anyhow::anyhow!("Failed to initialize Slint with software renderer: {e}")
                })?
            } else {
                return Err(anyhow::anyhow!("Failed to initialize Slint UI: {err}"));
            }
        }
    };

    // Wire Escape key / window close callback
    let window_weak = window.as_weak();
    window.on_request_close(move || {
        if let Some(w) = window_weak.upgrade() {
            let _ = w.hide();
        }
        let _ = slint::quit_event_loop();
    });

    // Wire window manager close hook
    let wm_close_weak = window.as_weak();
    window.window().on_close_requested(move || {
        if let Some(w) = wm_close_weak.upgrade() {
            let _ = w.hide();
        }
        let _ = slint::quit_event_loop();
        slint::CloseRequestResponse::HideWindow
    });

    // Wire Theme switcher callback
    window.on_theme_changed(move |_idx| {
        // Slint Theme global handles palette transition dynamically
    });

    // Wire CLI dock copy callback
    window.on_copy_cli(move || {
        // CLI Dock copy action
    });

    // Wire execution callback
    window.on_execute_action(move || {
        // Workflow execution action
    });

    // Automated testing guard to avoid blocking during headless CI/integration test runs
    if let Ok(val) = std::env::var("TB_TEST_AUTO_CLOSE") {
        let v = val.trim().to_lowercase();
        if !v.is_empty() && v != "0" && v != "false" {
            return Ok(());
        }
    }

    // Run the main window event loop
    window.run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_main_window() {
        let window = create_main_window();
        assert!(window.is_ok(), "MainWindow creation should succeed");
    }
}
