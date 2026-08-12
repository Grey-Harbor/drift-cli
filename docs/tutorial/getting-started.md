# Tutorial: administer an existing Drift tenant

Use this tutorial after a Drift operator has bootstrapped a tenant and securely provided its admin key. You will build drift-cli, verify the endpoint, inspect keys, and issue a read-only key.

Drift v1 bootstrap remains a server-local operation. drift-cli cannot create or select tenants; the supplied key determines the tenant for every authenticated request.

## What you will complete

By the end, a locally built `drift` binary will have discovered the target API contract, authenticated to one tenant, listed its key metadata, and created a read-only service key.

## Prerequisites

- Rust 1.85 or newer.
- A running Drift v1 endpoint.
- A tenant admin key supplied through an approved secret channel.
- A secure destination for the one-time service-key secret created in the final step.

## 1. Build the CLI

From this repository with Rust 1.85 or newer:

```bash
cargo build --release
export PATH="$PWD/target/release:$PATH"
drift --version
```

## 2. Configure the endpoint and admin key

```bash
export DRIFT_ENDPOINT='http://localhost:3000'
export DRIFT_API_KEY='drift_<admin-prefix>.<admin-secret>'
```

The example key is a placeholder. Avoid entering real keys directly in shell commands, where they can enter command history.

## 3. Check the instance contract

```bash
drift status
```

Expected shape:

```text
Drift is healthy

Endpoint: http://localhost:3000
API:      Drift API 1.0.0
```

This confirms HTTP health and contract discovery, not backup or storage health.

## 4. Inspect the tenant's keys

```bash
drift key list
```

The admin key's tenant is the only tenant visible. List output contains metadata, not recoverable secrets.

## 5. Create a read-only key

```bash
drift key create --label reporting --scope read
```

Save the printed secret immediately in the reporting service's secret store. Drift returns it once only. The new key is tenant-bound and cannot administer keys because it lacks `admin` scope.

## Verify the outcome

Run `drift key list` again and confirm that the new key metadata has the `reporting` label and only the `read` scope. Do not expect the secret to appear in list output.

You have completed the tutorial when the key is stored securely and its metadata is visible in the same tenant. Continue with the [how-to guides](../how-to/README.md), use the [command reference](../reference/commands.md) for exact syntax, or follow the [recovery guide](../how-to/restore-soft-deleted-record.md).
