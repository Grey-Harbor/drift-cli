use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cli::Scope;

#[derive(Debug, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenApiDocument {
    pub info: OpenApiInfo,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OpenApiInfo {
    pub title: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: String,
    pub tenant_id: String,
    pub label: String,
    pub prefix: String,
    pub scopes: Vec<Scope>,
    pub created_at: String,
    pub last_used_at: Option<String>,
    pub revoked_at: Option<String>,
}

#[derive(Debug)]
pub struct IssuedKey {
    pub api_key: ApiKey,
    pub secret: SecretString,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IssuedKeyResponse {
    pub api_key: ApiKey,
    pub secret: String,
}

impl From<IssuedKeyResponse> for IssuedKey {
    fn from(value: IssuedKeyResponse) -> Self {
        Self {
            api_key: value.api_key,
            secret: SecretString::from(value.secret),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct KeyRequest<'a> {
    pub label: &'a str,
    pub scopes: &'a [Scope],
}

#[derive(Debug, Deserialize)]
pub(crate) struct OperationResponse {
    pub ok: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vertex {
    pub id: String,
    pub tenant_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub slug: Option<String>,
    pub external_id: Option<String>,
    pub title: Option<String>,
    pub status: String,
    pub data: Value,
    pub metadata: Value,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub id: String,
    pub tenant_id: String,
    pub from_vertex_id: String,
    pub to_vertex_id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub status: String,
    pub data: Value,
    pub metadata: Value,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", content = "record", rename_all = "lowercase")]
pub enum GraphRecord {
    Vertex(Vertex),
    Edge(Edge),
}

impl GraphRecord {
    pub fn id(&self) -> &str {
        match self {
            Self::Vertex(record) => &record.id,
            Self::Edge(record) => &record.id,
        }
    }

    pub fn version(&self) -> u64 {
        match self {
            Self::Vertex(record) => record.version,
            Self::Edge(record) => record.version,
        }
    }

    pub fn deleted_at(&self) -> Option<&str> {
        match self {
            Self::Vertex(record) => record.deleted_at.as_deref(),
            Self::Edge(record) => record.deleted_at.as_deref(),
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct RestoreRequest {
    pub version: u64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorEnvelope {
    pub error: DriftErrorBody,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DriftErrorBody {
    pub code: String,
    pub message: String,
}
