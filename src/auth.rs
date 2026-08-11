use std::io::BufRead;

use secrecy::SecretString;

use crate::config::Environment;
use crate::error::AppError;

pub fn resolve_credential(
    key_stdin: bool,
    profile_environment: Option<&str>,
    environment: &dyn Environment,
    stdin: &mut dyn BufRead,
) -> Result<SecretString, AppError> {
    if key_stdin {
        let mut value = String::new();
        stdin.read_to_string(&mut value).map_err(|error| {
            AppError::Credential(format!("could not read key from stdin: {error}"))
        })?;
        trim_line_ending(&mut value);
        return secret_from(value, "standard input");
    }

    if let Some(value) = environment.get("DRIFT_API_KEY") {
        return secret_from(value, "DRIFT_API_KEY");
    }

    if let Some(name) = profile_environment {
        let value = environment.get(name).ok_or_else(|| {
            AppError::Credential(format!(
                "profile credential environment variable {name} is not set"
            ))
        })?;
        return secret_from(value, name);
    }

    Err(AppError::Credential(
        "no Drift API key; use --key-stdin, DRIFT_API_KEY, or a profile credential_env".to_owned(),
    ))
}

fn secret_from(value: String, source: &str) -> Result<SecretString, AppError> {
    if value.is_empty() {
        return Err(AppError::Credential(format!(
            "credential from {source} is empty"
        )));
    }
    Ok(SecretString::from(value))
}

fn trim_line_ending(value: &mut String) {
    if value.ends_with('\n') {
        value.pop();
        if value.ends_with('\r') {
            value.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Cursor;

    use secrecy::ExposeSecret;

    use super::*;

    struct TestEnvironment(HashMap<String, String>);

    impl Environment for TestEnvironment {
        fn get(&self, name: &str) -> Option<String> {
            self.0.get(name).cloned()
        }
    }

    #[test]
    fn stdin_takes_precedence_and_trims_one_line_ending() {
        let environment = TestEnvironment(HashMap::from([(
            "DRIFT_API_KEY".to_owned(),
            "environment-key".to_owned(),
        )]));
        let mut input = Cursor::new("stdin-key\r\n");

        let secret = resolve_credential(true, None, &environment, &mut input).expect("credential");
        assert_eq!(secret.expose_secret(), "stdin-key");
    }

    #[test]
    fn profile_environment_is_used_after_direct_environment() {
        let environment = TestEnvironment(HashMap::from([(
            "PROFILE_KEY".to_owned(),
            "profile-key".to_owned(),
        )]));
        let mut input = Cursor::new("");

        let secret = resolve_credential(false, Some("PROFILE_KEY"), &environment, &mut input)
            .expect("credential");
        assert_eq!(secret.expose_secret(), "profile-key");
    }

    #[test]
    fn missing_credential_is_actionable() {
        let environment = TestEnvironment(HashMap::new());
        let mut input = Cursor::new("");
        let error = resolve_credential(false, None, &environment, &mut input).unwrap_err();
        assert!(error.to_string().contains("DRIFT_API_KEY"));
    }
}
