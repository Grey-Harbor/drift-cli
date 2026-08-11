use std::io;
use std::process::ExitCode;

use clap::Parser;
use drift_cli::cli::Cli;
use drift_cli::error::AppError;
use drift_cli::output::{OutputMode, render_error};

fn main() -> ExitCode {
    let json_requested = std::env::args_os().any(|argument| argument == "--json")
        || std::env::var("DRIFT_OUTPUT").is_ok_and(|value| value.eq_ignore_ascii_case("json"));
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) if error.use_stderr() && json_requested => {
            let mut stderr = io::stderr().lock();
            let _ = render_error(
                &mut stderr,
                OutputMode::Json,
                &AppError::Usage(error.to_string()),
            );
            return ExitCode::from(2);
        }
        Err(error) => {
            let exit_code = error.exit_code() as u8;
            let _ = error.print();
            return ExitCode::from(exit_code);
        }
    };
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();

    ExitCode::from(drift_cli::run(cli, &mut stdin, &mut stdout, &mut stderr))
}
