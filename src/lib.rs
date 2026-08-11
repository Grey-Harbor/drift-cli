pub mod auth;
pub mod cli;
pub mod client;
pub mod commands;
pub mod config;
pub mod error;
pub mod output;

use std::io::{BufRead, Write};

use auth::resolve_credential;
use cli::{Cli, Command};
use client::DriftClient;
use commands::execute;
use config::{Environment, ProcessEnvironment, resolve};
use error::AppError;
use output::{OutputMode, render_error, render_success};

pub fn run(
    cli: Cli,
    stdin: &mut dyn BufRead,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
) -> u8 {
    run_with_environment(cli, stdin, stdout, stderr, &ProcessEnvironment)
}

pub fn run_with_environment(
    cli: Cli,
    stdin: &mut dyn BufRead,
    stdout: &mut dyn Write,
    stderr: &mut dyn Write,
    environment: &dyn Environment,
) -> u8 {
    let fallback_mode = if cli.json
        || environment
            .get("DRIFT_OUTPUT")
            .is_some_and(|value| value.eq_ignore_ascii_case("json"))
    {
        OutputMode::Json
    } else {
        OutputMode::Human
    };

    let settings = match resolve(&cli, environment) {
        Ok(settings) => settings,
        Err(error) => return render_failure(stderr, fallback_mode, error),
    };
    let mode = settings.output;
    let result = (|| {
        let credential = if matches!(cli.command, Command::Status) {
            None
        } else {
            Some(resolve_credential(
                cli.key_stdin,
                settings.credential_env.as_deref(),
                environment,
                stdin,
            )?)
        };
        let client = DriftClient::new(settings.endpoint.clone(), credential)?;
        execute(&cli.command, &client)
    })();

    match result {
        Ok(result) => match render_success(stdout, mode, &result) {
            Ok(()) => 0,
            Err(error) => render_failure(stderr, mode, error),
        },
        Err(error) => render_failure(stderr, mode, error),
    }
}

fn render_failure(stderr: &mut dyn Write, mode: OutputMode, error: AppError) -> u8 {
    let exit_code = error.exit_code();
    let _ = render_error(stderr, mode, &error);
    exit_code
}
