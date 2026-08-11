# AGENTS.md

## Project purpose

Drift CLI is a Rust command-line administration client for Drift's existing HTTP API. It replaces hand-built administrative JSON and `curl` workflows with explicit, scriptable operator commands.

The CLI is not a persistence engine, graph editor, application-data authoring surface, or privileged storage backdoor.

## Architectural boundaries

- Drift owns server behavior, authorization, tenancy, and persistence semantics.
- The live Drift OpenAPI contract is authoritative, followed by Drift reference documentation and implementation.
- drift-cli is an independent HTTP client only. Never read or modify Drift storage directly.
- Do not import, copy, or depend on Drift server internals.
- Keep HTTP DTOs isolated from command parsing and human-facing presentation.
- Do not add vertex, edge, traversal, or query authoring commands without explicit approval.
- Recovery commands may wrap existing admin-only soft-delete and restore behavior, but must preserve Drift terminology and optimistic-concurrency rules.
- Do not weaken security or broaden key scopes for operator convenience.
- Treat Drift's `admin` scope as tenant-local. Never use it as implicit instance authority or authorize tenant-registry operations with it.
- A missing server capability is an API gap to document, not a reason to bypass the API.

## Source-of-truth hierarchy

When behavior is ambiguous, consult these sources in order:

1. The OpenAPI document served by the target Drift instance at `GET /v1/openapi.json`.
2. Drift API contracts and schemas in the referenced Drift release.
3. Drift reference documentation for that release.
4. Drift implementation and contract tests.
5. drift-cli documentation.

drift-cli documentation must never redefine Drift behavior.

## Security

- Never log, include in diagnostics, or accidentally expose bearer credentials.
- Treat bootstrap, create-key, and rotate-key secrets as one-time values.
- Keep persisted configuration non-secret unless a separately approved credential-storage design says otherwise.
- Prefer environment variables and standard input over secret-bearing command arguments.
- Redact authorization headers and secret fields from errors and debug output.
- Make irreversible key revocation and rotation explicit in both interactive and automation flows.
- Preserve the tenant/instance trust boundary documented in `docs/explanation/tenant-administration-boundary.md`.

## Development expectations

- Prefer small, focused, backward-compatible changes.
- Do not introduce server assumptions without documenting them and adding contract tests.
- Keep command parsing, configuration, authentication, HTTP transport, DTOs, command handlers, and output formatting separate.
- Add tests for command parsing, configuration precedence, request construction, response/error mapping, secret redaction, and stable JSON output.
- The normal test suite must not require a live Drift instance. Keep live-server contract tests separate.
- Use idiomatic Rust and avoid dependencies that do not remove meaningful complexity.
- Run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets` before considering implementation complete.
- Update `PLAN.md`, architecture notes, and user documentation when behavior or scope changes.

## Git workflow

- Do not prefix branch names with `codex/` or another agent name.
- Use Conventional Commits in the form `<type>(<scope>): <description>`.
- Use imperative mood, keep the subject under 72 characters, and make one logical change per commit.
- Preserve user changes and avoid destructive Git commands unless explicitly requested.
