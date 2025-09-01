# elevenlabs_ttm

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_ttm.svg)](https://crates.io/crates/elevenlabs_ttm)
[![Docs.rs](https://docs.rs/elevenlabs_ttm/badge.svg)](https://docs.rs/elevenlabs_ttm)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Text-to-Music API](https://elevenlabs.io/app/music). Compose a song from a prompt or a composition plan with a simple, ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring TTM requests
- **Model Support**: Full support for ElevenLabs models (`models::elevenlabs_models::*`)
- **Customizable**: Elevanlabs TTM APIs, custom base URLs, and enterprise support
- **Tokio Ready**: Works seamlessly with the Tokio runtime

## Check-out Also:

**This project is part of a milestone to implement all ElevenLabs APIs in Rust.**

- **[Elevenlabs TTS](https://crates.io/crates/elevenlabs_tts)**: ElevenLabs Text-to-Speech API. ✅
- **[Elevenlabs TTD](https://crates.io/crates/elevenlabs_ttd)**: ElevenLabs Text-to-Dialogue API. ✅
- **[Elevenlabs STT](https://crates.io/crates/elevenlabs_stt)**: ElevenLabs Speech-to-Text API. ✅
- **[Elevenlabs SFX](https://crates.io/crates/elevenlabs_sfx)**: ElevenLabs Sound Effects API. ✅
- **[Elevenlabs VC](https://crates.io/crates/elevenlabs_vc)**: ElevenLabs Voice Changer API. ✅
- **[Elevenlabs TTV](https://crates.io/crates/elevenlabs_ttv)**: ElevenLabs Text-to-Voice API. ✅
- **[Elevenlabs TTM](https://crates.io/crates/elevenlabs_ttm)**: ElevenLabs Text-to-Music API. ✅
- **Elevenlabs AUI**: ElevenLabs Audio Isolation API. ⏳
- **Elevenlabs DUB**: ElevenLabs Dubbing API. ⏳

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_ttm = "0.0.1"
```

## Quick Start

```rust
use elevenlabs_ttm::{ElevenLabsTTMClient, MusicPlan, TTMPromptPlan};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsTTMClient::new("your-api-key");

    let prompt = "Generate an energetic house track with tribal percussion and atmospheric pads.";
    let prompt_plan: MusicPlan = MusicPlan::Prompt(TTMPromptPlan::new(prompt.to_string()));
    let music_audio = client.compose_music(prompt_plan).execute().await?;

    std::fs::write("output.mp3", &music_audio)?;
    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_ttm::{ElevenLabsTTMClient, MusicPlan, TTMPromptPlan};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTMClient::new(api_key);

    let prompt = "Generate an energetic house track with tribal percussion and atmospheric pads.";
    let prompt_plan: MusicPlan = MusicPlan::Prompt(TTMPromptPlan::new(prompt.to_string()));

    let music_audio = client.compose_music(prompt_plan).execute().await?;

    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &music_audio)?;

    Ok(())
}

```

### Advanced Configuration

```rust
use elevenlabs_ttm::{
    ElevenLabsTTMClient, MusicPlan, TTMCompositionPlan, TTMCompositionPlanSection, models,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsTTMClient::new(api_key);

    let section_1: TTMCompositionPlanSection = TTMCompositionPlanSection::new(
        "section-1".to_string(),
        vec!["afrobeats".to_string()],
        vec!["trap".to_string()],
        30000,
        vec![
            "Step in the light, we shine tonight".to_string(),
            "Rhythm and fire, we feel so alive".to_string(),
            "Hands in the sky, the stars in our eyes".to_string(),
            "Dancing together, no need to hide".to_string(),
            "Energy rising, the night is our guide".to_string(),
        ],
    );

    let section_2: TTMCompositionPlanSection = TTMCompositionPlanSection::new(
        "section-2".to_string(),
        vec!["guitar".to_string()],
        vec!["jazz".to_string()],
        30000,
        vec![
            "Heartbeat is racing, move to the sound".to_string(),
            "We lift each other, no holding down".to_string(),
            "Voices are singing, freedom is loud".to_string(),
            "We own the moment, we stand so proud".to_string(),
            "Nothing can stop us, we rule the crowd".to_string(),
        ],
    );

    let section_3: TTMCompositionPlanSection = TTMCompositionPlanSection::new(
        "section-3".to_string(),
        vec!["punchy 808s".to_string()],
        vec!["blues".to_string()],
        30000,
        vec![
            "Morning is coming, but we still go".to_string(),
            "Love in the rhythm, it’s all we know".to_string(),
            "Spirit uniting, it starts to show".to_string(),
            "Holding the groove, we never slow".to_string(),
            "Afrobeats fire, forever flow".to_string(),
        ],
    );

    let composition_plan: MusicPlan = MusicPlan::Composition(TTMCompositionPlan::new(
        vec![
            "afrobeats".to_string(),
            "guitar".to_string(),
            "punchy 808s".to_string(),
        ],
        vec!["trap".to_string(), "jazz".to_string(), "blues".to_string()],
        vec![section_1, section_2, section_3],
    ));

    let music_audio = client
        .compose_music(composition_plan)
        .model(models::elevanlabs_models::MUSIC_V1)
        .execute()
        .await?;

    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &music_audio)?;

    Ok(())
}

```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_ttm

# Run the advanced example
cargo run --example advanced_ttm
```

## API Overview

| Method                                                                                                      | Description                                                                                       |
| ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `ElevenLabsTTMClient::new(String)`                                                                          | Create client instance (required)\*                                                               |
| `.compose_music(MusicPlan)`                                                                                 | Build a TTM request, (MusicPlan::Prompt or MusicPlan::Composition) (required)\*                   |
| `.model(String)`                                                                                            | Select model (optional)                                                                           |
| `.execute()`                                                                                                | Run request → transcribe file (required)\*                                                        |
| `MusicPlan::Prompt(TTMPromptPlan::new(String))`                                                             | Build a Prompt Plan, (prompt) (required)\*                                                        |
| `.compose_music(MusicPlan)`                                                                                 | Build a TTM request, (PromptPlan) (required)\*                                                    |
| `MusicPlan::Composition(TTMCompositionPlan::new(Vec<String>, Vec<String>, Vec<TTMCompositionPlanSection>))` | Build a Composition Plan, (positive_global_styles, negative_global_styles, sections) (required)\* |
| `.compose_music(MusicPlan)`                                                                                 | Build a TTM request, (CompositionPlan) (required)\*                                               |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.compose_music(MusicPlan).execute().await {
    Ok(audio) => println!("Audio file as Vec<u8>: {}", audio.len()),
    Err(e) => eprintln!("TTM request failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/music/compose) for the most up-to-date API information.
