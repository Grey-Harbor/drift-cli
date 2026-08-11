# Configuration reference

Use a TOML configuration file to name endpoints and credential environment variables without storing raw keys.

```toml
default_profile = "local"
output = "human"

[profiles.local]
endpoint = "http://localhost:3000"
credential_env = "DRIFT_LOCAL_ADMIN_KEY"
```

Top-level fields:

| Field             | Values            | Purpose                                                                |
| ----------------- | ----------------- | ---------------------------------------------------------------------- |
| `default_profile` | Profile name      | Selects a profile when neither `--profile` nor `DRIFT_PROFILE` is set. |
| `endpoint`        | HTTP or HTTPS URL | Fallback endpoint when no selected profile supplies one.               |
| `output`          | `human` or `json` | Default output mode.                                                   |
| `profiles`        | TOML table        | Named endpoint and credential-source definitions.                      |

Profile fields:

| Field            | Required | Purpose                                                               |
| ---------------- | -------- | --------------------------------------------------------------------- |
| `endpoint`       | Yes      | Drift base URL for this profile.                                      |
| `credential_env` | No       | Name of the environment variable containing this tenant's bearer key. |

Unknown fields, missing explicitly selected profiles, malformed URLs, URL credentials, URL queries, and URL fragments are errors. A raw `api_key` field is not supported.

The default file is `config.toml` in the operating system's standard drift-cli configuration directory. Use `--config` or `DRIFT_CONFIG` when a deterministic path matters.

Non-secret values resolve in this order:

1. command-line option;
2. direct environment override;
3. selected profile;
4. top-level configuration;
5. `http://localhost:3000` and human output defaults.

Credential resolution is documented in the [environment reference](environment.md).
