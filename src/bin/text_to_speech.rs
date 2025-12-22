use clap::Parser;
use kokoro_tiny::TtsEngine;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to speak. If not provided, interactive mode is used.
    #[arg(short, long)]
    text: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Initializing kokoro-tiny Demo...");

    // Initialize (downloads model on first run)
    let mut tts = TtsEngine::new().await?;

    // Get text to speak
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

    // Generate speech
    let audio = tts.synthesize(&text_to_speak, Some("af_sky"))?;

    println!("Playing audio...");

    // Play directly (requires 'playback' feature which is default)
    tts.play(&audio, 1.0)?;

    println!("Playback finished.");

    Ok(())
}
