use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use reqwest::Url;
use serde::Deserialize;

use crate::cli::Cli;
use crate::error::AppError;
use crate::output::OutputMode;

pub trait Environment {
    fn get(&self, name: &str) -> Option<String>;
}

pub struct ProcessEnvironment;

impl Environment for ProcessEnvironment {
    fn get(&self, name: &str) -> Option<String> {
        std::env::var(name).ok()
    }
}

#[derive(Debug)]
pub struct ResolvedSettings {
    pub endpoint: Url,
    pub output: OutputMode,
    pub credential_env: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfigFile {
    default_profile: Option<String>,
    endpoint: Option<String>,
    output: Option<OutputMode>,
    #[serde(default)]
    profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Profile {
    endpoint: String,
    credential_env: Option<String>,
}

pub fn resolve(cli: &Cli, environment: &dyn Environment) -> Result<ResolvedSettings, AppError> {
    let (config, _) = load_config(cli.config.as_deref(), environment)?;
    let environment_profile = nonempty_env(environment, "DRIFT_PROFILE").transpose()?;
    let selected_profile = cli
        .profile
        .clone()
        .or(environment_profile)
        .or_else(|| config.default_profile.clone());

    let profile = match selected_profile.as_deref() {
        Some(name) => Some(config.profiles.get(name).ok_or_else(|| {
            AppError::Config(format!("configuration profile '{name}' does not exist"))
        })?),
        None => None,
    };

    let endpoint = cli
        .endpoint
        .clone()
        .map(Ok)
        .or_else(|| nonempty_env(environment, "DRIFT_ENDPOINT"))
        .or_else(|| profile.map(|profile| Ok(profile.endpoint.clone())))
        .or_else(|| config.endpoint.clone().map(Ok))
        .unwrap_or_else(|| Ok("http://localhost:3000".to_owned()))?;

    let output = if cli.json {
        OutputMode::Json
    } else if let Some(value) = nonempty_env(environment, "DRIFT_OUTPUT") {
        parse_output(&value?)?
    } else {
        config.output.unwrap_or_default()
    };

    let credential_env = profile
        .and_then(|profile| profile.credential_env.clone())
        .map(validate_environment_name)
        .transpose()?;

    Ok(ResolvedSettings {
        endpoint: parse_endpoint(&endpoint)?,
        output,
        credential_env,
    })
}

fn load_config(
    cli_path: Option<&Path>,
    environment: &dyn Environment,
) -> Result<(ConfigFile, Option<PathBuf>), AppError> {
    let environment_path = nonempty_env(environment, "DRIFT_CONFIG").transpose()?;
    let explicit_path = cli_path
        .map(Path::to_path_buf)
        .or_else(|| environment_path.map(PathBuf::from));
    let path = explicit_path.clone().or_else(default_config_path);

    let Some(path) = path else {
        return Ok((ConfigFile::default(), None));
    };

    if !path.exists() {
        if explicit_path.is_some() {
            return Err(AppError::Config(format!(
                "configuration file '{}' does not exist",
                path.display()
            )));
        }
        return Ok((ConfigFile::default(), Some(path)));
    }

    let contents = fs::read_to_string(&path).map_err(|error| {
        AppError::Config(format!(
            "could not read configuration file '{}': {error}",
            path.display()
        ))
    })?;
    let parsed = toml::from_str(&contents).map_err(|error| {
        AppError::Config(format!(
            "could not parse configuration file '{}': {error}",
            path.display()
        ))
    })?;

    Ok((parsed, Some(path)))
}

fn default_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "GreyHarbor", "drift-cli")
        .map(|project| project.config_dir().join("config.toml"))
}

fn nonempty_env(environment: &dyn Environment, name: &str) -> Option<Result<String, AppError>> {
    environment.get(name).map(|value| {
        if value.is_empty() {
            Err(AppError::Config(format!(
                "environment variable {name} is empty"
            )))
        } else {
            Ok(value)
        }
    })
}

