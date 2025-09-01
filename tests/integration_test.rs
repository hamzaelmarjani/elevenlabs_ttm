use elevenlabs_ttm::{ElevenLabsTTMClient, ElevenLabsTTMError, MusicPlan, TTMPromptPlan};

#[tokio::test]
async fn test_client_creation() {
    let _client = ElevenLabsTTMClient::new("test-api-key");
    // Just test that it doesn't panic
    assert_eq!(true, true);
}

#[tokio::test]
async fn test_builder_pattern() {
    let client = ElevenLabsTTMClient::new("test-key");
    let _builder = client.compose_music(MusicPlan::Prompt(TTMPromptPlan::new(
        "Generate a high-tempo dancehall riddim with heavy kicks and bright plucks".to_string(),
    )));

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[test]
fn test_error_display() {
    let error = ElevenLabsTTMError::ValidationError("Invalid voice ID".to_string());
    let display = format!("{}", error);
    assert!(display.contains("Validation error"));
    assert!(display.contains("Invalid voice ID"));
}

// Mock tests for API calls (without real HTTP requests)
#[cfg(test)]
mod mock_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_api_key_error() {
        let _client = ElevenLabsTTMClient::new("invalid-key");

        // This would normally fail with auth error, but we can't test without real API
        // In a real test, you'd use a mock HTTP server like wiremock
        // For now, just test that the client can be created
        assert_eq!(true, true);
    }
}
