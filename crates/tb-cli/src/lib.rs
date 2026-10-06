use clap::{CommandFactory, Parser};
use serde::{Deserialize, Serialize};

pub use tb_core;

/// CLI command line arguments parser for Toolbox.
#[derive(Debug, Parser, Default)]
#[command(name = "tb", about = "Toolbox multi-call utility", version)]
pub struct CliArgs {
    #[arg(long, help = "Output machine-readable JSON envelope")]
    pub json: bool,
}

/// Standardized JSON response envelope across Toolbox CLI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum JsonEnvelope<T> {
    Success { data: T },
    Error { error: JsonError },
}

pub type Envelope<T> = JsonEnvelope<T>;
pub type ErrorPayload = JsonError;

/// Structured error payload adhering to the Toolbox error taxonomy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct JsonError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_action: Option<String>,
}

impl<T> JsonEnvelope<T> {
    pub fn success(data: T) -> Self {
        JsonEnvelope::Success { data }
    }

    pub fn error(
        code: impl Into<String>,
        message: impl Into<String>,
        suggested_action: Option<String>,
    ) -> Self {
        JsonEnvelope::Error {
            error: JsonError {
                code: code.into(),
                message: message.into(),
                suggested_action,
            },
        }
    }
}

impl From<&tb_core::TbError> for JsonError {
    fn from(err: &tb_core::TbError) -> Self {
        Self {
            code: err.error_code().to_string(),
            message: err.to_string(),
            suggested_action: err.suggested_action(),
        }
    }
}

impl From<tb_core::TbError> for JsonError {
    fn from(err: tb_core::TbError) -> Self {
        JsonError::from(&err)
    }
}

/// Checks whether color output should be enabled based on environment and terminal detection.
pub fn should_use_color(is_terminal: bool) -> bool {
    should_color_with_env(
        std::env::var("NO_COLOR").ok().as_deref(),
        std::env::var("TERM").ok().as_deref(),
        is_terminal,
    )
}

/// Pure helper to evaluate color support given explicit environment variables and TTY status.
pub fn should_color_with_env(
    no_color: Option<&str>,
    term: Option<&str>,
    is_terminal: bool,
) -> bool {
    if matches!(no_color, Some(nc) if !nc.is_empty()) {
        return false;
    }
    if matches!(term, Some("dumb")) {
        return false;
    }
    is_terminal
}

/// Strips all ANSI color/styling escape sequences from a string.
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            chars.next(); // consume '['
            while let Some(&c) = chars.peek() {
                chars.next();
                if (0x40..=0x7e).contains(&(c as u32)) {
                    // CSI final byte reached
                    break;
                }
                if !(0x20..=0x3f).contains(&(c as u32)) {
                    // Non-CSI byte encountered; terminate escape sequence parsing
                    result.push(c);
                    break;
                }
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Formats a human-friendly error string with optional ANSI coloring.
pub fn format_error(message: &str, suggested_action: Option<&str>, color_enabled: bool) -> String {
    if color_enabled {
        let mut out = format!("\x1b[1;31m[FAIL]\x1b[0m {}", message);
        if let Some(action) = suggested_action {
            out.push_str(&format!("\n       \x1b[1;33mSuggested:\x1b[0m {}", action));
        }
        out
    } else {
        let clean_msg = strip_ansi(message);
        let mut out = format!("[FAIL] {}", clean_msg);
        if let Some(action) = suggested_action {
            let clean_action = strip_ansi(action);
            out.push_str(&format!("\n       Suggested: {}", clean_action));
        }
        out
    }
}

/// Formats a human-friendly success string with optional ANSI coloring.
pub fn format_success(message: &str, color_enabled: bool) -> String {
    if color_enabled {
        format!("\x1b[1;32m[OK]\x1b[0m {}", message)
    } else {
        format!("[OK] {}", strip_ansi(message))
    }
}

/// Runs the CLI adapter with process arguments.
pub fn run() -> anyhow::Result<()> {
    run_with_args(std::env::args_os())
}

/// Runs the CLI adapter with specified arguments and returns the appropriate process exit code.
pub fn run_cli<I, T>(args: I) -> i32
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    use std::io::IsTerminal;

    let raw_args: Vec<std::ffi::OsString> = args.into_iter().map(Into::into).collect();
    let is_json = raw_args.iter().any(|arg| arg.to_str() == Some("--json"));

    match CliArgs::try_parse_from(&raw_args) {
        Ok(cli_args) => {
            if cli_args.json {
                let envelope = JsonEnvelope::success(serde_json::json!({}));
                println!("{}", serde_json::to_string(&envelope).unwrap());
            }
            tb_core::EXIT_SUCCESS
        }
        Err(e) if !e.use_stderr() => {
            let _ = e.print();
            tb_core::EXIT_SUCCESS
        }
        Err(e) => {
            let full_err = e.to_string();
            let err_body = full_err
                .split("\n\n")
                .next()
                .unwrap_or(&full_err)
                .trim_start_matches("error: ")
                .trim();

            if is_json {
                let envelope: JsonEnvelope<()> = JsonEnvelope::error(
                    "INVALID_ARGUMENT",
                    err_body,
                    Some("Run 'tb --help' for valid options and usage.".to_string()),
                );
                println!("{}", serde_json::to_string(&envelope).unwrap());
            } else {
                let use_color = should_use_color(std::io::stderr().is_terminal());
                eprintln!(
                    "{}",
                    format_error(
                        err_body,
                        Some("Check 'tb --help' for valid options and usage."),
                        use_color
                    )
                );
            }
            tb_core::EXIT_INVALID_ARGUMENT
        }
    }
}

