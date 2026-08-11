# Initial command surface

Use this reference to understand the scope boundary behind the implemented first command set. Every available command is backed by an existing Drift HTTP route; unavailable commands remain marked blocked.

## Global interface

```text
drift [--profile <name>] [--config <path>] [--endpoint <url>] [--json] <command>
```

`--key-stdin` supplies credentials to authenticated commands. A secret-valued `--api-key` option is deliberately excluded because command lines commonly leak through shell history and process inspection.

## Commands supported by Drift v1

| Proposed command                                                 | Drift route                                      | Notes                                                                                          |
| ---------------------------------------------------------------- | ------------------------------------------------ | ---------------------------------------------------------------------------------------------- |
| `drift status`                                                   | `GET /health`; optionally `GET /v1/openapi.json` | Human output distinguishes process health from contract discovery. No credential is required.  |
| `drift key list`                                                 | `GET /v1/admin/keys`                             | Displays metadata only. JSON output mirrors a stable CLI response, not terminal decoration.    |
| `drift key create --label <label> --scope <scope>...`            | `POST /v1/admin/keys`                            | Requires at least one explicit scope. Shows the new secret once.                               |
| `drift key revoke <id> --yes`                                    | `DELETE /v1/admin/keys/{id}`                     | Immediate and irreversible. The explicit acknowledgement is required.                          |
| `drift key rotate <id> --label <label> --scope <scope>... --yes` | `POST /v1/admin/keys/{id}/rotate`                | Revokes before issuing the replacement; there is no overlap period. Shows the new secret once. |
| `drift recovery show vertex <id>`                                | `GET /v1/vertices/{id}?includeDeleted=true`      | Reports whether the record is active or soft-deleted and its current version.                  |
| `drift recovery show edge <id>`                                  | `GET /v1/edges/{id}?includeDeleted=true`         | Reports state, version, and endpoint IDs.                                                      |
| `drift recovery restore vertex <id> --version <version>`         | `POST /v1/vertices/{id}/restore`                 | Requires the current deleted-record version.                                                   |
| `drift recovery restore edge <id> --version <version>`           | `POST /v1/edges/{id}/restore`                    | Requires active endpoint vertices and the current version.                                     |

The `recovery` grouping communicates an operator-only workflow without suggesting that Drift has an archive resource. Output uses Drift's "soft-deleted" and "restore" terms.

## Deferred or blocked commands

| Requested command                                                | Status       | Reason                                                                                                    |
| ---------------------------------------------------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `drift tenant create <name>`                                     | Blocked      | Drift v1 has no tenant/bootstrap HTTP endpoint or instance-admin credential.                              |
| `drift tenant list`                                              | Blocked      | Drift v1 exposes no tenant collection route.                                                              |
| `drift tenant show <tenant>`                                     | Blocked      | No route returns tenant slug/name/status; a key reveals only tenant IDs in key metadata.                  |
| `drift recovery list ...`                                        | Deferred     | `includeDeleted=true` mixes active and deleted records. Deleted-only, race-safe listing is not supported. |
| Generic vertex/edge create, mutate, query, or traversal commands | Out of scope | They represent normal application behavior, not Drift administration.                                     |
| Raw arbitrary HTTP command                                       | Out of scope | It exposes transport rather than operator intent and would expand the CLI into a generic API shell.       |

## Output rules

- Human output is the default; `--json` is available globally from the first executable release.
- CLI-owned JSON envelopes will be versioned and snapshot-tested so scripts do not depend on decorative output or unstable server messages.
- Create and rotate are the only normal commands that print raw secrets, because the server returns them once and they cannot be recovered later.
- Errors include the operation, HTTP status when present, Drift error code, and safe remediation. They exclude request authorization and response secret fields.
- Cursors remain opaque. Commands that page must never decode or modify them.
