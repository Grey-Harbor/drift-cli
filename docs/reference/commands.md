# Command reference

Use this reference for the exact drift-cli command interface. Global options may appear before or after a subcommand.

## Global options

```text
--profile <name>   Select a named configuration profile.
--config <path>    Read a specific TOML configuration file.
--endpoint <url>   Override the Drift base URL.
--json             Emit the versioned JSON envelope.
--key-stdin        Read the bearer key from standard input.
```

There is no secret-bearing command-line option. See [configuration](configuration.md) and [environment variables](environment.md) for resolution rules.

## Status

```text
drift status
```

Requests `GET /health` and `GET /v1/openapi.json`. Success means Drift reports `status: ok` and publishes non-empty OpenAPI title and version fields. It does not prove backup freshness, storage capacity, or graph correctness.

## Key administration

```text
drift key list
drift key create --label <label> --scope <read|write|admin> [--scope ...]
drift key revoke <id> --yes
drift key rotate <id> --label <label> --scope <scope> [--scope ...] --yes
```

All key commands operate inside the tenant derived from the bearer key.

- `list` returns key metadata and never a raw secret.
- `create` requires an explicit label and at least one scope. It prints the new secret once.
- `revoke` is immediate and irreversible; `--yes` is mandatory.
- `rotate` immediately revokes the old key and returns its replacement secret once; `--yes` is mandatory.

The CLI never automatically retries these mutations.

## Recovery

```text
drift recovery show <vertex|edge> <id>
drift recovery restore <vertex|edge> <id> --version <positive-integer>
```

`show` requests the known record with `includeDeleted=true` and reports whether it is active or soft-deleted. `restore` sends the supplied optimistic-concurrency version.

Restoring a vertex does not restore its incident edges. An edge can be restored only after both endpoint vertices are active.

There is no recovery-list command because Drift v1 cannot filter its paginated list to deleted records only.

## Task guides

Use the how-to guides for procedures and verification:

- [Create a tenant key](../how-to/create-tenant-key.md)
- [Revoke a tenant key](../how-to/revoke-key.md)
- [Rotate a tenant key](../how-to/rotate-key.md)
- [Restore a soft-deleted record](../how-to/restore-soft-deleted-record.md)
- [Use JSON output in automation](../how-to/use-json-output.md)
