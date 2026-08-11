# Tenant administration trust boundary

Use this explanation when evaluating tenant provisioning or interpreting Drift's `admin` scope. The absence of remote tenant-management routes is a security boundary, not evidence that Drift lacks tenants.

## What Drift manages today

Drift persists tenants and uses them as authorization boundaries. The server-local bootstrap command creates one tenant and that tenant's first admin key. Authentication then derives the tenant ID and scopes from every bearer key.

An `admin` key is therefore a **tenant administrator**, not an instance administrator. It can manage keys, inspect soft-deleted records, and restore records only inside its own tenant. Binding a key to a tenant establishes the request's security context; it does not grant authority over the tenant registry.

Bootstrap can create another tenant when a trusted operator runs it locally with a different unique slug. It crosses the tenant boundary because it runs inside the Drift deployment and accesses the configured repository directly. drift-cli is an independent HTTP client and deliberately cannot inherit that local trust.

## Security failure to avoid

Authorizing remote tenant creation or enumeration with an ordinary tenant admin key would turn a tenant-scoped credential into an instance-wide credential. That would violate the isolation property the key currently establishes.

A compromised admin key for tenant A could otherwise:

- create or discover tenants outside tenant A's authority;
- consume instance-wide storage or operational capacity by provisioning tenants;
- learn tenant names, slugs, status, or other deployment metadata;
- become a stepping stone to issuing credentials for another tenant if route checks drift; or
- exploit ambiguity between tenant `admin` and instance administration in future endpoints.

Exposing the existing bootstrap operation as an unauthenticated or ordinary-admin HTTP route would introduce similar risks, including first-caller takeover, replay, automated tenant creation, and remote use of a capability that is currently restricted to trusted deployment access.

These are potential flaws in a hypothetical design. Current Drift avoids them by keeping bootstrap server-local and keeping every API key tenant-bound.

## Required invariants

drift-cli and any future Drift API must preserve these rules:

1. A tenant admin key never gains authority over another tenant or the tenant registry.
2. Supplying a tenant ID, slug, path value, or profile never changes the tenant derived from the key.
3. drift-cli never simulates bootstrap by reading storage, importing server internals, or executing database mutations.
4. Missing instance-management capability remains unavailable rather than falling back to a weaker authorization path.
5. Authorization failures do not reveal whether another tenant or its resources exist.

## Requirements for any future remote provisioning API

Remote tenant provisioning requires a separately reviewed Drift server design. At minimum it must:

- define an instance-operator principal and credential that are distinct from all tenant API keys;
- reject ordinary tenant admin keys for every instance-level operation;
- be explicitly enabled and configured by the deployment operator;
- define secure initial provisioning, credential rotation, revocation, and recovery;
- return raw credentials only once and never include them in logs or error details;
- provide auditable events without recording bearer secrets;
- bound tenant creation with rate limits, quotas, and replay or idempotency rules;
- prevent tenant enumeration unless explicitly required and authorized; and
- ship OpenAPI contracts and tests proving cross-tenant denial before drift-cli adds commands.

An "administration tenant" alone does not satisfy these requirements. Making its ordinary tenant admin key globally authoritative would collapse two trust domains into one. A future instance operator must be a separate server concept, even if an operator-facing profile makes the two credentials convenient to use.

## drift-cli behavior

The current CLI administers keys and recovery within the tenant selected by the supplied admin key. It may display the tenant ID already present in key metadata, but it cannot create, enumerate, select, inspect, update, or delete tenant records.

Tenant commands must remain unavailable until Drift publishes a server contract that satisfies the boundary above. Client convenience is not sufficient justification for expanding credential authority.
