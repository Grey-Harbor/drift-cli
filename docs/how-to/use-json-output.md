# Use JSON output

Add `--json` to any command or set `DRIFT_OUTPUT=json`:

```bash
drift --json key list
```

Success uses this stable outer envelope:

```json
{
  "schemaVersion": 1,
  "command": "key.list",
  "data": {
    "keys": []
  }
}
```

Runtime errors are written to standard error:

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

Create and rotate success responses intentionally include the one-time secret at `data.secret`. Other commands and all errors exclude credentials.
