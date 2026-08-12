<p align="center">
  <a href="https://drift-cli.greyharborsoftware.com">
    <img src="site/public/drift-cli-mark.svg" width="120" height="120" alt="Drift CLI" />
  </a>
</p>

# Drift CLI

Drift CLI is a Rust command-line administration client for [Drift](https://github.com/Grey-Harbor/drift). It wraps Drift's existing HTTP contracts so self-hosters, operators, and automation can perform privileged workflows without hand-crafting JSON and `curl` commands.

It is not a graph editor, application-data authoring interface, persistence engine, or shortcut around Drift authorization and tenant isolation.

Read the [Drift CLI documentation](https://drift-cli.greyharborsoftware.com/docs/) or start with the [operator tutorial](https://drift-cli.greyharborsoftware.com/docs/tutorial/).

## Status

The tenant-scoped administration and recovery commands are implemented. The project has not published precompiled release binaries yet.

The supported surface includes:

```text
drift status
drift key list
drift key create --label reporting --scope read
drift key revoke <key-id> --yes
drift key rotate <key-id> --label reporting --scope read --yes
drift recovery show vertex <vertex-id>
drift recovery restore vertex <vertex-id> --version <version>
```

Drift manages tenants as authorization boundaries, while this CLI administers keys and recovery within the tenant identified by the supplied admin key. Drift v1 intentionally exposes no HTTP authority for tenant provisioning or enumeration; treating an ordinary tenant admin key as instance-wide authority would break tenant isolation. See the [tenant administration trust boundary](docs/explanation/tenant-administration-boundary.md).

## Build and run

Rust 1.85 or newer is required to build from source:

```bash
cargo build --release
./target/release/drift --help
```

Point the CLI at Drift and provide the tenant's admin key through the environment:

```bash
export DRIFT_ENDPOINT='http://localhost:3000'
export DRIFT_API_KEY='drift_<prefix>.<secret>'

cargo run -- status
cargo run -- key list
```

Use `--key-stdin` when an environment variable is inappropriate. The CLI never stores raw credentials in its configuration file.

## Documentation

- [Drift CLI website](https://drift-cli.greyharborsoftware.com)
- [Kickoff findings and plan](PLAN.md)
- [Drift API inventory](docs/reference/drift-api-inventory.md)
- [Initial command surface](docs/reference/initial-command-surface.md)
- [Client architecture and dependencies](docs/explanation/architecture.md)
- [Administration security model](docs/explanation/admin-security-model.md)
- [Tenant administration trust boundary](docs/explanation/tenant-administration-boundary.md)
- [Configuration and credential handling](docs/explanation/configuration-and-credentials.md)
- [Command reference](docs/reference/commands.md)
- [Configuration reference](docs/reference/configuration.md)
- [Exit codes](docs/reference/exit-codes.md)

Start with the [operator tutorial](docs/tutorial/getting-started.md).

## License

drift-cli is licensed under the [Apache License 2.0](LICENSE).
