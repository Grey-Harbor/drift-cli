use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[command(
    name = "drift",
    version,
    about = "Tenant-scoped administration and recovery for Drift"
)]
pub struct Cli {
    /// Select a named profile from the configuration file.
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Read configuration from this file.
    #[arg(long, global = true, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Override the Drift endpoint.
    #[arg(long, global = true, value_name = "URL")]
    pub endpoint: Option<String>,

    /// Emit stable JSON instead of human-readable output.
    #[arg(long, global = true)]
    pub json: bool,

    /// Read the bearer key from standard input.
    #[arg(long, global = true)]
    pub key_stdin: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Check Drift health and discover its API contract.
    Status,

    /// Manage API keys in the authenticated tenant.
    Key {
        #[command(subcommand)]
        command: KeyCommand,
    },

    /// Inspect or restore known soft-deleted records.
    Recovery {
        #[command(subcommand)]
        command: RecoveryCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum KeyCommand {
    /// List key metadata for the authenticated tenant.
    List,

    /// Create a tenant-scoped API key.
    Create(KeySpec),

    /// Immediately and irreversibly revoke a key.
    Revoke {
        id: String,

        /// Acknowledge the irreversible operation.
        #[arg(long, required = true)]
        yes: bool,
    },

    /// Immediately revoke a key and issue its replacement.
    Rotate {
        id: String,

        #[command(flatten)]
        spec: KeySpec,

        /// Acknowledge that the old key stops working immediately.
        #[arg(long, required = true)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct KeySpec {
    /// Human-readable key label.
    #[arg(long)]
    pub label: String,

    /// Grant an explicit Drift scope; repeat for multiple scopes.
    #[arg(long = "scope", required = true, num_args = 1.., value_enum)]
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Read,
    Write,
    Admin,
}

impl Scope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Admin => "admin",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum RecoveryCommand {
    /// Inspect an active or soft-deleted record by ID.
    Show {
        #[arg(value_enum)]
        kind: ResourceKind,
        id: String,
    },

    /// Restore a known soft-deleted record at its current version.
    Restore {
        #[arg(value_enum)]
        kind: ResourceKind,
        id: String,
        #[arg(long)]
        version: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ResourceKind {
    Vertex,
    Edge,
}

impl ResourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
        }
    }

    pub fn collection(self) -> &'static str {
        match self {
            Self::Vertex => "vertices",
            Self::Edge => "edges",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_creation_with_repeated_scopes() {
        let cli = Cli::try_parse_from([
            "drift", "--json", "key", "create", "--label", "service", "--scope", "read", "--scope",
            "write",
        ])
        .unwrap();

        assert!(cli.json);
        let Command::Key {
            command: KeyCommand::Create(spec),
        } = cli.command
        else {
            panic!("expected key create");
        };
        assert_eq!(spec.scopes, [Scope::Read, Scope::Write]);
    }

    #[test]
    fn rejects_missing_key_scope() {
        assert!(Cli::try_parse_from(["drift", "key", "create", "--label", "service"]).is_err());
    }

    #[test]
    fn parses_recovery_restore() {
        let cli = Cli::try_parse_from([
            "drift",
            "recovery",
            "restore",
            "edge",
            "edge-1",
            "--version",
            "3",
        ])
        .unwrap();

        assert!(matches!(
            cli.command,
            Command::Recovery {
                command: RecoveryCommand::Restore {
                    kind: ResourceKind::Edge,
                    version: 3,
                    ..
                }
            }
        ));
    }
}
