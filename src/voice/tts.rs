use anyhow::Result;
use kokoro_tts::{KokoroTts, Voice};
use std::path::Path;

pub struct SpeechSynthesizer {
    tts: KokoroTts,
}

impl SpeechSynthesizer {
    pub async fn new(model_path: &str) -> Result<Self> {
        let model_path = if model_path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                model_path.replace("~", home.to_str().unwrap_or(""))
            } else {
                model_path.to_string()
            }
        } else {
            model_path.to_string()
        };

        // Assuming voices-v1.0.bin is in the same directory as the model
        let voices_path = Path::new(&model_path)
            .parent()
            .unwrap()
            .join("voices-v1.0.bin");

        let tts = KokoroTts::new(&model_path, &voices_path.to_str().unwrap().to_string()).await?;
        Ok(Self { tts })
    }

    pub async fn speak(&self, text: &str) -> Result<Vec<f32>> {
        // Use AfHeart voice with speed 1.0
        let (audio, _) = self.tts.synth(text, Voice::AfHeart(1.0)).await?;
        Ok(audio)
    }
}
