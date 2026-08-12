# Revoke a tenant key

Use this guide when a credential must stop working permanently. Revocation is immediate and irreversible, so identify the key by metadata before acknowledging the mutation.

## Prerequisites

- A bearer key with `admin` scope for the same tenant as the target key.
- The target key ID. Labels are descriptive and are not accepted in place of IDs.

List the tenant's key metadata if you need to confirm the ID:

```bash
drift key list
```

## Revoke the key

```bash
drift key revoke <key-id> --yes
```

`--yes` is mandatory because Drift cannot undo revocation. The command does not print or recover the revoked secret.

## Verify the result

List key metadata again and confirm that the target key is recorded as revoked:

```bash
drift key list
```

Also verify that services which used the credential have stopped sending requests or have switched to an approved replacement.

## Handle an uncertain result

The CLI never retries revocation automatically. If the request fails after transmission or the connection closes before a response, inspect key metadata before issuing another mutation.

Use [rotation](rotate-key.md) instead when Drift should create a replacement credential as part of the operation.
