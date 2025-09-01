use std::fmt;

/// All possible errors that can occur when using the ElevenLabs API
#[derive(Debug)]
pub enum ElevenLabsTTMError {
    /// HTTP request failed (network issues, timeout, etc.)
    RequestError(reqwest::Error),

    /// API returned an error status code
    ApiError { status: u16, message: String },

    /// Failed to parse JSON response
    ParseError(serde_json::Error),

    /// Invalid API key or authentication failed
    AuthenticationError(String),

    /// Rate limit exceeded
    RateLimitError {
        retry_after: Option<u64>, // seconds
        message: String,
    },

    /// Quota exceeded (not enough credits)
    QuotaExceededError(String),

    /// Invalid input parameters
    ValidationError(String),
}

impl fmt::Display for ElevenLabsTTMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElevenLabsTTMError::RequestError(e) => write!(f, "Request failed: {}", e),
            ElevenLabsTTMError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            ElevenLabsTTMError::ParseError(e) => write!(f, "Failed to parse response: {}", e),
            ElevenLabsTTMError::AuthenticationError(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
            ElevenLabsTTMError::RateLimitError {
                retry_after,
                message,
            } => match retry_after {
                Some(seconds) => write!(
                    f,
                    "Rate limit exceeded (retry in {}s): {}",
                    seconds, message
                ),
                None => write!(f, "Rate limit exceeded: {}", message),
            },
            ElevenLabsTTMError::QuotaExceededError(msg) => write!(f, "Quota exceeded: {}", msg),
            ElevenLabsTTMError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for ElevenLabsTTMError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ElevenLabsTTMError::RequestError(e) => Some(e),
            ElevenLabsTTMError::ParseError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ElevenLabsTTMError {
    fn from(error: reqwest::Error) -> Self {
        // Check if it's a specific HTTP status error
        if let Some(status) = error.status() {
            let status_code = status.as_u16();
            match status_code {
                401 => ElevenLabsTTMError::AuthenticationError("Invalid API key".to_string()),
                429 => {
                    // Try to extract retry-after header if available
                    ElevenLabsTTMError::RateLimitError {
                        retry_after: None, // Could be enhanced to parse Retry-After header
                        message: "Too many requests".to_string(),
                    }
                }
                402 => ElevenLabsTTMError::QuotaExceededError("Insufficient credits".to_string()),
                _ => ElevenLabsTTMError::ApiError {
                    status: status_code,
                    message: error.to_string(),
                },
            }
        } else {
            ElevenLabsTTMError::RequestError(error)
        }
    }
}

impl From<serde_json::Error> for ElevenLabsTTMError {
    fn from(error: serde_json::Error) -> Self {
        ElevenLabsTTMError::ParseError(error)
    }
}
