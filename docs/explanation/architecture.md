# Client architecture and dependency proposal

Use this design record to review the Rust foundation before Phase 1 begins. The goal is a small synchronous command-line client with explicit boundaries, not a reusable Drift application SDK.

## Component boundaries

```text
main
  -> CLI parser
  -> configuration + credential resolver
  -> domain command handler
  -> Drift HTTP client
  -> request/response DTOs
  -> output renderer
```

The parser produces typed command intent and performs no HTTP calls. Command handlers coordinate operations but do not build URLs or deserialize JSON. The client owns transport, bearer authentication, status handling, and narrow DTOs aligned with the OpenAPI contract. Renderers turn domain results into human or stable JSON output.

Proposed source layout:

```text
src/
  main.rs
  lib.rs
  cli.rs
  config.rs
  auth.rs
  error.rs
  client/
    mod.rs
    dto.rs
  commands/
    mod.rs
    status.rs
    keys.rs
    recovery.rs
  output/
    mod.rs
    human.rs
    json.rs
tests/
  cli.rs
  config.rs
  http_client.rs
  json_output.rs
  contract/
```

`lib.rs` exposes internal seams to integration tests; it is not a promise of a public Rust SDK.

## Contract strategy

Drift serves its OpenAPI document at runtime, but the repository has no checked-in generated client or standalone schema artifact to import. Phase 1 should therefore define only the small DTO set required by approved commands in `client::dto`.

Contract tests will compare requests and representative responses with the reviewed OpenAPI document. A separate, opt-in live test suite will run against a pinned Drift release. Automatic client generation is deferred until it demonstrably reduces drift without exposing the entire application-data API as CLI surface.

## Dependency review

The following current stable releases were reviewed from crates.io on 2026-08-11. Cargo should use compatible version requirements and commit `Cargo.lock` because drift-cli is an application.

| Crate         | Reviewed version | Role and justification                                                                                                                                                                                                             |
| ------------- | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `clap`        | 4.6.6            | Derive-based, typed parsing, generated help, conflicts, and repeatable scope options. Parsing behavior remains testable without HTTP.                                                                                              |
| `reqwest`     | 0.13.4           | Mature HTTP client with blocking and JSON features. A sequential administration CLI does not need an application-level async runtime. Its default TLS stack is Rustls, avoiding a system OpenSSL requirement for release binaries. |
| `serde`       | 1.0.229          | Derives narrowly aligned request, response, configuration, and output contracts.                                                                                                                                                   |
| `serde_json`  | 1.0.151          | Drift transport and stable automation output are JSON.                                                                                                                                                                             |
| `toml`        | 1.1.4            | Parses a small human-editable, non-secret configuration file without a larger configuration framework.                                                                                                                             |
| `directories` | 6.0.0            | Resolves platform-standard configuration paths consistently across Linux, macOS, and future Windows support.                                                                                                                       |
| `thiserror`   | 2.0.20           | Typed internal errors without using unstructured strings as control flow.                                                                                                                                                          |
| `secrecy`     | 0.10.3           | Redacting, zeroizing wrapper for bearer credentials and one-time secret handling.                                                                                                                                                  |

Proposed development dependencies:

| Crate        | Reviewed version | Role and justification                                                                               |
| ------------ | ---------------- | ---------------------------------------------------------------------------------------------------- |
| `assert_cmd` | 2.2.2            | Black-box CLI exit status and stdout/stderr tests.                                                   |
| `predicates` | 3.1.4            | Focused assertions for command output and secret absence.                                            |
| `mockito`    | 1.7.2            | Local HTTP expectations compatible with the blocking client; most tests need no live Drift instance. |

No general-purpose async runtime, logging framework, table-rendering crate, configuration framework, keyring integration, or OpenAPI generator is justified in Phase 1. Plain text rendering can remain local until display complexity proves otherwise.

The proposed minimum supported Rust version is 1.85, matching the minimum declared by the current `clap` and `reqwest` releases. CI will build the pinned current stable toolchain as well as the MSRV before distribution targets are finalized.

## Error and retry design

Errors are classified as CLI usage, configuration, credential, transport, Drift API, response-contract, and unexpected internal failures. The API error type retains HTTP status, Drift code, safe message, and optional details, but never authorization headers.

The client will not automatically retry mutations. Status and safe read-only operations may later gain bounded retries only after timeout and retry semantics are documented. Every request receives explicit connect and total timeouts.

## Distribution shape

The repository will produce one `drift` binary. Phase 5 will build release artifacts for Linux x86_64, Linux arm64, macOS x86_64, and macOS arm64, with checksums and a release manifest. Cross-compilation and TLS behavior must be validated before a release workflow is considered complete.
