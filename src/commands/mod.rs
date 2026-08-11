use crate::cli::{Command, KeyCommand, KeySpec, RecoveryCommand, ResourceKind};
use crate::client::DriftClient;
use crate::client::dto::{ApiKey, GraphRecord, IssuedKey, OpenApiInfo};
use crate::error::AppError;

#[derive(Debug)]
pub enum CommandResult {
    Status {
        endpoint: String,
        api: OpenApiInfo,
    },
    KeyList(Vec<ApiKey>),
    KeyCreated(IssuedKey),
    KeyRevoked {
        id: String,
    },
    KeyRotated(IssuedKey),
    RecoveryShown {
        kind: ResourceKind,
        record: GraphRecord,
    },
    RecoveryRestored {
        kind: ResourceKind,
        record: GraphRecord,
    },
}

pub fn execute(command: &Command, client: &DriftClient) -> Result<CommandResult, AppError> {
    match command {
        Command::Status => {
            client.health()?;
            let api = client.openapi_info()?;
            Ok(CommandResult::Status {
                endpoint: client.endpoint().as_str().trim_end_matches('/').to_owned(),
                api,
            })
        }
        Command::Key { command } => execute_key(command, client),
        Command::Recovery { command } => execute_recovery(command, client),
    }
}

fn execute_key(command: &KeyCommand, client: &DriftClient) -> Result<CommandResult, AppError> {
    match command {
        KeyCommand::List => client.list_keys().map(CommandResult::KeyList),
        KeyCommand::Create(spec) => {
            validate_spec(spec)?;
            client
                .create_key(&spec.label, &spec.scopes)
                .map(CommandResult::KeyCreated)
        }
        KeyCommand::Revoke { id, yes } => {
            validate_id(id)?;
            require_acknowledgement(*yes, "key revoke", "use --yes to revoke the key")?;
            client.revoke_key(id)?;
            Ok(CommandResult::KeyRevoked { id: id.clone() })
        }
        KeyCommand::Rotate { id, spec, yes } => {
            validate_id(id)?;
            validate_spec(spec)?;
            require_acknowledgement(
                *yes,
                "key rotate",
                "use --yes to acknowledge immediate revocation of the old key",
            )?;
            client
                .rotate_key(id, &spec.label, &spec.scopes)
                .map(CommandResult::KeyRotated)
        }
    }
}

fn execute_recovery(
    command: &RecoveryCommand,
    client: &DriftClient,
) -> Result<CommandResult, AppError> {
    match command {
        RecoveryCommand::Show { kind, id } => {
            validate_id(id)?;
            let record = client.get_record(*kind, id)?;
            Ok(CommandResult::RecoveryShown {
                kind: *kind,
                record,
            })
        }
        RecoveryCommand::Restore { kind, id, version } => {
            validate_id(id)?;
            if *version == 0 {
                return Err(AppError::Usage(
                    "recovery restore requires a positive --version".to_owned(),
                ));
            }
            let record = client.restore_record(*kind, id, *version)?;
            Ok(CommandResult::RecoveryRestored {
                kind: *kind,
                record,
            })
        }
    }
}

fn validate_spec(spec: &KeySpec) -> Result<(), AppError> {
    if spec.label.trim().is_empty() {
        return Err(AppError::Usage(
            "key label must contain a non-whitespace character".to_owned(),
        ));
    }
    if spec.scopes.is_empty() {
        return Err(AppError::Usage(
            "at least one key scope is required".to_owned(),
        ));
    }
    Ok(())
}

fn validate_id(id: &str) -> Result<(), AppError> {
    if id.is_empty() {
        Err(AppError::Usage("resource ID must not be empty".to_owned()))
    } else {
        Ok(())
    }
}

fn require_acknowledgement(yes: bool, operation: &str, message: &str) -> Result<(), AppError> {
    if yes {
        Ok(())
    } else {
        Err(AppError::Usage(format!(
            "{operation} was not sent: {message}"
        )))
    }
}
