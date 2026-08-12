# Create a tenant key

Use an existing admin key to issue a narrowly scoped credential inside the same tenant.

## Prerequisites

- A bearer key with `admin` scope for the target tenant.
- A unique, descriptive label for the consumer.
- An operator-approved set of `read`, `write`, or `admin` scopes.
- A secure destination for the one-time secret.

## Create the key

```bash
drift key create \
  --label inventory-service \
  --scope read \
  --scope write
```

The command prints the secret once. Move it directly into the client service's secret store and avoid terminal capture or shared logs.

## Create a key from automation

For automation:

```bash
drift --json key create \
  --label reporting \
  --scope read
```

Read the new metadata from `data.apiKey` and the one-time secret from `data.secret`. The CLI never infers or broadens scopes. The [JSON output reference](../reference/json-output.md) defines the surrounding envelope.

## Verify the result

```bash
drift key list
```

Confirm that the new key metadata has the requested label and exact scope set. List output never repeats the secret.

If the create request has an uncertain outcome, inspect key metadata before retrying. Repeating a successful request creates another credential.
