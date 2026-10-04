use clap::{CommandFactory, Parser};

/// CLI command line arguments parser for Toolbox.
#[derive(Debug, Parser, Default)]
#[command(name = "tb", about = "Toolbox multi-call utility", version)]
pub struct CliArgs {
    #[arg(long, help = "Output machine-readable JSON envelope")]
    pub json: bool,
}

/// Runs the CLI adapter with process arguments.
pub fn run() -> anyhow::Result<()> {
    run_with_args(std::env::args_os())
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
}
