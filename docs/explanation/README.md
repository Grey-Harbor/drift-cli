# Explanation

Use explanation when you need to understand why Drift CLI has a constraint or boundary before making a design or operational decision. These pages describe context, tradeoffs, and consequences rather than procedures.

- [Client architecture](architecture.md) — why parsing, transport, DTOs, workflows, and output remain separate.
- [Configuration and credentials](configuration-and-credentials.md) — why profiles remain non-secret and bearer keys establish tenant context.
- [Administration security model](admin-security-model.md) — how tenant-scoped keys authorize administrative workflows.
- [Tenant administration trust boundary](tenant-administration-boundary.md) — why tenant administration is not instance authority.
- [Command scope](command-scope.md) — why the CLI exposes a narrow operator surface instead of a generic Drift API shell.
