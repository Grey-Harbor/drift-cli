# Drift CLI documentation

Use these documents to configure and operate drift-cli or understand the Drift contract behind it.

## Current design records

- [Architecture](explanation/architecture.md)
- [Administration security model](explanation/admin-security-model.md)
- [Tenant administration trust boundary](explanation/tenant-administration-boundary.md)
- [Configuration and credentials](explanation/configuration-and-credentials.md)
- [Drift API inventory](reference/drift-api-inventory.md)
- [Initial command surface](reference/initial-command-surface.md)

## Operator documentation

- [Getting started](tutorial/getting-started.md)
- [Configure admin access](how-to/configure-admin-access.md)
- [Create a tenant key](how-to/create-tenant-key.md)
- [Revoke a key](how-to/revoke-key.md)
- [Restore a soft-deleted record](how-to/restore-soft-deleted-record.md)
- [Use JSON output](how-to/use-json-output.md)
- [Command reference](reference/commands.md)
- [Configuration reference](reference/configuration.md)
- [Environment reference](reference/environment.md)
- [Exit codes](reference/exit-codes.md)

## Documentation boundaries

There is deliberately no tenant-creation guide. Drift v1 requires server-local bootstrap and exposes no compatible HTTP endpoint. Authorizing tenant provisioning with an ordinary tenant admin key would violate the documented [tenant administration trust boundary](explanation/tenant-administration-boundary.md). Recovery documentation covers known IDs only because Drift has no deleted-only listing filter.
