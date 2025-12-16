use clap::Parser;
use light_yagami_the_ai::utils::{
    cli::{Cli, Commands, ConfigCommands},
    config::Config,
    logger,
};
use log::{error, info};

use light_yagami_the_ai::llm::engine::LLMEngine;
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
            if let Some(engine) = engine {
                run_chat_mode(&engine).await?;
            } else {
                eprintln!("Cannot start chat mode without LLM Engine.");
            }
        }
        Some(Commands::Voice) => {
            if let Some(engine) = engine {
                run_voice_mode(&engine, &config).await?;
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
            // Interactive Menu
            if let Some(engine) = engine {
                loop {
                    println!("\n=== Light Yagami AI ===");
                    println!("1. Chat Mode");
                    println!("2. Voice Mode");
                    println!("3. Exit");
                    print!("\nSelect an option (1-3): ");
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let choice = input.trim();

                    match choice {
                        "1" => {
                            run_chat_mode(&engine).await?;
                        }
                        "2" => {
                            run_voice_mode(&engine, &config).await?;
                        }
                        "3" => {
                            println!("Goodbye!");
                            break;
                        }
                        _ => {
                            println!("Invalid option. Please try again.");
                        }
                    }
                }
            } else {
                eprintln!("Cannot start interactive mode without LLM Engine.");
            }
        }
    }

    Ok(())
}

async fn run_chat_mode(engine: &LLMEngine) -> anyhow::Result<()> {
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
    Ok(())
}

async fn run_voice_mode(engine: &LLMEngine, config: &Config) -> anyhow::Result<()> {
    info!("Starting voice mode...");
    use light_yagami_the_ai::voice::audio::{AudioPlayer, AudioRecorder};
    use light_yagami_the_ai::voice::stt::SpeechRecognizer;
    use light_yagami_the_ai::voice::tts::SpeechSynthesizer;
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
}
