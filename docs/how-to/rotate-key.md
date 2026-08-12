# Rotate a tenant key

Use this guide when a Drift credential needs a replacement. Rotation revokes the old key immediately and has no overlap period, so plan how the new one-time secret will reach its consumer.

## Prerequisites

- A bearer key with `admin` scope for the same tenant as the target key.
- The target key ID.
- An explicit label and complete scope set for the replacement.
- A secure destination for the new secret.

## Rotate the key

```bash
drift key rotate <key-id> \
  --label inventory-service \
  --scope read \
  --scope write \
  --yes
```

Drift revokes the old key before returning the replacement. Store the printed secret immediately; it cannot be retrieved later.

## Deploy and verify the replacement

Update the consuming service through its approved secret-delivery process, then verify that it can authenticate and perform only the intended operations.

List key metadata to confirm the old key is revoked and the replacement has the expected label and scopes:

```bash
drift key list
```

## Handle an uncertain result

Do not repeat rotation automatically after a timeout or connection failure. Inspect key metadata first to determine whether Drift created the replacement. An unobserved one-time secret may require revoking that replacement and performing a new, deliberate rotation.

Use [revocation](revoke-key.md) when no replacement should be issued.
