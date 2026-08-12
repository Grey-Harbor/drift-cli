# Drift CLI documentation

Use this index to choose documentation by the kind of help you need. Drift CLI follows Diátaxis so guided learning, operator tasks, exact contracts, and design context remain separate.

- [Tutorials](tutorial/README.md) teach a complete first workflow, beginning with an existing bootstrapped Drift tenant.
- [How-to guides](how-to/README.md) solve focused tasks such as configuring access, managing keys, restoring records, and using JSON in automation.
- [Reference](reference/README.md) defines commands, configuration, environment variables, JSON envelopes, exit codes, and the upstream Drift API surface.
- [Explanation](explanation/README.md) describes the architecture, security model, tenant boundary, credential design, and intentionally narrow command scope.
- [Documentation style](STYLE.md) defines how contributors classify, write, and review these pages.

## Product boundaries

There is deliberately no tenant-creation guide because Drift v1 exposes no compatible instance-level HTTP authority. Recovery guidance covers known IDs only because Drift has no deleted-only listing filter.

These are server capability boundaries, not missing client shortcuts. See [Why Drift CLI has a narrow command surface](explanation/command-scope.md) and the [tenant administration trust boundary](explanation/tenant-administration-boundary.md).
