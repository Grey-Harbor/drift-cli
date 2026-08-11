use std::collections::HashMap;
use std::io::Cursor;

use clap::Parser;
use drift_cli::cli::Cli;
use drift_cli::config::Environment;
use mockito::{Matcher, Server};
use serde_json::{Value, json};

#[derive(Default)]
struct TestEnvironment(HashMap<String, String>);

impl TestEnvironment {
    fn with_key(key: &str) -> Self {
        Self(HashMap::from([(
            "DRIFT_API_KEY".to_owned(),
            key.to_owned(),
        )]))
    }
}

impl Environment for TestEnvironment {
    fn get(&self, name: &str) -> Option<String> {
        self.0.get(name).cloned()
    }
}

fn run(arguments: &[&str], environment: &TestEnvironment) -> (u8, String, String) {
    let cli = Cli::try_parse_from(std::iter::once("drift").chain(arguments.iter().copied()))
        .expect("valid CLI arguments");
    let mut stdin = Cursor::new(Vec::<u8>::new());
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let code =
        drift_cli::run_with_environment(cli, &mut stdin, &mut stdout, &mut stderr, environment);
    (
        code,
        String::from_utf8(stdout).unwrap(),
        String::from_utf8(stderr).unwrap(),
    )
}

fn key(id: &str, label: &str, scopes: &[&str]) -> Value {
    json!({
        "id": id,
        "tenantId": "tenant-1",
        "label": label,
        "prefix": "prefix",
        "scopes": scopes,
        "createdAt": "2026-08-11T12:00:00.000Z",
        "lastUsedAt": null,
        "revokedAt": null
    })
}

fn vertex(deleted: bool) -> Value {
    json!({
        "id": "vertex-1",
        "tenantId": "tenant-1",
        "type": "service",
        "slug": "beacon",
        "externalId": null,
        "title": "Beacon",
        "status": "active",
        "data": {},
        "metadata": {},
        "version": if deleted { 2 } else { 3 },
        "createdAt": "2026-08-11T12:00:00.000Z",
        "updatedAt": "2026-08-11T12:05:00.000Z",
        "deletedAt": if deleted { Some("2026-08-11T12:05:00.000Z") } else { None }
    })
}

fn edge(deleted: bool) -> Value {
    json!({
        "id": "edge-1",
        "tenantId": "tenant-1",
        "fromVertexId": "vertex-1",
        "toVertexId": "vertex-2",
        "type": "contains",
        "status": "active",
        "data": {},
        "metadata": {},
        "version": if deleted { 4 } else { 5 },
        "createdAt": "2026-08-11T12:00:00.000Z",
        "updatedAt": "2026-08-11T12:05:00.000Z",
        "deletedAt": if deleted { Some("2026-08-11T12:05:00.000Z") } else { None }
    })
}

#[test]
fn status_checks_health_and_contract_without_a_key() {
    let mut server = Server::new();
    let health = server
        .mock("GET", "/health")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"status":"ok"}"#)
        .create();
    let openapi = server
        .mock("GET", "/v1/openapi.json")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"openapi":"3.0.3","info":{"title":"Drift API","version":"1.0.0"}}"#)
        .create();

    let (code, stdout, stderr) = run(
        &["--endpoint", &server.url(), "--json", "status"],
        &TestEnvironment::default(),
    );

    assert_eq!(code, 0, "{stderr}");
    let output: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(output["command"], "status");
    assert_eq!(output["data"]["healthy"], true);
    assert_eq!(output["data"]["api"]["version"], "1.0.0");
    health.assert();
    openapi.assert();
}

#[test]
fn key_list_sends_bearer_auth_and_never_renders_an_unexpected_secret() {
    let mut server = Server::new();
    let mut key = key("key-1", "bootstrap admin", &["admin"]);
    key["secret"] = json!("must-not-leak");
    let request = server
        .mock("GET", "/v1/admin/keys")
        .match_header("authorization", "Bearer admin-secret")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(json!([key]).to_string())
        .create();

    let (code, stdout, stderr) = run(
        &["--endpoint", &server.url(), "--json", "key", "list"],
        &TestEnvironment::with_key("admin-secret"),
    );

    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("bootstrap admin"));
    assert!(!stdout.contains("admin-secret"));
    assert!(!stdout.contains("must-not-leak"));
    request.assert();
}

#[test]
fn key_create_returns_the_one_time_secret_only_on_success() {
    let mut server = Server::new();
    let request = server
        .mock("POST", "/v1/admin/keys")
        .match_header("authorization", "Bearer admin-secret")
        .match_body(Matcher::Json(json!({
            "label": "reporting",
            "scopes": ["read"]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "apiKey": key("key-2", "reporting", &["read"]),
                "secret": "drift_new.secret"
            })
            .to_string(),
        )
        .create();

    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "--json",
            "key",
            "create",
            "--label",
            "reporting",
            "--scope",
            "read",
        ],
        &TestEnvironment::with_key("admin-secret"),
    );

    assert_eq!(code, 0, "{stderr}");
    let output: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(output["data"]["secret"], "drift_new.secret");
    assert!(!stderr.contains("admin-secret"));
    request.assert();
}

