use elevenlabs_ttm::{ElevenLabsTTMClient, MusicPlan, TTMPromptPlan};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    println!("Creating ElevenLabs client...");
    let client = ElevenLabsTTMClient::new(api_key);

    // Test basic music composition with prompt plan
    println!("Composing music ...");

    // Example prompt text
    let prompt = "Generate an energetic house track with tribal percussion and atmospheric pads.";

    // Example Prompt Plan
    let prompt_plan: MusicPlan = MusicPlan::Prompt(TTMPromptPlan::new(prompt.to_string()));

    let music_audio = client.compose_music(prompt_plan).execute().await?;

    println!("Generated {} bytes of music audio", music_audio.len());

    // Save to file to outputs directory
    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &music_audio)?;
    println!("Music audio saved to {}", file_name);

    Ok(())
}
