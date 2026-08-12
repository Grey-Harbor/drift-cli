# JSON output reference

Use this reference when implementing a parser for Drift CLI output. JSON mode uses a versioned success envelope on standard output and a versioned error envelope on standard error.

## Selection and destination

JSON mode is selected by `--json`, `DRIFT_OUTPUT=json`, or a configuration value of `json`. The command-line flag has the highest precedence.

| Outcome       | Destination     | Exit status |
| ------------- | --------------- | ----------- |
| Success       | Standard output | `0`         |
| Runtime error | Standard error  | Nonzero     |
| Parse error   | Standard error  | `2`         |

## Success envelope

```json
{
  "schemaVersion": 1,
  "command": "key.list",
  "data": {
    "keys": []
  }
}
```

| Field           | Type   | Contract                                                |
| --------------- | ------ | ------------------------------------------------------- |
| `schemaVersion` | number | Currently `1`; consumers must reject unknown versions. |
| `command`       | string | Stable identifier for the completed command.           |
| `data`          | object | Command-specific success payload.                       |

Create and rotate success payloads include the returned one-time secret at `data.secret`. Other success payloads do not expose credentials.

## Error envelope

```json
{
  "schemaVersion": 1,
  "error": {
    "kind": "api",
    "message": "Drift returned HTTP 403: Admin scope required",
    "httpStatus": 403,
    "code": "forbidden"
  }
}
```

| Field              | Required | Contract                                                    |
| ------------------ | -------- | ----------------------------------------------------------- |
| `schemaVersion`    | Yes      | Currently `1`.                                              |
| `error.kind`       | Yes      | Stable coarse failure category.                             |
| `error.message`    | Yes      | Safe diagnostic text; not a control-flow contract.          |
| `error.httpStatus` | No       | Drift HTTP status when the failure has a server response.   |
| `error.code`       | No       | Drift error code when the response supplies one.            |

Error output never includes the authorization header, bearer key, or a one-time secret returned by a successful mutation.

## Compatibility rules

- Consumers should branch on `schemaVersion`, `command`, exit status, `error.kind`, and `error.code`.
- Consumers must not parse human output or depend on `error.message` wording.
- New optional fields may appear within schema version 1.
- Removing or changing the meaning of an existing field requires a new schema version.
- Cursors and server-owned identifiers are opaque strings.

See [Use JSON output in automation](../how-to/use-json-output.md) for a task-oriented example.
