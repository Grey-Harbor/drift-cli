# Environment reference

Use these variables for automation and credential injection.

| Variable         | Purpose                                |
| ---------------- | -------------------------------------- |
| `DRIFT_CONFIG`   | Explicit configuration file path.      |
| `DRIFT_PROFILE`  | Selected configuration profile.        |
| `DRIFT_ENDPOINT` | Drift base URL override.               |
| `DRIFT_OUTPUT`   | `human` or `json`.                     |
| `DRIFT_API_KEY`  | Bearer key for authenticated commands. |

An authenticated command resolves its credential in this order:

1. standard input when `--key-stdin` is present;
2. `DRIFT_API_KEY`;
3. the variable named by the selected profile's `credential_env`.

Empty variables are errors rather than silent fallbacks. `status` requires no credential.
