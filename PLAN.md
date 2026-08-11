# Drift CLI implementation plan

This plan is the approval boundary between kickoff research and implementation. It is based on Drift commit `d3e8e16012b42a4d5aa2889172681d0e5f389e34` and the contract review recorded in [the API inventory](docs/reference/drift-api-inventory.md).

## Findings that shape the plan

- Drift v1 has health, OpenAPI, tenant-scoped key administration, soft-deleted record reads, and restore endpoints.
- Drift v1 has no tenant-management or bootstrap HTTP API. Its local bootstrap command accesses SQLite through server internals and cannot be reused by this independent client.
- There is no global/original admin tenant. Every bootstrap creates a new tenant and that tenant's distinct admin key.
- Drift has no archive resource. It exposes soft deletion and restore for vertices and edges.
- `includeDeleted=true` returns active and deleted records together, so deleted-only listing is not an exact existing operation.
- The live OpenAPI document is canonical. No reusable generated client artifact is checked into Drift.

These findings intentionally narrow the first implementation. Tenant commands remain blocked until Drift approves and ships an HTTP contract with an appropriate instance-level authorization model.

## Phase 1: repository foundation and CLI shell — complete

### Scope

- Create the Cargo application and commit `Cargo.lock`.
- Set Rust edition, MSRV, package metadata, lint policy, formatting policy, and release profiles.
- Implement typed command parsing for the approved global options, `status`, and placeholder command groups whose parsing behavior is settled.
- Implement non-secret TOML profiles and deterministic CLI/environment/profile/default precedence.
- Implement environment and explicit-standard-input credential sources without plaintext persistence.
- Implement a blocking Drift HTTP client with URL joining, bearer auth, connect/total timeouts, narrow DTOs, API error decoding, response-contract errors, and secret redaction.
- Implement human and versioned JSON output seams.
- Implement `drift status` against `/health`, with optional OpenAPI discovery that does not overstate readiness.
- Add command parsing, configuration, mock HTTP, error mapping, secret-redaction, and JSON snapshot tests.
- Document the separate opt-in contract-test strategy against a pinned real Drift release.

### Decisions applied

- The command spelling in `docs/reference/initial-command-surface.md` is implemented.
- Configuration and credential precedence follow the documented design.
- Status always checks health and discovers the OpenAPI title/version.
- The project is licensed under Apache-2.0.

### Exit criteria

- `cargo fmt --check`, strict Clippy, and all offline tests pass.
- `drift --help`, `drift status`, and `drift status --json` are stable and documented.
- Tests prove credentials cannot appear in rendered errors or debug output.
- No code reads Drift files or depends on Drift implementation modules.

## Phase 2: tenant administration

### Status

Blocked on server capability.

### Required upstream design proposal

- Define whether Drift should expose tenant creation, list, and show over HTTP.
- Define a new instance-level authorization model without treating a tenant admin key as global.
- Treat authorization by an ordinary tenant admin key as an explicit security rejection, not an implementation option.
- Define bootstrap availability, replay protection, initialization state, secret return, auditability, rate limits, and deployment-secret rotation.
- Publish OpenAPI schemas, human documentation, and contract tests in Drift before drift-cli implements commands.
- Decide whether server-local bootstrap remains supported and how its semantics relate to any HTTP route.

### Exit criteria

- A released Drift contract authoritatively supports the approved tenant operations.
- drift-cli DTOs and mock fixtures are derived from that contract.
- Cross-tenant isolation and authorization failures have live contract tests.

No Phase 2 code may access SQLite, invoke Drift's internal TypeScript CLI, or infer tenant data from unrelated records.

## Phase 3: key administration — complete

### Scope

- Implement key metadata listing.
- Implement key creation with an explicit non-empty label and one or more explicit scopes.
- Implement irreversible revocation with a reviewed interactive/automation acknowledgement.
- Implement immediate rotation with explicit label and scopes; do not imply a grace period.
- Render one-time secrets only for successful create and rotate operations.
- Add stable JSON fixtures and mock-server assertions for route, method, body, bearer header, and error mapping.
- Add tests that list output never includes secrets and failures never echo them.

### Exit criteria

- All four key routes match the target Drift OpenAPI contract.
- Documentation explains tenant scoping, one-time secret handling, and immediate rotation/revocation.
- Non-idempotent requests are never automatically retried.

## Phase 4: soft-delete recovery — known-ID scope complete

### Scope

- Implement inspection of known soft-deleted vertex and edge IDs with `includeDeleted=true`.
- Display current state and version before restore.
- Implement vertex restore with an explicit version.
- Implement edge restore with an explicit version and actionable handling when endpoints are inactive.
- Explain that restoring a vertex does not restore incident edges.
- Keep generic graph creation, mutation, traversal, and retrieval out of scope.

### Deferred listing decision

Do not advertise a complete recovery list while Drift returns active and deleted records in the same paginated result. Choose one of these through a separate review:

1. add a Drift deleted-only filter and implement after release; or
2. explicitly approve client-side full pagination/filtering with documented cost, race, and completeness limitations.

### Exit criteria

- Recovery commands use Drift's soft-delete and restore terminology in output.
- Version conflicts, active-record conflicts, inactive edge endpoints, and cross-tenant not-found behavior are tested.
- No command claims an archive object exists.

## Phase 5: documentation and distribution polish — documentation complete, distribution pending

### Documentation

- Write `docs/tutorial/getting-started.md` using only released commands.
- Write focused how-to guides for admin access, key creation, revocation, recovery, and JSON output.
- Publish concise command, configuration, environment, output schema, and exit-code references.
- Add a tenant guide only after Phase 2 is supported.
- Cross-link canonical Drift contracts rather than copying them.

### Distribution

- Build Linux x86_64, Linux arm64, macOS x86_64, and macOS arm64 release binaries.
- Test each artifact, generate checksums, and document verification.
- Add GitHub Releases automation only after manual target builds validate TLS and cross-compilation.
- Add supply-chain metadata and signing as a separately reviewed hardening step.

### Exit criteria

- A new operator can obtain a binary, configure an endpoint and secret source, inspect status, and complete supported admin workflows from the documentation.
- Release artifacts run without a Node or Rust installation.
- Docs contain no real secrets and no examples for unsupported commands.

## Remaining work

The tenant-scoped CLI is implemented. Remaining work is limited to:

1. an upstream Drift proposal for tenant administration and deleted-only listing; and
2. validating release targets before adding GitHub Releases automation.
