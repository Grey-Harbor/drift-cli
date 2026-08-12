# Why configuration and credentials are separate

Use this explanation when evaluating a new configuration source or credential-storage feature. Drift CLI separates reusable operator preferences from bearer secrets so convenience does not weaken the tenant boundary.

## Profiles describe access, not identity

A profile names a Drift endpoint and the environment variable from which a credential can be read. It may make an operator's intent easier to recognize, but it does not select a tenant on the server.

The bearer key remains the authentication and tenant context. Renaming a profile, changing a profile's endpoint, or supplying a tenant-like label cannot change the tenant encoded by that key.

## Configuration stays non-secret

Configuration files are useful because they can be reviewed, backed up, and shared as operational setup. Storing raw bearer keys in the same files would turn those ordinary workflows into secret-distribution paths.

Drift CLI therefore stores endpoints, output preferences, profile names, and credential-variable names only. Raw credentials come from an explicit standard-input flow or an environment variable.

The [configuration reference](../reference/configuration.md) defines the accepted fields and precedence. The [environment reference](../reference/environment.md) defines credential resolution.

## Explicit sources prevent accidental fallback

Configuration precedence favors the most deliberate source: command-line selections override environment settings, which override profiles and defaults. Credential resolution is narrower and treats empty values as errors.

This asymmetry is intentional. Falling through from an empty or misnamed secret source to another credential could run an administrative command against the wrong tenant while appearing successful.

## Why there is no secret argument

A secret-valued command-line option would be convenient but unsafe on systems that retain shell history or expose process arguments. `--key-stdin` provides an explicit short-lived alternative without placing the bearer key in the command itself.

## Why keychain support is deferred

Native credential stores could improve interactive use, but they introduce platform, headless-environment, deletion, profile-renaming, and fallback semantics. An incomplete design could silently persist secrets in plaintext or behave differently in automation.

Any future keychain integration must be opt-in, define those behaviors across supported platforms, and preserve environment and standard-input workflows for CI and servers.

## Consequences for automation

Automation may choose profiles, set non-secret endpoints, inject environment variables, or pipe a credential through standard input. It must not infer tenant ownership, credential scope, rotation timing, or whether a secret is safe to persist.

One-time secrets may appear only in successful create and rotate results. Errors, retries, diagnostics, and configuration files must remain secret-free.
