use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("transport error: {0}")]
    Transport(String),

    #[error("failed to decode response: {0}")]
    Decode(#[from] serde_json::Error),

    /// The well-known case from the docs: the credential is missing a scope
    /// the endpoint needs. Matched out separately so you don't have to parse
    /// `Error::Api.error` yourself for the one case Lyntr documents.
    #[error("insufficient scope: credential is missing \"{required_scope}\" — {message}")]
    InsufficientScope {
        required_scope: String,
        message: String,
    },

    /// Any other non-2xx response — 400s you didn't expect, 404s, 500s,
    /// future error shapes Lyntr adds. `status` and `error` (the machine-
    /// readable code, when present) let you branch on it yourself instead of
    /// this crate deciding what's "expected" on your behalf.
    #[error("Lyntr API error {status}: {message}")]
    Api {
        status: u16,
        error: Option<String>,
        message: String,
    },

    #[error("invalid request: {0}")]
    Validation(String),
}

#[derive(Debug, Deserialize)]
pub(crate) struct ErrorEnvelope {
    pub error: Option<String>,
    pub required_scope: Option<String>,
    pub message: Option<String>,
}

/// Shared by both transports: given a non-2xx status and the raw response
/// body, decide which `Error` variant it is. `request_scope` is the scope
/// this endpoint required (if any) — used as a fallback when the server's
/// error body says "insufficient_scope" but doesn't itself say which scope.
pub(crate) fn error_from_response(
    status: u16,
    body: &str,
    request_scope: Option<&str>,
) -> Error {
    match serde_json::from_str::<ErrorEnvelope>(body) {
        Ok(envelope) => {
            let required_scope = envelope
                .required_scope
                .or_else(|| request_scope.map(str::to_string));

            if envelope.error.as_deref() == Some("insufficient_scope")
                && let Some(required_scope) = required_scope
            {
                return Error::InsufficientScope {
                    required_scope,
                    message: envelope.message.unwrap_or_else(|| body.to_string()),
                };
            }

            Error::Api {
                status,
                error: envelope.error,
                message: envelope.message.unwrap_or_else(|| body.to_string()),
            }
        }
        Err(_) => Error::Api {
            status,
            error: None,
            message: body.to_string(),
        },
    }
}
