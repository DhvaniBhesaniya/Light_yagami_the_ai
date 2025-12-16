use serde::Deserialize;
// use std::path::PathBuf;
use anyhow::Result;
// use std::fs;

use crate::configuration;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub llm: LlmConfig,
    pub voice: VoiceConfig,
    pub plugins: PluginsConfig,
    // pub system: SystemConfig,
}

#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    pub model_path: String,
    pub max_context: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model_path: configuration::get::<String>("laamaModelPath"),
            max_context: 8192,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 40,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VoiceConfig {
    pub tts_engine: String,
    pub stt_engine: String,
    pub voice_model_path: String, // Used for STT (Vosk)
    pub tts_model_path: String,   // Used for TTS (Kokoro)
    pub sample_rate: u32,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            tts_engine: "kokoro".to_string(),
            stt_engine: "vosk".to_string(),
            voice_model_path: configuration::get::<String>("voiceModelPath"),
            tts_model_path: configuration::get::<String>("voiceTtsModelPath"),
            sample_rate: 16000,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct PluginsConfig {
    pub enabled: Vec<String>,
}

// #[derive(Debug, Deserialize)]
// pub struct SystemConfig {
//     pub log_path: String,
//     pub db_path: String,
// }

// impl Default for SystemConfig {
//     fn default() -> Self {
//         Self {
//             log_path: "~/.light_yagami/logs".to_string(),
//             db_path: "~/.light_yagami/data.db".to_string(),
//         }
//     }
// }

impl Config {
    pub fn load() -> Result<Self> {
        // let config_path = dirs::home_dir()
        //     .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
        //     .join(".light_yagami/config.toml");

        // if !config_path.exists() {
        // Return default config if file doesn't exist
        // In a real app, we might want to create the default file here
        return Ok(Config::default());
        // }

        // let content = fs::read_to_string(config_path)?;
        // let config: Config = toml::from_str(&content)?;
        // Ok(config)
    }
}
