# Documentation style and review guide

Use this guide whenever you add or change Drift CLI documentation. It keeps learning paths, task procedures, exact contracts, and design context separate while preserving Drift terminology and security boundaries.

## Choose one Diátaxis purpose

Put each page in the section that matches the reader's immediate need:

| Section        | Reader need                            | Page shape                                        |
| -------------- | -------------------------------------- | ------------------------------------------------- |
| `tutorial/`    | Learn by completing a guided path.     | Ordered steps with a known start and outcome.     |
| `how-to/`      | Accomplish a specific operator task.   | Prerequisites, procedure, verification, recovery. |
| `reference/`   | Look up an exact contract.             | Inputs, outputs, defaults, failures, limitations. |
| `explanation/` | Understand a design choice.            | Context, constraints, tradeoffs, consequences.    |

Do not make one page serve multiple purposes. Link to the canonical page in another section when a task needs exact definitions or design context.

## Open with use and intent

The first paragraph must say when the reader should use the page and why the result matters. Avoid openings that only restate the title or describe the document.

## Write operator-safe procedures

How-to guides must identify prerequisites, provide complete commands, explain how to verify the outcome, and describe recovery when a mutation can leave uncertain state.

Use placeholders that cannot be mistaken for real credentials. Never place a bearer secret in a command argument, URL, diagnostic example, or committed configuration file.

## State contracts explicitly

Reference pages should cover the applicable contract dimensions:

- inputs and accepted forms;
- outputs and destination stream;
- defaults and precedence;
- invariants and tenant ownership;
- exit or HTTP failure behavior; and
- unsupported operations and other limitations.

The live Drift OpenAPI document is authoritative for server behavior. Link to it or the API inventory instead of duplicating route semantics across guides.

## Preserve product boundaries

Documentation must not imply that Drift CLI can create tenants, enumerate deleted records, access Drift storage, or broaden key authority. Use Drift's terms such as tenant, API key, soft-deleted record, restore, and version conflict.

Separate safe mechanical automation from decisions that require operator judgment, including tenant ownership, credential scope, mutation retry, recovery completeness, and secret deployment.

## Review checklist

Before committing documentation, confirm:

- [ ] Each page has one Diátaxis purpose and the correct directory.
- [ ] The opening says when and why to use the page.
- [ ] Procedures include prerequisites, commands, verification, and recovery where relevant.
- [ ] Reference contracts identify inputs, outputs, defaults, failures, and limitations.
- [ ] Explanations focus on rationale rather than step-by-step instructions.
- [ ] Internal links resolve and canonical contracts are not duplicated unnecessarily.
- [ ] Examples contain placeholders rather than real credentials.
- [ ] `npm run check` passes in `site/`.
- [ ] `npm run build` produces the complete static documentation export.