/// Formats and outputs the result of a domain operation according to whether JSON mode is enabled.
pub fn handle_result<T: Serialize>(result: Result<T, tb_core::TbError>, is_json: bool) -> i32 {
    use std::io::IsTerminal;
    match result {
        Ok(data) => {
            if is_json {
                let envelope = JsonEnvelope::success(data);
                println!("{}", serde_json::to_string(&envelope).unwrap());
            }
            tb_core::EXIT_SUCCESS
        }
        Err(err) => {
            let exit_code = err.exit_code();
            if is_json {
                let json_err: JsonError = (&err).into();
                let envelope: JsonEnvelope<()> = JsonEnvelope::Error { error: json_err };
                println!("{}", serde_json::to_string(&envelope).unwrap());
            } else {
                let use_color = should_use_color(std::io::stderr().is_terminal());
                eprintln!(
                    "{}",
                    format_error(&err.to_string(), err.suggested_action().as_deref(), use_color)
                );
            }
            exit_code
        }
    }
}

/// Runs the CLI adapter with specified arguments.
pub fn run_with_args<I, T>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    match CliArgs::try_parse_from(args) {
        Ok(_args) => Ok(()),
        Err(e) if e.use_stderr() => Err(e.into()),
        Err(e) => {
            e.print()?;
            Ok(())
        }
    }
}

