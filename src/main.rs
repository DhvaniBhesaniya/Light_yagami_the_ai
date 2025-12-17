use crate::llm::engine::LLMEngine;
use crate::utils::{
    cli::{Cli, Commands, ConfigCommands},
    logger,
};
use clap::Parser;
use log::{error, info};
use std::io::{self, Write};

pub mod configuration;
pub mod llm;
pub mod utils;
pub mod voice;
pub mod app;

// Re-export common types
pub use utils::config::AppConfig;
pub use utils::error::Error;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    logger::startLogger();
    info!("Starting Light Yagami AI...");

    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let config = match AppConfig::load() {
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
                app::run_chat_mode(&engine).await?;
            } else {
                eprintln!("Cannot start chat mode without LLM Engine.");
            }
        }
        Some(Commands::Voice) => {
            if let Some(engine) = engine {
                app::run_voice_mode(&engine, &config).await?;
            } else {
                eprintln!("Cannot start voice mode without LLM Engine.");
            }
        }
        Some(Commands::Exec { query }) => {
            if let Some(engine) = engine {
                app::run_exec_mode(&engine, query).await?;
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
                            app::run_chat_mode(&engine).await?;
                        }
                        "2" => {
                            app::run_voice_mode(&engine, &config).await?;
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
