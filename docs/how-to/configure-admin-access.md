# Configure admin access

Use a named profile when one operator works with multiple Drift tenants or endpoints. Profiles store no raw secrets.

Create a TOML file at a controlled path:

```toml
default_profile = "acme"
output = "human"

[profiles.acme]
endpoint = "https://drift.example.com"
credential_env = "ACME_DRIFT_ADMIN_KEY"
```

Export the referenced variable, then select the file:

```bash
export ACME_DRIFT_ADMIN_KEY='drift_<prefix>.<secret>'
export DRIFT_CONFIG="$PWD/drift-cli.toml"

drift status
drift key list
```

`status` is unauthenticated; `key list` verifies the credential and its admin scope. The bearer key—not the profile name—determines the tenant.

For short-lived pipelines, avoid a long-lived environment variable:

```bash
secret-command | drift --key-stdin key list
```

Standard input takes precedence over environment credential sources.
