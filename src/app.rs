use crate::llm::engine::LLMEngine;
use crate::utils::config::AppConfig;
use crate::utils::ui::Loader;
use log::{error, info};
use std::io::{self, Write};

pub async fn run_chat_mode(engine: &LLMEngine) -> anyhow::Result<()> {
    info!("Starting chat mode...");
    println!("Chat mode started. Type 'exit' or 'quit' to end.");
    loop {
        print!("<< User --->: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        print!("<< AI --->: ");
        io::stdout().flush()?;

        let mut loader = Loader::new("Analyzing your question..., gathering the answer...");
        loader.start();

        let res = engine.predict(input);

        loader.stop();

        match res {
            Ok(response) => {
                println!("<< AI --->: {}", response);
            }
            Err(e) => {
                error!("Error generating response: {}", e);
            }
        }
        println!(); // Newline after response
    }
    Ok(())
}

pub async fn run_voice_mode(engine: &LLMEngine, config: &AppConfig) -> anyhow::Result<()> {
    info!("Starting voice mode...");
    use crate::voice::audio::{AudioPlayer, AudioRecorder};
    use crate::voice::stt::SpeechRecognizer;
    use crate::voice::tts::SpeechSynthesizer;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    // Initialize Audio Recorder first to get the device's sample rate
    let (_recorder, sample_rate) = match AudioRecorder::new(tx) {
        Ok((r, sr)) => (r, sr),
        Err(e) => {
            error!("Failed to initialize Audio Recorder: {}", e);
            return Err(e);
        }
    };

    info!("Input device sample rate: {} Hz", sample_rate);

    let stt = match SpeechRecognizer::new(&config.voice.voice_model_path, sample_rate as f32) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to initialize STT: {}", e);
            return Err(e);
        }
    };

    let mut tts = match SpeechSynthesizer::new(&config.voice.tts_model_path).await {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to initialize TTS: {}", e);
            return Err(e);
        }
    };

    let player = match AudioPlayer::new() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to initialize Audio Player: {}", e);
            return Err(e);
        }
    };

    println!("Voice mode started. Speak now! (Press Ctrl+C to exit)");

    loop {
        if let Ok(data) = rx.recv() {
            if stt.accept_waveform(&data) {
                let text = stt.final_result();
                if !text.is_empty() {
                    println!("User: {}", text);

                    println!("AI is thinking...");
                    let mut loader =
                        Loader::new("Analyzing your question..., gathering the answer...");
                    loader.start();

                    let res = engine.predict(&text);

                    loader.stop();

                    match res {
                        Ok(response) => {
                            println!("<< AI --->: {}", response);
                            match tts.speak(&response).await {
                                Ok(samples) => {
                                    if let Err(e) = player.play(samples) {
                                        error!("Failed to play audio: {}", e);
                                    }
                                }
                                Err(e) => error!("Failed to synthesize speech: {}", e),
                            }
                        }
                        Err(e) => error!("Error generating response: {}", e),
                    }
                }
            }
        }
    }
}

pub async fn run_exec_mode(engine: &LLMEngine, query: &[String]) -> anyhow::Result<()> {
    let query_str = query.join(" ");
    info!("Executing query: {}", query_str);

    match engine.predict(&query_str) {
        Ok(response) => println!("{}", response),
        Err(e) => error!("Error generating response: {}", e),
    }
    Ok(())
}
