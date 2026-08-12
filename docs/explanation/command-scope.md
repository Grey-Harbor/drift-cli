# Why Drift CLI has a narrow command surface

Use this explanation when deciding whether an operator workflow belongs in Drift CLI. The CLI exposes deliberate administration and recovery commands rather than every operation available in Drift.

## The HTTP contract is the boundary

Drift owns authorization, tenancy, validation, and persistence. Drift CLI is an independent HTTP client, so every command must map to a released Drift route without importing server code or accessing storage directly.

This boundary keeps the command line useful without creating a second implementation of Drift behavior. When the server lacks a capability, the CLI documents the gap instead of bypassing it.

## Commands express operator intent

A focused command such as `drift key rotate` communicates an administrative outcome and can make its risks explicit. A raw HTTP passthrough would expose transport details, weaken discoverability, and turn the CLI into an unrestricted API shell.

The implemented surface is therefore limited to:

| Operator intent             | Drift capability                                      |
| --------------------------- | ----------------------------------------------------- |
| Check process and API state | Health and OpenAPI discovery                          |
| Manage tenant keys          | Tenant-scoped admin key routes                        |
| Inspect a deleted record    | Known-ID reads with `includeDeleted=true`             |
| Restore a deleted record    | Versioned vertex and edge restore routes              |

Generic vertex creation, mutation, traversal, and retrieval remain application concerns rather than administration workflows.

## Tenant commands remain unavailable

Drift v1 has no instance-level HTTP authority for creating or enumerating tenants. Its `admin` scope belongs to one tenant and cannot safely authorize access to the tenant registry.

Adding tenant commands to the client first would either advertise operations that cannot work or encourage an authorization bypass. Drift must define and release a separate instance-operator contract before the CLI can add that surface. The [tenant administration trust boundary](tenant-administration-boundary.md) explains the security consequences.

## Recovery is limited to known IDs

Drift can include deleted records in ordinary paginated lists, but it cannot filter those lists to deleted records only. A client-side scan would have unclear cost and completeness and could race concurrent changes.

Drift CLI therefore supports inspection and restore when the operator already knows the record ID. A recovery-list command remains deferred until Drift exposes an appropriate filter or a separate design explicitly accepts the client-side tradeoffs.

## Consequences for future commands

A proposed command belongs in Drift CLI only when:

1. a released Drift HTTP route supports the complete operation;
2. the route's authorization matches the intended operator authority;
3. the command can preserve Drift terminology and concurrency rules;
4. errors and one-time secrets can be handled without weakening security; and
5. the workflow is administrative rather than ordinary application-data authoring.

Use the [command reference](../reference/commands.md) for the exact implemented interface and the [Drift API inventory](../reference/drift-api-inventory.md) for the upstream routes behind it.
