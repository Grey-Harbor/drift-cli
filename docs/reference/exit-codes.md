# Exit-code reference

Use exit codes for coarse automation decisions and JSON `error.kind`, `error.httpStatus`, and `error.code` for detail.

| Code | Meaning                                                          |
| ---- | ---------------------------------------------------------------- |
| `0`  | Command succeeded.                                               |
| `2`  | CLI usage or configuration error.                                |
| `3`  | Missing/invalid credential or Drift HTTP 401.                    |
| `4`  | Drift HTTP 403 forbidden.                                        |
| `5`  | Drift HTTP 404 not found in the authenticated tenant.            |
| `6`  | Drift HTTP 409 state or version conflict.                        |
| `7`  | Transport, DNS, TLS, connection, or timeout failure.             |
| `8`  | Other Drift API, response-contract, output, or internal failure. |

The CLI does not automatically retry mutations after any error.
