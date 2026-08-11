# Drift v1 administration API inventory

Use this inventory to decide which drift-cli commands can be implemented without changing Drift. It records the local reference checkout at commit `d3e8e16012b42a4d5aa2889172681d0e5f389e34` on 2026-08-11.

The live OpenAPI document at `GET /v1/openapi.json` is the authoritative contract for a running instance. The reference checkout's schemas, API reference, service, and HTTP contract tests were reviewed to explain semantics not visible from route names alone.

## Operator-relevant HTTP routes

| HTTP route                                  | Authentication | Scope   | Contract relevant to drift-cli                                                                                                  |
| ------------------------------------------- | -------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `GET /health`                               | None           | None    | Returns `{ "status": "ok" }`. It proves only that the HTTP process responds.                                                    |
| `GET /v1/openapi.json`                      | None           | None    | Returns the canonical OpenAPI 1.0.0 contract for that instance.                                                                 |
| `GET /v1/admin/keys`                        | Bearer key     | `admin` | Lists key metadata for the calling key's tenant. Raw secrets are never returned.                                                |
| `POST /v1/admin/keys`                       | Bearer key     | `admin` | Requires `label` and one or more explicit `read`, `write`, or `admin` scopes. Returns metadata plus the raw secret once.        |
| `DELETE /v1/admin/keys/{id}`                | Bearer key     | `admin` | Immediately revokes a key in the caller's tenant. Returns `{ "ok": true }`; revocation cannot be undone.                        |
| `POST /v1/admin/keys/{id}/rotate`           | Bearer key     | `admin` | Immediately revokes the old key and creates a replacement using the supplied label and scopes. Returns the new raw secret once. |
| `GET /v1/vertices/{id}?includeDeleted=true` | Bearer key     | `admin` | Reads an active or soft-deleted vertex in the caller's tenant.                                                                  |
| `GET /v1/edges/{id}?includeDeleted=true`    | Bearer key     | `admin` | Reads an active or soft-deleted edge in the caller's tenant.                                                                    |
| `GET /v1/vertices?includeDeleted=true`      | Bearer key     | `admin` | Pages through active and soft-deleted vertices together; there is no deleted-only filter.                                       |
| `GET /v1/edges?includeDeleted=true`         | Bearer key     | `admin` | Pages through active and soft-deleted edges together; there is no deleted-only filter.                                          |
| `POST /v1/vertices/{id}/restore`            | Bearer key     | `admin` | Restores a soft-deleted vertex when the supplied `version` is current.                                                          |
| `POST /v1/edges/{id}/restore`               | Bearer key     | `admin` | Restores a soft-deleted edge when the supplied `version` is current and both endpoint vertices are active.                      |

All authenticated routes derive the tenant from the bearer key. The caller never supplies a tenant ID to select authorization context. An `admin` key includes `read` and `write` capabilities, but the CLI should invoke only the operator workflows in its approved scope.

## Bootstrap and tenant workflow

Drift v1 bootstrap is a server-local TypeScript command, not an HTTP endpoint:

```text
npm run cli -- bootstrap --slug <slug> --name <name> [--label <label>]
```

It constructs the SQLite repository directly, applies migrations, and calls `DriftService.bootstrap`. Each successful call:

1. rejects an already-used tenant slug;
2. creates exactly one tenant;
3. creates that tenant's first key with `admin` scope; and
4. prints the raw key secret once.

Bootstrap can be run again with a different unique slug to create another isolated tenant. There is no original, global administrative identity. Each initial admin key remains tied to the tenant created in the same bootstrap call, and no tenant's key can administer another tenant.

This differs from the kickoff's tentative description of an original admin tenant that creates later tenants. The Drift API and its current documentation are authoritative, so drift-cli must follow the per-tenant model.

## Recovery semantics

Drift does not define an archive resource or expose `/archive` routes. It defines soft-deleted vertices and edges plus admin-only restore operations.

- A restore body contains only `{ "version": <positive integer> }`.
- A successful mutation increments the record version.
- A stale version or wrong active/deleted state produces `409 Conflict`.
- Deleting a vertex also soft-deletes its active incident edges atomically.
- Restoring that vertex does not restore its incident edges.
- An edge can be restored only after both endpoint vertices are active.
- `includeDeleted=true` includes active and deleted records; it does not mean deleted-only.

The CLI may provide a recovery-oriented operator workflow, but its output and documentation must call the underlying records "soft-deleted" rather than inventing archive objects.

## Standard errors

Drift returns an envelope with `error.code`, `error.message`, and optional `error.details`. Clients should branch on status and code, not message text.

| Status | Meaning                                                               |
| ------ | --------------------------------------------------------------------- |
| `400`  | Invalid request or contract validation failure.                       |
| `401`  | Missing, malformed, invalid, or revoked bearer key.                   |
| `403`  | Authenticated key lacks the required scope.                           |
| `404`  | Resource is unavailable in the authenticated tenant.                  |
| `409`  | State or optimistic-concurrency conflict.                             |
| `422`  | A bounded operation exceeds server limits.                            |
| `500`  | Unexpected server failure; mutation outcome may require verification. |

Error rendering must never include the authorization header or a returned one-time secret.

## API gaps and ambiguities

### Tenant administration is absent

There are no HTTP routes to create, list, show, update, or delete tenants, and no endpoint that reveals the current tenant's slug or name. An independent HTTP client therefore cannot truthfully implement `tenant create`, `tenant list`, or `tenant show`.

A server-side proposal is required before those commands proceed. It must define an instance-level authorization model, because existing admin keys are intentionally tenant-scoped. Authorizing tenant routes with an ordinary tenant admin key would create a privilege escalation across the tenant/instance boundary. The [tenant administration trust boundary](../explanation/tenant-administration-boundary.md) records the threat model and minimum requirements.

drift-cli must not work around this gap by reading SQLite, invoking Drift internals, presenting the local bootstrap command as an HTTP capability, or treating one tenant as implicitly privileged over the others.

### Deleted-only listing is absent

The list routes support `includeDeleted=true`, which returns active and deleted records together. A CLI could page through everything and filter locally, but that changes cost and completeness expectations and can race concurrent mutations. A first release should support inspecting and restoring known IDs; deleted-only listing needs either an explicit documented client-side compromise or a Drift server filter.

### Contract compatibility metadata is limited

The OpenAPI document identifies API version `1.0.0`, but Drift exposes no separate instance metadata, server build version, or capability negotiation route. `drift status` can report health and contract version only when available; it must not imply full readiness or deployment version.
