# Revoke or rotate a key

Use revocation when a credential must stop working permanently:

```bash
drift key revoke <key-id> --yes
```

`--yes` is required because revocation is immediate and cannot be undone.

Use rotation when Drift should revoke one key and issue a replacement:

```bash
drift key rotate <key-id> \
  --label inventory-service \
  --scope read \
  --scope write \
  --yes
```

Rotation has no overlap period: the old secret stops working before the command returns the replacement. Store and deploy the new secret immediately. If the request fails after it was sent, inspect key metadata before deciding whether to retry; the CLI never retries this mutation automatically.