/// Prints formatted CLI help to stderr.
pub fn print_help_to_stderr() -> anyhow::Result<()> {
    use std::io::Write;
    let mut cmd = CliArgs::command();
    let help_bytes = cmd.render_help();
    let mut stderr = std::io::stderr().lock();
    writeln!(stderr, "{}", help_bytes)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_run() {
        assert!(run_with_args(["tb"]).is_ok());
    }

    #[test]
    fn test_cli_run_invalid_arg() {
        assert!(run_with_args(["tb", "--unknown-flag"]).is_err());
    }

    #[test]
    fn test_cli_args_parsing() {
        use clap::Parser;
        let args = CliArgs::try_parse_from(["tb", "--json"]).expect("args parse failed");
        assert!(args.json);
    }

    #[test]
    fn test_cli_version() {
        assert!(run_with_args(["tb", "--version"]).is_ok());
    }

    #[test]
    fn test_json_success_envelope_serialization() {
        let envelope = JsonEnvelope::success(serde_json::json!({"item": "test_data"}));
        let serialized = serde_json::to_string(&envelope).expect("serialization failed");
        assert_eq!(
            serialized,
            r#"{"status":"success","data":{"item":"test_data"}}"#
        );
    }

    #[test]
    fn test_json_error_envelope_serialization() {
        let envelope: JsonEnvelope<()> = JsonEnvelope::error(
            "INVALID_ARGUMENT",
            "Unknown flag passed",
            Some("Use 'tb --help' for assistance".to_string()),
        );
        let serialized = serde_json::to_string(&envelope).expect("serialization failed");
        assert_eq!(
            serialized,
            r#"{"status":"error","error":{"code":"INVALID_ARGUMENT","message":"Unknown flag passed","suggested_action":"Use 'tb --help' for assistance"}}"#
        );

        let envelope_no_action: JsonEnvelope<()> = JsonEnvelope::error(
            "OPERATION_FAILED",
            "Something failed",
            None,
        );
        let serialized_no_action = serde_json::to_string(&envelope_no_action).expect("serialization failed");
        assert_eq!(
            serialized_no_action,
            r#"{"status":"error","error":{"code":"OPERATION_FAILED","message":"Something failed"}}"#
        );
    }

    #[test]
    fn test_ansi_stripping_and_formatting() {
        let text_with_ansi = "\x1b[1;31m[FAIL]\x1b[0m Operation failed";
        assert_eq!(strip_ansi(text_with_ansi), "[FAIL] Operation failed");

        // Non-CSI byte encountered terminates escape scanning without hanging or consuming normal text
        let non_csi_terminated = "\x1b[12\nOperation failed";
        assert_eq!(strip_ansi(non_csi_terminated), "\nOperation failed");

        let colored = format_error("Test failure", Some("Try running tb --help"), true);
        assert!(colored.contains("\x1b["));
        assert!(colored.contains("[FAIL]"));

        let uncolored = format_error("Test failure", Some("Try running tb --help"), false);
        assert!(!uncolored.contains("\x1b["));
        assert!(uncolored.contains("[FAIL]"));
        assert!(uncolored.contains("Suggested: Try running tb --help"));

        // format_error strips ANSI codes from message and action when color is disabled
        let uncolored_with_ansi_input = format_error(
            "\x1b[1;31mRaw ANSI error\x1b[0m",
            Some("\x1b[1;33mRaw ANSI action\x1b[0m"),
            false,
        );
        assert_eq!(
            uncolored_with_ansi_input,
            "[FAIL] Raw ANSI error\n       Suggested: Raw ANSI action"
        );
    }

    #[test]
    fn test_should_color_with_env() {
        // NO_COLOR non-empty disables color regardless of terminal
        assert!(!should_color_with_env(Some("1"), None, true));
        assert!(!should_color_with_env(Some("true"), None, true));

        // TERM=dumb disables color
        assert!(!should_color_with_env(None, Some("dumb"), true));

        // When terminal is not interactive / not a TTY, color is false
        assert!(!should_color_with_env(None, Some("xterm-256color"), false));

        // Interactive terminal with normal TERM and no NO_COLOR enables color
        assert!(should_color_with_env(None, Some("xterm-256color"), true));
    }

    #[test]
    fn test_should_use_color_env_inspection() {
        unsafe {
            std::env::set_var("NO_COLOR", "1");
            std::env::remove_var("TERM");
        }
        assert!(!should_use_color(true));

        unsafe {
            std::env::remove_var("NO_COLOR");
            std::env::set_var("TERM", "dumb");
        }
        assert!(!should_use_color(true));

        unsafe {
            std::env::remove_var("NO_COLOR");
            std::env::remove_var("TERM");
        }
    }

    #[test]
    fn test_tb_error_to_json_error() {
        let tb_err = tb_core::TbError::EngineMissing("ffmpeg".to_string());
        let json_err: JsonError = (&tb_err).into();
        assert_eq!(json_err.code, "ENGINE_MISSING");
        assert!(json_err.message.contains("ffmpeg"));
        assert!(json_err.suggested_action.is_some());
    }
}
