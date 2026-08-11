# Configuration and credential handling

Use this design when implementing profile resolution or deciding how automation supplies a Drift admin key.

The configuration file stores non-secret operator preferences. The first release will not persist raw Drift credentials.

## Configuration

Platform-standard configuration directories come from the Rust `directories` crate. An explicit `--config` path and `DRIFT_CONFIG` environment variable make automation and tests deterministic.

```toml
default_profile = "local"
output = "human"

[profiles.local]
endpoint = "http://localhost:3000"
credential_env = "DRIFT_LOCAL_ADMIN_KEY"
```

`credential_env` names an environment variable; it is not the credential itself. Profiles label an endpoint and credential source for operator convenience, but they do not select a tenant on the server. The supplied key still determines the tenant.

## Resolution precedence

Non-secret values resolve in this order:

1. explicit command-line option;
2. direct environment override (`DRIFT_ENDPOINT`, `DRIFT_OUTPUT`, or `DRIFT_PROFILE`);
3. selected profile;
4. top-level config value;
5. safe default where one exists.

The endpoint default is `http://localhost:3000`, and output defaults to `human`. Configuration errors fail before any HTTP request and identify the source without printing secret values.

Credential resolution is intentionally narrower:

1. standard input when `--key-stdin` is explicitly selected;
2. `DRIFT_API_KEY` when set;
3. the environment variable named by the selected profile's `credential_env`;
4. otherwise fail with guidance.

There is no secret-bearing command-line option and no plaintext `api_key` configuration field. Standard input trims one trailing line ending but must not otherwise rewrite a key.

## Future credential stores

Native OS keychain support may be proposed after the environment/stdin workflow is validated. It must be opt-in, identify portability and headless-server behavior, define deletion and profile-renaming semantics, and avoid silently falling back to plaintext files.

## Secret-safe implementation rules

- Wrap credentials with `secrecy::SecretString` and expose them only at the authorization-header boundary.
- Do not derive or implement `Debug`, `Display`, serialization, cloning, or equality for application secret wrappers unless required and reviewed.
- Redact the `Authorization` header from HTTP traces and error context.
- Parse error bodies into a DTO that does not retain the outgoing request headers.
- One-time secrets from create/rotate may appear in successful human or JSON output. They must not appear in errors, retries, telemetry, or test failure diffs.
- Do not automatically retry non-idempotent create, revoke, rotate, or restore operations.
