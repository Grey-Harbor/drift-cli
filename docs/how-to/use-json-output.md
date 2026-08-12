# Use JSON output in automation

Use this guide when a script or CI job needs stable Drift CLI output. JSON mode separates machine-readable success and failure data from human presentation.

## Select JSON output

Add `--json` to one command:

```bash
drift --json key list
```

Set `DRIFT_OUTPUT=json` when every command in the process should use JSON:

```bash
export DRIFT_OUTPUT=json
drift key list
```

An explicit `--json` option takes precedence over configuration. The [JSON output reference](../reference/json-output.md) defines the complete envelope.

## Parse a success result

Check the process exit code before consuming `data`. For example, with `jq`:

```bash
if result="$(drift --json key list)"; then
  printf '%s\n' "$result" | jq -r '.data.keys[].id'
else
  exit $?
fi
```

Create and rotate results include a one-time secret at `data.secret`. Send those values directly to the approved secret destination and avoid logging the captured JSON.

## Handle errors

Runtime error JSON is written to standard error and the process exits nonzero. Branch on the exit code and stable `error.kind` or `error.code`, not on human message text.

The [exit-code reference](../reference/exit-codes.md) defines coarse process outcomes. The CLI excludes bearer credentials and one-time response secrets from error envelopes.

## Verify automation behavior

Test both a successful request and an expected failure before deploying a script. Confirm that logs capture neither `DRIFT_API_KEY` nor `data.secret` and that the script does not retry mutations automatically.