fn parse_output(value: &str) -> Result<OutputMode, AppError> {
    match value.to_ascii_lowercase().as_str() {
        "human" => Ok(OutputMode::Human),
        "json" => Ok(OutputMode::Json),
        _ => Err(AppError::Config(format!(
            "invalid output mode '{value}'; expected 'human' or 'json'"
        ))),
    }
}

fn parse_endpoint(value: &str) -> Result<Url, AppError> {
    let mut endpoint = Url::parse(value)
        .map_err(|error| AppError::Config(format!("invalid Drift endpoint '{value}': {error}")))?;
    if !matches!(endpoint.scheme(), "http" | "https") {
        return Err(AppError::Config(
            "Drift endpoint must use http or https".to_owned(),
        ));
    }
    if !endpoint.username().is_empty() || endpoint.password().is_some() {
        return Err(AppError::Config(
            "Drift endpoint must not contain credentials".to_owned(),
        ));
    }
    if endpoint.query().is_some() || endpoint.fragment().is_some() {
        return Err(AppError::Config(
            "Drift endpoint must not contain a query or fragment".to_owned(),
        ));
    }
    if !endpoint.path().ends_with('/') {
        endpoint.set_path(&format!("{}/", endpoint.path()));
    }
    Ok(endpoint)
}

fn validate_environment_name(value: String) -> Result<String, AppError> {
    if value.is_empty()
        || !value
            .chars()
            .all(|character| character == '_' || character.is_ascii_alphanumeric())
    {
        return Err(AppError::Config(format!(
            "invalid credential environment variable name '{value}'"
        )));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Write;

    use clap::Parser;
    use tempfile::NamedTempFile;

    use super::*;

    #[derive(Default)]
    struct TestEnvironment(HashMap<String, String>);

    impl Environment for TestEnvironment {
        fn get(&self, name: &str) -> Option<String> {
            self.0.get(name).cloned()
        }
    }

    fn cli(arguments: &[&str]) -> Cli {
        Cli::try_parse_from(std::iter::once("drift").chain(arguments.iter().copied())).unwrap()
    }

    #[test]
    fn resolves_cli_over_environment_over_profile() {
        let mut file = NamedTempFile::new().unwrap();
        write!(
            file,
            r#"
default_profile = "local"
output = "human"

[profiles.local]
endpoint = "https://profile.example"
credential_env = "PROFILE_KEY"
"#
        )
        .unwrap();
        let environment = TestEnvironment(HashMap::from([
            (
                "DRIFT_ENDPOINT".to_owned(),
                "https://environment.example".to_owned(),
            ),
            ("DRIFT_OUTPUT".to_owned(), "json".to_owned()),
        ]));
        let cli = cli(&[
            "--config",
            file.path().to_str().unwrap(),
            "--endpoint",
            "https://cli.example",
            "status",
        ]);

        let resolved = resolve(&cli, &environment).unwrap();
        assert_eq!(resolved.endpoint.as_str(), "https://cli.example/");
        assert_eq!(resolved.output, OutputMode::Json);
        assert_eq!(resolved.credential_env.as_deref(), Some("PROFILE_KEY"));
    }

    #[test]
    fn rejects_secret_in_endpoint() {
        let cli = cli(&["--endpoint", "https://secret@example.com", "status"]);
        let error = resolve(&cli, &TestEnvironment::default()).unwrap_err();
        assert!(error.to_string().contains("must not contain credentials"));
    }

    #[test]
    fn rejects_unknown_profile() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "output = \"human\"").unwrap();
        let cli = cli(&[
            "--config",
            file.path().to_str().unwrap(),
            "--profile",
            "missing",
            "status",
        ]);
        let error = resolve(&cli, &TestEnvironment::default()).unwrap_err();
        assert!(error.to_string().contains("does not exist"));
    }
}
