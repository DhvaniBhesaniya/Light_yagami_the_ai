use Light_yagami_the_ai::{
    models,
    utils::{
        cli::{Cli, Commands, ConfigCommands},
        config::Config,
        logger,
    },
};
use clap::Parser;
use log::{error, info};

use Light_yagami_the_ai::llm::engine::LLMEngine;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    logger::startLogger();
    info!("Starting Light Yagami AI...");

    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let config = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e);
        }
    };

    // Initialize LLM Engine
    // Note: This might take a while to load the model
    let engine = match LLMEngine::new(&config.llm.model_path) {
        Ok(e) => Some(e),
        Err(e) => {
            error!("Failed to initialize LLM Engine: {}", e);
            eprintln!("Warning: LLM Engine failed to load. Check model path in config.");
            None
        }
    };

    match &cli.command {
        Some(Commands::Chat) => {
            info!("Starting chat mode...");
            if let Some(engine) = engine {
                println!("Chat mode started. Type 'exit' or 'quit' to end.");
                loop {
                    print!("> ");
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

                    print!("AI: ");
                    io::stdout().flush()?;

                    let res = engine.stream(input, |token| {
                        print!("{}", token);
                        io::stdout().flush().unwrap_or(());
                        true
                    });

                    if let Err(e) = res {
                        error!("Error generating response: {}", e);
                    }
                    println!(); // Newline after response
                }
            } else {
                eprintln!("Cannot start chat mode without LLM Engine.");
            }
        }
        Some(Commands::Voice) => {
            info!("Starting voice mode...");

            if let Some(engine) = engine {
                use Light_yagami_the_ai::voice::audio::{AudioPlayer, AudioRecorder};
                use Light_yagami_the_ai::voice::stt::SpeechRecognizer;
                use Light_yagami_the_ai::voice::tts::SpeechSynthesizer;
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

                let stt =
                    match SpeechRecognizer::new(&config.voice.voice_model_path, sample_rate as f32)
                    {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Failed to initialize STT: {}", e);
                            return Err(e);
                        }
                    };

                let tts = match SpeechSynthesizer::new(&config.voice.tts_model_path).await {
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

                // Simple loop: Accumulate audio, check for silence/result?
                // Vosk handles stream.

                loop {
                    if let Ok(data) = rx.recv() {
                        if stt.accept_waveform(&data) {
                            let text = stt.final_result();
                            if !text.is_empty() {
                                println!("User: {}", text);

                                println!("AI is thinking...");
                                match engine.predict(&text) {
                                    Ok(response) => {
                                        println!("AI: {}", response);
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
            } else {
                eprintln!("Cannot start voice mode without LLM Engine.");
            }
        }
        Some(Commands::Exec { query }) => {
            let query_str = query.join(" ");
            info!("Executing query: {}", query_str);

            if let Some(engine) = engine {
                match engine.predict(&query_str) {
                    Ok(response) => println!("{}", response),
                    Err(e) => error!("Error generating response: {}", e),
                }
            } else {
                eprintln!("Cannot execute query without LLM Engine.");
            }
        }
        Some(Commands::Config { cmd }) => match cmd {
            ConfigCommands::Show => {
                println!("{:#?}", config);
            }
            ConfigCommands::Init => {
                println!("Config init not implemented yet.");
            }
        },
        None => {
            // Default to chat mode if no command provided
            info!("No command provided, starting chat mode...");
            if let Some(engine) = engine {
                println!("Chat mode started. Type 'exit' or 'quit' to end.");
                loop {
                    print!("> ");
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

                    print!("AI is analyzing...");
                    io::stdout().flush()?;

                    match engine.predict(input) {
                        Ok(response) => {
                            // Clear "AI is analyzing..." line
                            print!("\r\x1b[K"); // Carriage return and clear line
                            println!("AI: {}", response);
                        }
                        Err(e) => {
                            print!("\r\x1b[K");
                            error!("Error generating response: {}", e);
                        }
                    }
                }
            } else {
                eprintln!("Cannot start chat mode without LLM Engine.");
            }
        }
    }

    Ok(())
}
