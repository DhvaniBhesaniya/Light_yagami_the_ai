use anyhow::Result;
use light_yagami_the_ai::utils::config::AppConfig;
use light_yagami_the_ai::voice::audio::AudioRecorder;
use light_yagami_the_ai::voice::stt::SpeechRecognizer;
use std::sync::mpsc;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    println!("Initializing STT Demo (Powered by Vosk)...");

    // 1. Load configuration
    let config = AppConfig::load()?;
    let voice_model_path = &config.voice.voice_model_path;

    // Check if model path exists
    let expanded_path = if voice_model_path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            voice_model_path.replace("~", home.to_str().unwrap_or(""))
        } else {
            voice_model_path.clone()
        }
    } else {
        voice_model_path.clone()
    };

    if !std::path::Path::new(&expanded_path).exists() {
        eprintln!("Error: Vosk Model path not found at: {}", expanded_path);
        eprintln!("Please check your config.json or download a model.");
        return Ok(());
    }

    println!("Using Vosk model at: {}", expanded_path);

    // 2. Setup Audio Recorder
    let (tx, rx) = mpsc::channel();
    let (_recorder, sample_rate) = AudioRecorder::new(tx)?;

    println!("Input device sample rate: {} Hz", sample_rate);

    // 3. Initialize Speech Recognizer
    let recognizer = SpeechRecognizer::new(voice_model_path, sample_rate as f32)?;

    println!("Listening... (Press Ctrl+C to exit)");

    // 4. Processing Loop
    loop {
        // AudioRecorder sends Vec<i16> chunks
        if let Ok(data) = rx.recv() {
            if recognizer.accept_waveform(&data) {
                // Returns true if a full utterance is finalized
                let result = recognizer.final_result();
                if !result.is_empty() {
                    println!("Recognized: {}", result);
                }
            } else {
                // Partial results could be printed here if desired
                // let partial = recognizer.partial_result();
                // if !partial.is_empty() {
                //    print!("\rPartial: {}", partial);
                //    use std::io::Write;
                //    std::io::stdout().flush()?;
                // }
            }
        }
    }
}
