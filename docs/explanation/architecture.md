# Client architecture

Use this explanation when changing Drift CLI internals or evaluating a new dependency. The architecture keeps command parsing, Drift transport, and user-facing output separate so server contracts do not leak across the client.

## Component boundaries

```text
main
  -> CLI parser
  -> configuration + credential resolver
  -> command handler
  -> Drift HTTP client
  -> request/response DTOs
  -> output renderer
```

The parser produces typed command intent and performs no HTTP calls. Configuration resolution determines non-secret settings and the credential source before a command runs. Command handlers coordinate a workflow without building URLs or deserializing response bodies.

The HTTP client owns URL construction, bearer authentication, timeouts, status handling, and narrow DTOs aligned with the Drift API. The output layer converts successful results and typed failures into human-readable text or the stable JSON envelope.

The current source layout reflects those boundaries:

```text
src/
  main.rs        process entry point
  lib.rs         execution seam used by integration tests
  cli.rs         clap command model
  config.rs      non-secret configuration resolution
  auth.rs        credential-source resolution
  client/        HTTP transport and Drift DTOs
  commands/      command workflow coordination
  output.rs      human and JSON rendering
  error.rs       typed failure classification
```

`lib.rs` supports testable composition; it is not a public Rust SDK contract.

## Why the client is synchronous

Administrative commands make a small, ordered set of requests and then exit. A blocking `reqwest` client keeps control flow and error handling direct without introducing an application-level async runtime.

This choice would need review if a future workflow required concurrent requests, streaming, or long-lived sessions. It is not a general claim that asynchronous Rust is unsuitable.

## Contract isolation

The live OpenAPI document served by Drift is authoritative. Drift CLI defines only the request and response DTOs needed by its approved commands rather than generating or exposing the entire application-data API.

Keeping DTOs inside the client layer prevents server wire shapes from becoming command-parser or presentation types. Contract changes are tested at request construction and response mapping boundaries.

## Failure and retry policy

Errors are classified as CLI usage, configuration, credential, transport, Drift API, response-contract, and unexpected internal failures. The API error representation retains safe status and code information but never authorization headers.

Mutations are not retried automatically. A timeout after key creation, revocation, rotation, or record restore can leave the outcome uncertain, so an operator must inspect state before choosing another mutation.

## Dependency policy

Dependencies are justified when they remove meaningful complexity at a component boundary. The CLI currently uses focused crates for parsing, HTTP, serialization, configuration, platform paths, typed errors, and secret handling.

It deliberately avoids a general configuration framework, logging framework, table renderer, keyring integration, and OpenAPI generator until one of those tools solves a demonstrated problem without broadening the product surface.

## Testing shape

Unit tests cover parsing, precedence, and secret-safe behavior. Integration tests use a local mock HTTP server to verify routes, headers, bodies, response mapping, errors, and stable JSON output without requiring a live Drift instance.

Live contract tests, when added, remain opt-in and target a pinned Drift release. The normal suite must stay deterministic and offline.
