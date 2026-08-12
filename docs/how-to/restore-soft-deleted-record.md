# Restore a soft-deleted record

Use recovery when you know the vertex or edge ID. Drift v1 has no deleted-only list, so drift-cli does not claim to discover every deleted record.

## Prerequisites

- A bearer key with `admin` scope for the record's tenant.
- The known vertex or edge ID.
- Operator approval to restore the record and accept its current relationships.

## Inspect the record

First inspect the record and note its current version:

```bash
drift recovery show vertex <vertex-id>
```

Confirm that the output identifies the record as soft-deleted. An active record should not be restored.

## Restore the current version

Restore that exact version:

```bash
drift recovery restore vertex <vertex-id> --version <version>
```

A `409` means the record version or state changed. Inspect it again before choosing another restore.

## Restore dependent edges deliberately

Deleting a vertex also soft-deletes its active incident edges. Restoring the vertex does not restore those edges. After both endpoint vertices are active, inspect and restore each required edge:

```bash
drift recovery show edge <edge-id>
drift recovery restore edge <edge-id> --version <version>
```

Recovery is always tenant-scoped by the admin bearer key.

## Verify the result

Run `drift recovery show` for each restored ID and confirm that it is active with an incremented version. Verify application behavior separately; restoring a record does not prove that every formerly connected edge was restored.

The CLI never retries restore automatically. After a timeout or connection failure, inspect the record again before issuing another mutation.
