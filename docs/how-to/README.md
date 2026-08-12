# How-to guides

Use a how-to guide when you already understand the Drift CLI basics and need to complete a specific operator task. Each guide identifies prerequisites, the procedure, verification, and recovery where the operation can leave uncertain state.

- [Configure admin access](configure-admin-access.md) — select an endpoint and supply a tenant admin key without storing the secret in configuration.
- [Create a tenant key](create-tenant-key.md) — issue a narrowly scoped service credential and handle its one-time secret.
- [Revoke a tenant key](revoke-key.md) — stop a credential permanently and verify the result.
- [Rotate a tenant key](rotate-key.md) — replace a credential when there is no overlap period.
- [Restore a soft-deleted record](restore-soft-deleted-record.md) — inspect and restore a known vertex or edge with its current version.
- [Use JSON output in automation](use-json-output.md) — consume stable output without leaking credentials.
