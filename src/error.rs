use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Usage(String),

    #[error("{0}")]
    Config(String),

    #[error("{0}")]
    Credential(String),

    #[error("could not reach Drift: {0}")]
    Transport(#[from] reqwest::Error),

    #[error("Drift returned HTTP {status}: {message}")]
    Api {
        status: u16,
        code: Option<String>,
        message: String,
    },

    #[error("Drift returned an unexpected response: {0}")]
    Contract(String),

    #[error("output failed: {0}")]
    Output(#[from] std::io::Error),
}

impl AppError {
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::Usage(_) | Self::Config(_) => 2,
            Self::Credential(_) => 3,
            Self::Api { status: 401, .. } => 3,
            Self::Api { status: 403, .. } => 4,
            Self::Api { status: 404, .. } => 5,
            Self::Api { status: 409, .. } => 6,
            Self::Transport(_) => 7,
            Self::Api { .. } | Self::Contract(_) | Self::Output(_) => 8,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Usage(_) => "usage",
            Self::Config(_) => "configuration",
            Self::Credential(_) => "credential",
            Self::Transport(_) => "transport",
            Self::Api { .. } => "api",
            Self::Contract(_) => "contract",
            Self::Output(_) => "output",
        }
    }

    pub fn api_status(&self) -> Option<u16> {
        match self {
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    pub fn api_code(&self) -> Option<&str> {
        match self {
            Self::Api { code, .. } => code.as_deref(),
            _ => None,
        }
    }
}
