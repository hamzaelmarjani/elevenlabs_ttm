//! ElevenLabs Text-to-Music API client
//!
//! A type-safe, async Rust client for the ElevenLabs TTM API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_ttm::{ElevenLabsTTMClient, MusicPlan, TTMPromptPlan};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsTTMClient::new("your-api-key");
//!
//!    let prompt = "Generate an energetic house track with tribal percussion and atmospheric pads.";
//!    let prompt_plan: MusicPlan = MusicPlan::Prompt(TTMPromptPlan::new(prompt.to_string()));
//!     
//!    let music_audio = client.compose_music(prompt_plan).execute().await?;
//!     
//!     // music_audio is Vec<u8> - raw msuic audio's data
//!     std::fs::write("output.mp3", music_audio)?;
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod models;
pub mod types;

pub use error::ElevenLabsTTMError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsTTMClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsTTMClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL (for testing/enterprise)
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a compose music request
    pub fn compose_music<P: Into<MusicPlan>>(&self, plan: P) -> TextToMusicBuilder {
        TextToMusicBuilder::new(self.clone(), plan.into())
    }

    /// Internal method to execute TTM request
    pub(crate) async fn execute_ttm(
        &self,
        request: TTMRequest,
        output_format: String,
    ) -> Result<Vec<u8>, ElevenLabsTTMError> {
        let url = format!("{}/music?output_format={}", self.base_url, output_format);

        let body = match &request.plan {
            MusicPlan::Prompt(prompt_plan) => TTMRequestBody::Prompt(TTMPromptPlan {
                prompt: prompt_plan.prompt.clone(),
                music_length_ms: prompt_plan.music_length_ms,
            }),
            MusicPlan::Composition(composition_plan) => {
                TTMRequestBody::Composition(TTMRequestBodyComposition {
                    composition_plan: TTMCompositionPlan {
                        positive_global_styles: composition_plan.positive_global_styles.clone(),
                        negative_global_styles: composition_plan.negative_global_styles.clone(),
                        sections: composition_plan.sections.clone(),
                    },
                })
            }
        };

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsTTMError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        Ok(response.bytes().await?.to_vec())
    }
}

/// Builder for text-to-speech requests
pub struct TextToMusicBuilder {
    client: ElevenLabsTTMClient,
    output_format: Option<String>,
    model_id: Option<String>,
    plan: MusicPlan,
}

impl TextToMusicBuilder {
    fn new(client: ElevenLabsTTMClient, plan: MusicPlan) -> Self {
        Self {
            client,
            plan,
            model_id: None,
            output_format: None,
        }
    }

    /// Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    /// MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    /// Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    /// Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    /// Default to: mp3_44100_128
    /// This goes in the URL path, not in the body.
    pub fn output_format<S: Into<String>>(mut self, output_format: S) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// The model to use for the generation.
    /// Supported models until now: music_v1. Default to music_v1
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Execute the text-to-music request
    pub async fn execute(self) -> Result<Vec<u8>, ElevenLabsTTMError> {
        let output_format = self
            .output_format
            .unwrap_or_else(|| "mp3_44100_128".to_string());

        let request = TTMRequest {
            plan: self.plan,
            output_format: Some(output_format.clone()),
            model_id: Some(
                self.model_id
                    .unwrap_or_else(|| models::elevanlabs_models::MUSIC_V1.to_string()),
            ),
        };

        self.client.execute_ttm(request, output_format).await
    }
}
