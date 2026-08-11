# Create a tenant key

Use an existing admin key to issue a narrowly scoped credential inside the same tenant.

```bash
drift key create \
  --label inventory-service \
  --scope read \
  --scope write
```

The command prints the secret once. Move it directly into the client service's secret store and avoid terminal capture or shared logs.

For automation:

```bash
drift --json key create \
  --label reporting \
  --scope read
```

Read the new metadata from `data.apiKey` and the one-time secret from `data.secret`. The CLI never infers or broadens scopes.