#[test]
fn key_revoke_and_rotate_require_acknowledgement_and_match_drift_routes() {
    let environment = TestEnvironment::with_key("admin-secret");
    let mut server = Server::new();

    let missing_acknowledgement = Cli::try_parse_from([
        "drift",
        "--endpoint",
        &server.url(),
        "key",
        "revoke",
        "key-1",
    ]);
    assert!(missing_acknowledgement.is_err());

    let revoke = server
        .mock("DELETE", "/v1/admin/keys/key-1")
        .match_header("authorization", "Bearer admin-secret")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"ok":true}"#)
        .create();
    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "key",
            "revoke",
            "key-1",
            "--yes",
        ],
        &environment,
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("Key revoked"));
    revoke.assert();

    let rotate = server
        .mock("POST", "/v1/admin/keys/key-2/rotate")
        .match_header("authorization", "Bearer admin-secret")
        .match_body(Matcher::Json(json!({
            "label": "service",
            "scopes": ["read", "write"]
        })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "apiKey": key("key-3", "service", &["read", "write"]),
                "secret": "replacement-secret"
            })
            .to_string(),
        )
        .create();
    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "key",
            "rotate",
            "key-2",
            "--label",
            "service",
            "--scope",
            "read",
            "--scope",
            "write",
            "--yes",
        ],
        &environment,
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("replacement-secret"));
    rotate.assert();
}

#[test]
fn recovery_inspects_known_vertices_and_edges() {
    let mut server = Server::new();
    let vertex_request = server
        .mock("GET", "/v1/vertices/vertex-1?includeDeleted=true")
        .match_header("authorization", "Bearer admin-secret")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(vertex(true).to_string())
        .create();
    let edge_request = server
        .mock("GET", "/v1/edges/edge-1?includeDeleted=true")
        .match_header("authorization", "Bearer admin-secret")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(edge(true).to_string())
        .create();
    let environment = TestEnvironment::with_key("admin-secret");

    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "recovery",
            "show",
            "vertex",
            "vertex-1",
        ],
        &environment,
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("soft-deleted"));
    vertex_request.assert();

    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "--json",
            "recovery",
            "show",
            "edge",
            "edge-1",
        ],
        &environment,
    );
    assert_eq!(code, 0, "{stderr}");
    let output: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(output["data"]["state"], "soft-deleted");
    assert_eq!(output["data"]["record"]["fromVertexId"], "vertex-1");
    edge_request.assert();
}

#[test]
fn recovery_restores_vertices_and_edges_with_explicit_versions() {
    let mut server = Server::new();
    let vertex_request = server
        .mock("POST", "/v1/vertices/vertex-1/restore")
        .match_header("authorization", "Bearer admin-secret")
        .match_body(Matcher::Json(json!({ "version": 2 })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(vertex(false).to_string())
        .create();
    let edge_request = server
        .mock("POST", "/v1/edges/edge-1/restore")
        .match_header("authorization", "Bearer admin-secret")
        .match_body(Matcher::Json(json!({ "version": 4 })))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(edge(false).to_string())
        .create();
    let environment = TestEnvironment::with_key("admin-secret");

    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "recovery",
            "restore",
            "vertex",
            "vertex-1",
            "--version",
            "2",
        ],
        &environment,
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("Vertex restored"));
    vertex_request.assert();

    let (code, stdout, stderr) = run(
        &[
            "--endpoint",
            &server.url(),
            "recovery",
            "restore",
            "edge",
            "edge-1",
            "--version",
            "4",
        ],
        &environment,
    );
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("Edge restored"));
    edge_request.assert();
}

#[test]
fn api_errors_map_to_stable_json_without_exposing_the_bearer_key() {
    let mut server = Server::new();
    let request = server
        .mock("GET", "/v1/admin/keys")
        .match_header("authorization", "Bearer super-secret-admin-key")
        .with_status(403)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "error": {
                    "code": "forbidden",
                    "message": "Admin scope required",
                    "secret": "unexpected-response-secret"
                }
            })
            .to_string(),
        )
        .create();

    let (code, stdout, stderr) = run(
        &["--endpoint", &server.url(), "--json", "key", "list"],
        &TestEnvironment::with_key("super-secret-admin-key"),
    );

    assert_eq!(code, 4);
    assert!(stdout.is_empty());
    let output: Value = serde_json::from_str(&stderr).unwrap();
    assert_eq!(output["error"]["httpStatus"], 403);
    assert_eq!(output["error"]["code"], "forbidden");
    assert!(!stderr.contains("super-secret-admin-key"));
    assert!(!stderr.contains("unexpected-response-secret"));
    request.assert();
}
