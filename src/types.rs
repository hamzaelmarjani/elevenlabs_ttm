use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicPlan {
    Prompt(TTMPromptPlan),
    Composition(TTMCompositionPlan),
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum TTMRequestBody {
    Prompt(TTMPromptPlan),
    Composition(TTMRequestBodyComposition),
}

#[derive(Debug, Clone, Serialize)]
pub struct TTMRequestBodyComposition {
    pub composition_plan: TTMCompositionPlan,
}

/// Request body for text-to-music API calls
#[derive(Debug, Clone, Serialize)]
pub struct TTMRequest {
    #[serde(skip_serializing)]
    // Output format of the generated audio. Formatted as codec_sample_rate_bitrate. So an mp3 with 22.05kHz sample rate at 32kbs is represented as mp3_22050_32.
    // MP3 with 192kbps bitrate requires you to be subscribed to Creator tier or above. PCM with 44.1kHz sample rate requires you to be subscribed to Pro tier or above.
    // Note that the μ-law format (sometimes written mu-law, often approximated as u-law) is commonly used for Twilio audio inputs.
    // Possible values are: mp3_22050_32 | mp3_44100_32 | mp3_44100_64 | mp3_44100_96 | mp3_44100_128 | mp3_44100_192 | pcm_8000 | pcm_16000 | pcm_22050 | pcm_24000 | pcm_44100 | pcm_48000 | ulaw_8000 | alaw_8000 | opus_48000_32 | opus_48000_64 | opus_48000_96
    // Default to: mp3_44100_128
    // This goes in the URL path, not in the body.
    pub output_format: Option<String>,

    /// The model to use for the generation.
    /// Supported models until now: music_v1. Default to music_v1
    pub model_id: Option<String>,

    /// A detailed music plan to guide the music generation.
    /// You can use only prompt_plan or composition_plan, not boths.
    pub plan: MusicPlan,
}

/// Prompt Plan Option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTMPromptPlan {
    /// A simple text prompt to generate a song from
    pub prompt: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of the song to generate in milliseconds.
    /// Must be between 10000ms and 300000ms.
    /// If not provided, the model will choose a length based on the prompt.
    pub music_length_ms: Option<u32>,
}

impl TTMPromptPlan {
    /// Create new TTMPromptPlan with prompt and custom values
    ///
    /// prompt: A simple text prompt to generate a song from
    ///
    /// music_length_ms: Set music length in milliseconds, between 10000ms and 300000ms.
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            music_length_ms: None,
        }
    }

    /// Set music length in milliseconds
    /// Between 10000ms and 300000ms
    pub fn music_length_ms(mut self, length: u32) -> Self {
        self.music_length_ms = Some(length.clamp(10000, 300000));
        self
    }
}

/// Composition Plan Option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTMCompositionPlan {
    /// The styles that should be present in the entire song.
    pub positive_global_styles: Vec<String>,

    /// The styles that should not be present in the entire song.
    pub negative_global_styles: Vec<String>,

    /// The sections of the song.
    pub sections: Vec<TTMCompositionPlanSection>,
}

impl TTMCompositionPlan {
    /// Create new TTMCompositionPlan with presented values
    pub fn new(
        positive_global_styles: Vec<String>,
        negative_global_styles: Vec<String>,
        sections: Vec<TTMCompositionPlanSection>,
    ) -> Self {
        Self {
            positive_global_styles,
            negative_global_styles,
            sections,
        }
    }
}

/// Composition Plan - Sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTMCompositionPlanSection {
    /// The name of the section. Must be between 1 and 100 characters.
    pub section_name: String,

    /// The styles that should be present in this section.
    pub positive_local_styles: Vec<String>,

    /// The styles that should not be present in this section.
    pub negative_local_styles: Vec<String>,

    /// The duration of the section in milliseconds.
    /// Must be between 3000ms and 120000ms.
    pub duration_ms: u32,

    /// The lyrics of the section.
    pub lines: Vec<String>,
}

impl TTMCompositionPlanSection {
    /// Create new TTMCompositionPlanSection with presented values
    pub fn new(
        section_name: String,
        positive_local_styles: Vec<String>,
        negative_local_styles: Vec<String>,
        duration_ms: u32,
        lines: Vec<String>,
    ) -> Self {
        Self {
            section_name,
            positive_local_styles,
            negative_local_styles,
            duration_ms,
            lines,
        }
    }
}
