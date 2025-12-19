use anyhow::Result;
use clap::Parser;
use light_yagami_the_ai::utils::config::AppConfig;
use light_yagami_the_ai::voice::audio::AudioPlayer;
use light_yagami_the_ai::voice::tts::SpeechSynthesizer;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to speak. If not provided, interactive mode is used.
    #[arg(short, long)]
    text: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    println!("Initializing TTS Demo...");

    // 1. Load configuration to get model paths
    let config = AppConfig::load()?;
    let tts_model_path = config.voice.tts_model_path;

    // Expand tilde if present for checking existence
    let expanded_path = if tts_model_path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            tts_model_path.replace("~", home.to_str().unwrap_or(""))
        } else {
            tts_model_path.clone()
        }
    } else {
        tts_model_path.clone()
    };

    let model_path = Path::new(&expanded_path);
    if !model_path.exists() {
        eprintln!("Error: TTS Model file not found at: {}", expanded_path);
        eprintln!("Please ensure the Kokoro ONNX model is at the correct path.");
        eprintln!("You also need 'voices-v1.0.bin' in the same directory.");
        return Ok(());
    }

    println!("Loading model from: {}", expanded_path);

    // 2. Initialize Speech Synthesizer
    // Note: SpeechSynthesizer::new handles tilde expansion internally too, but we pass the config string directly
    let synthesizer = match SpeechSynthesizer::new(&tts_model_path).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize SpeechSynthesizer: {}", e);
            eprintln!("Ensure 'voices-v1.0.bin' is in the same directory as the model.");
            return Ok(());
        }
    };

    // 3. Initialize Audio Player
    let player = match AudioPlayer::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to initialize AudioPlayer: {}", e);
            return Ok(());
        }
    };

    // 4. Get text to speak
    let text_to_speak = match args.text {
        Some(t) => t,
        None => {
            print!("Enter text to speak: ");
            io::stdout().flush()?;
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let trimmed = buffer.trim();
            if trimmed.is_empty() {
                println!("No text entered. Exiting.");
                return Ok(());
            }
            trimmed.to_string()
        }
    };

    println!("Synthesizing text: \"{}\" ...", text_to_speak);

    // 5. Synthesize
    let audio_samples = synthesizer.speak(&text_to_speak).await?;

    println!("Playing audio ({} samples)...", audio_samples.len());

    // 6. Play
    player.play(audio_samples)?;

    println!("Playback finished.");

    Ok(())
}
