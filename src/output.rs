use std::io::Write;

use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::client::dto::{ApiKey, GraphRecord, IssuedKey};
use crate::commands::CommandResult;
use crate::error::AppError;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    #[default]
    Human,
    Json,
}

pub fn render_success(
    writer: &mut dyn Write,
    mode: OutputMode,
    result: &CommandResult,
) -> Result<(), AppError> {
    match mode {
        OutputMode::Human => render_human(writer, result),
        OutputMode::Json => render_json(writer, result),
    }
}

pub fn render_error(
    writer: &mut dyn Write,
    mode: OutputMode,
    error: &AppError,
) -> Result<(), std::io::Error> {
    match mode {
        OutputMode::Human => {
            writeln!(writer, "Error: {error}")?;
            if let Some(code) = error.api_code() {
                writeln!(writer, "Drift code: {code}")?;
            }
            Ok(())
        }
        OutputMode::Json => {
            let mut body = json!({
                "schemaVersion": 1,
                "error": {
                    "kind": error.kind(),
                    "message": error.to_string(),
                }
            });
            if let Some(status) = error.api_status() {
                body["error"]["httpStatus"] = json!(status);
            }
            if let Some(code) = error.api_code() {
                body["error"]["code"] = json!(code);
            }
            serde_json::to_writer_pretty(&mut *writer, &body)?;
            writeln!(writer)
        }
    }
}

fn render_json(writer: &mut dyn Write, result: &CommandResult) -> Result<(), AppError> {
    let (command, data) = match result {
        CommandResult::Status { endpoint, api } => (
            "status",
            json!({
                "healthy": true,
                "endpoint": endpoint,
                "api": api,
            }),
        ),
        CommandResult::KeyList(keys) => ("key.list", json!({ "keys": keys })),
        CommandResult::KeyCreated(key) => ("key.create", issued_key_json(key)),
        CommandResult::KeyRevoked { id } => ("key.revoke", json!({ "id": id, "revoked": true })),
        CommandResult::KeyRotated(key) => ("key.rotate", issued_key_json(key)),
        CommandResult::RecoveryShown { kind, record } => (
            "recovery.show",
            json!({
                "resourceKind": kind.as_str(),
                "state": record_state(record),
                "record": graph_record_value(record),
            }),
        ),
        CommandResult::RecoveryRestored { kind, record } => (
            "recovery.restore",
            json!({
                "resourceKind": kind.as_str(),
                "restored": true,
                "record": graph_record_value(record),
            }),
        ),
    };
    let body = json!({
        "schemaVersion": 1,
        "command": command,
        "data": data,
    });
    serde_json::to_writer_pretty(&mut *writer, &body)
        .map_err(|error| AppError::Output(std::io::Error::other(error)))?;
    writeln!(writer)?;
    Ok(())
}

fn issued_key_json(key: &IssuedKey) -> Value {
    json!({
        "apiKey": key.api_key,
        "secret": key.secret.expose_secret(),
    })
}

fn graph_record_value(record: &GraphRecord) -> Value {
    match record {
        GraphRecord::Vertex(value) => json!(value),
        GraphRecord::Edge(value) => json!(value),
    }
}

fn render_human(writer: &mut dyn Write, result: &CommandResult) -> Result<(), AppError> {
    match result {
        CommandResult::Status { endpoint, api } => {
            writeln!(writer, "Drift is healthy")?;
            writeln!(writer)?;
            writeln!(writer, "Endpoint: {endpoint}")?;
            writeln!(writer, "API:      {} {}", api.title, api.version)?;
        }
        CommandResult::KeyList(keys) => render_key_list(writer, keys)?,
        CommandResult::KeyCreated(key) => {
            writeln!(writer, "Key created")?;
            render_issued_key(writer, key)?;
        }
        CommandResult::KeyRevoked { id } => {
            writeln!(writer, "Key revoked")?;
            writeln!(writer)?;
            writeln!(writer, "ID: {id}")?;
        }
        CommandResult::KeyRotated(key) => {
            writeln!(writer, "Key rotated")?;
            writeln!(writer, "The old key is no longer valid.")?;
            render_issued_key(writer, key)?;
        }
        CommandResult::RecoveryShown { kind, record } => {
            writeln!(writer, "{} inspected", title(kind.as_str()))?;
            render_record(writer, record)?;
        }
        CommandResult::RecoveryRestored { kind, record } => {
            writeln!(writer, "{} restored", title(kind.as_str()))?;
            render_record(writer, record)?;
        }
    }
    Ok(())
}

fn render_key_list(writer: &mut dyn Write, keys: &[ApiKey]) -> Result<(), std::io::Error> {
    writeln!(writer, "Keys: {}", keys.len())?;
    for (index, key) in keys.iter().enumerate() {
        writeln!(writer)?;
        writeln!(writer, "ID:        {}", key.id)?;
        writeln!(writer, "Label:     {}", key.label)?;
        writeln!(writer, "Prefix:    {}", key.prefix)?;
        writeln!(writer, "Scopes:    {}", scopes(key))?;
        writeln!(
            writer,
            "State:     {}",
            if key.revoked_at.is_some() {
                "revoked"
            } else {
                "active"
            }
        )?;
        writeln!(writer, "Created:   {}", key.created_at)?;
        if index == 0 {
            writeln!(writer, "Tenant ID: {}", key.tenant_id)?;
        }
    }
    Ok(())
}

fn render_issued_key(writer: &mut dyn Write, key: &IssuedKey) -> Result<(), std::io::Error> {
    writeln!(writer)?;
    writeln!(writer, "ID:     {}", key.api_key.id)?;
    writeln!(writer, "Label:  {}", key.api_key.label)?;
    writeln!(writer, "Scopes: {}", scopes(&key.api_key))?;
    writeln!(writer, "Secret: {}", key.secret.expose_secret())?;
    writeln!(writer)?;
    writeln!(
        writer,
        "Save this secret now; Drift will not return it again."
    )
}

fn render_record(writer: &mut dyn Write, record: &GraphRecord) -> Result<(), std::io::Error> {
    writeln!(writer)?;
    writeln!(writer, "ID:      {}", record.id())?;
    writeln!(writer, "Version: {}", record.version())?;
    writeln!(writer, "State:   {}", record_state(record))?;
    match record {
        GraphRecord::Vertex(vertex) => {
            writeln!(writer, "Type:    {}", vertex.kind)?;
            if let Some(title) = &vertex.title {
                writeln!(writer, "Title:   {title}")?;
            }
        }
        GraphRecord::Edge(edge) => {
            writeln!(writer, "Type:    {}", edge.kind)?;
            writeln!(writer, "From:    {}", edge.from_vertex_id)?;
            writeln!(writer, "To:      {}", edge.to_vertex_id)?;
        }
    }
    if let Some(deleted_at) = record.deleted_at() {
        writeln!(writer, "Deleted: {deleted_at}")?;
    }
    Ok(())
}

fn scopes(key: &ApiKey) -> String {
    key.scopes
        .iter()
        .map(|scope| scope.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn record_state(record: &GraphRecord) -> &'static str {
    if record.deleted_at().is_some() {
        "soft-deleted"
    } else {
        "active"
    }
}

fn title(value: &str) -> String {
    let mut characters = value.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
        None => String::new(),
    }
}
