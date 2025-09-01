use elevenlabs_ttm::{
    ElevenLabsTTMClient, MusicPlan, TTMCompositionPlan, TTMCompositionPlanSection, models,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    println!("Creating ElevenLabs client...");
    let client = ElevenLabsTTMClient::new(api_key);

    // Test advanced music composition with composition plan
    println!("Composing music ...");

    // Sections:
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

    // Example Composition Plan
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

    println!("Generated {} bytes of music audio", music_audio.len());

    // Save to file to outputs directory
    std::fs::create_dir_all("outputs")?;
    let audio_id = chrono::Utc::now().timestamp();
    let file_name = format!("outputs/{}.mp3", audio_id);
    std::fs::write(file_name.clone(), &music_audio)?;
    println!("Music audio saved to {}", file_name);

    Ok(())
}
