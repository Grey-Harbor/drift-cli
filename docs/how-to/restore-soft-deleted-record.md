# Restore a soft-deleted record

Use recovery when you know the vertex or edge ID. Drift v1 has no deleted-only list, so drift-cli does not claim to discover every deleted record.

First inspect the record and note its current version:

```bash
drift recovery show vertex <vertex-id>
```

Restore that exact version:

```bash
drift recovery restore vertex <vertex-id> --version <version>
```

A `409` means the record version or state changed. Inspect it again before choosing another restore.

Deleting a vertex also soft-deletes its active incident edges. Restoring the vertex does not restore those edges. After both endpoint vertices are active, inspect and restore each required edge:

```bash
drift recovery show edge <edge-id>
drift recovery restore edge <edge-id> --version <version>
```

Recovery is always tenant-scoped by the admin bearer key.
