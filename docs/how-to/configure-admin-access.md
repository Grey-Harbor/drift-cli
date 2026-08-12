# Configure admin access

Use a named profile when one operator works with multiple Drift tenants or endpoints. Profiles store no raw secrets.

## Prerequisites

- A running Drift endpoint.
- A tenant admin key supplied through an approved secret channel.
- A local path whose permissions are appropriate for non-secret operator configuration.

## Create the profile

Create a TOML file at a controlled path:

```toml
default_profile = "acme"
output = "human"

[profiles.acme]
endpoint = "https://drift.example.com"
credential_env = "ACME_DRIFT_ADMIN_KEY"
```

The file contains the credential variable's name, not its value. The [configuration reference](../reference/configuration.md) defines every accepted field.

## Supply the credential

Export the referenced variable, then select the file:

```bash
export ACME_DRIFT_ADMIN_KEY='drift_<prefix>.<secret>'
export DRIFT_CONFIG="$PWD/drift-cli.toml"

drift status
drift key list
```

`status` is unauthenticated; `key list` verifies the credential and its admin scope. The bearer key—not the profile name—determines the tenant.

## Use standard input for a short-lived command

For short-lived pipelines, avoid a long-lived environment variable:

```bash
secret-command | drift --key-stdin key list
```

Standard input takes precedence over environment credential sources.

## Verify and troubleshoot

Successful `drift key list` output confirms that the endpoint, credential source, tenant context, and admin scope work together. A missing or empty variable is a configuration error; a `401` indicates an invalid or revoked key; a `403` indicates insufficient scope.

Do not work around a failure by placing the bearer key directly in the TOML file or a command argument. Use the [environment reference](../reference/environment.md) to review exact resolution order.
