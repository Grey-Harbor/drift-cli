pub mod dto;

use std::time::Duration;

use reqwest::Url;
use reqwest::blocking::{Client as HttpClient, RequestBuilder, Response};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;

use crate::cli::{ResourceKind, Scope};
use crate::error::AppError;

use self::dto::{
    ApiKey, Edge, ErrorEnvelope, GraphRecord, HealthResponse, IssuedKey, IssuedKeyResponse,
    KeyRequest, OpenApiDocument, OpenApiInfo, OperationResponse, RestoreRequest, Vertex,
};

pub struct DriftClient {
    endpoint: Url,
    credential: Option<SecretString>,
    http: HttpClient,
}

impl DriftClient {
    pub fn new(endpoint: Url, credential: Option<SecretString>) -> Result<Self, AppError> {
        let http = HttpClient::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(30))
            .user_agent(concat!("drift-cli/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self {
            endpoint,
            credential,
            http,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn health(&self) -> Result<(), AppError> {
        let response: HealthResponse = self.send(self.http.get(self.url(&["health"])?))?;
        if response.status != "ok" {
            return Err(AppError::Contract(format!(
                "health status was '{}' instead of 'ok'",
                response.status
            )));
        }
        Ok(())
    }

    pub fn openapi_info(&self) -> Result<OpenApiInfo, AppError> {
        let document: OpenApiDocument =
            self.send(self.http.get(self.url(&["v1", "openapi.json"])?))?;
        if document.info.title.is_empty() || document.info.version.is_empty() {
            return Err(AppError::Contract(
                "OpenAPI info.title and info.version must be non-empty".to_owned(),
            ));
        }
        Ok(document.info)
    }

    pub fn list_keys(&self) -> Result<Vec<ApiKey>, AppError> {
        let request = self.authenticated(self.http.get(self.url(&["v1", "admin", "keys"])?))?;
        self.send(request)
    }

    pub fn create_key(&self, label: &str, scopes: &[Scope]) -> Result<IssuedKey, AppError> {
        let request = self.authenticated(self.http.post(self.url(&["v1", "admin", "keys"])?))?;
        self.send::<IssuedKeyResponse>(request.json(&KeyRequest { label, scopes }))
            .map(Into::into)
    }

    pub fn revoke_key(&self, id: &str) -> Result<(), AppError> {
        let request =
            self.authenticated(self.http.delete(self.url(&["v1", "admin", "keys", id])?))?;
        let response: OperationResponse = self.send(request)?;
        if !response.ok {
            return Err(AppError::Contract(
                "key revocation response did not confirm success".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn rotate_key(
        &self,
        id: &str,
        label: &str,
        scopes: &[Scope],
    ) -> Result<IssuedKey, AppError> {
        let request = self.authenticated(
            self.http
                .post(self.url(&["v1", "admin", "keys", id, "rotate"])?),
        )?;
        self.send::<IssuedKeyResponse>(request.json(&KeyRequest { label, scopes }))
            .map(Into::into)
    }

    pub fn get_record(&self, kind: ResourceKind, id: &str) -> Result<GraphRecord, AppError> {
        let mut url = self.url(&["v1", kind.collection(), id])?;
        url.query_pairs_mut().append_pair("includeDeleted", "true");
        let request = self.authenticated(self.http.get(url))?;
        match kind {
            ResourceKind::Vertex => self.send::<Vertex>(request).map(GraphRecord::Vertex),
            ResourceKind::Edge => self.send::<Edge>(request).map(GraphRecord::Edge),
        }
    }

    pub fn restore_record(
        &self,
        kind: ResourceKind,
        id: &str,
        version: u64,
    ) -> Result<GraphRecord, AppError> {
        let request = self.authenticated(self.http.post(self.url(&[
            "v1",
            kind.collection(),
            id,
            "restore",
        ])?))?;
        let request = request.json(&RestoreRequest { version });
        match kind {
            ResourceKind::Vertex => self.send::<Vertex>(request).map(GraphRecord::Vertex),
            ResourceKind::Edge => self.send::<Edge>(request).map(GraphRecord::Edge),
        }
    }

    fn url(&self, segments: &[&str]) -> Result<Url, AppError> {
        let mut url = self.endpoint.clone();
        let mut path = url.path_segments_mut().map_err(|_| {
            AppError::Config("Drift endpoint cannot be used as a hierarchical URL".to_owned())
        })?;
        path.pop_if_empty();
        path.extend(segments);
        drop(path);
        Ok(url)
    }

    fn authenticated(&self, request: RequestBuilder) -> Result<RequestBuilder, AppError> {
        let credential = self.credential.as_ref().ok_or_else(|| {
            AppError::Credential("this Drift operation requires an API key".to_owned())
        })?;
        Ok(request.bearer_auth(credential.expose_secret()))
    }

    fn send<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, AppError> {
        let response = request.send()?;
        if response.status().is_success() {
            decode_success(response)
        } else {
            Err(decode_api_error(response))
        }
    }
}

fn decode_success<T: DeserializeOwned>(response: Response) -> Result<T, AppError> {
    let status = response.status().as_u16();
    response.json().map_err(|_| {
        AppError::Contract(format!(
            "HTTP {status} body did not match the expected JSON contract"
        ))
    })
}

fn decode_api_error(response: Response) -> AppError {
    let status = response.status().as_u16();
    match response.json::<ErrorEnvelope>() {
        Ok(envelope) => AppError::Api {
            status,
            code: Some(envelope.error.code),
            message: envelope.error.message,
        },
        Err(_) => AppError::Api {
            status,
            code: None,
            message: "response did not contain a Drift error envelope".to_owned(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_ids_are_encoded_as_single_path_segments() {
        let client =
            DriftClient::new(Url::parse("https://drift.example/base/").unwrap(), None).unwrap();

        let url = client
            .url(&["v1", "admin", "keys", "id/with?parts"])
            .unwrap();

        assert_eq!(
            url.as_str(),
            "https://drift.example/base/v1/admin/keys/id%2Fwith%3Fparts"
        );
    }
}
