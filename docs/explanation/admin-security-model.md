# Administration security model

Use this explanation when deciding how an operator profile, bearer key, or command relates to a Drift tenant.

Drift authentication is tenant context, not merely permission. A raw key has a stored prefix and secret; Drift verifies it, then derives the key ID, tenant ID, and scopes. The client does not select a tenant in a header, path, or request body.

```text
raw bearer key
    -> Drift verifies prefix and secret hash
    -> Drift derives tenant ID and scopes
    -> the requested operation stays inside that tenant
```

An `admin` scope authorizes key management, soft-deleted reads, and restore for the same tenant. It also satisfies Drift's read and write checks, but drift-cli should expose only approved administration workflows.

`admin` means tenant administrator. It does not mean instance administrator and must never authorize tenant provisioning, enumeration, or cross-tenant operations. See the [tenant administration trust boundary](tenant-administration-boundary.md) for the threat model and requirements for any future instance-level API.

## Bootstrap identities

The current server-local bootstrap command creates a new tenant and that tenant's first admin key together. Running bootstrap with a second unique slug creates a second tenant with a different admin key. Neither key is global and neither can administer the other tenant.

Operators must retain each one-time bootstrap secret or immediately create a managed replacement. Drift stores only a secret hash, so neither the server nor drift-cli can reveal a lost key.

## Key lifecycle

- Listing returns metadata, never secrets.
- Creation returns one new secret once.
- Revocation is immediate and irreversible.
- Rotation immediately revokes the selected key and then returns one replacement secret. Drift provides no grace period or dual-validity window.
- Every created or rotated key remains bound to the admin caller's tenant.

The CLI must not infer scopes, silently add `admin`, retry a rotation after an uncertain server failure, or claim that a secret was safely deployed. Those remain operator decisions.

## Recovery authority

Only admin keys can request deleted records or restore them. Version checks prevent stale recovery from overwriting a newer state. Edge recovery additionally requires both endpoint vertices to be active. The CLI should explain those constraints but leave enforcement to Drift.
