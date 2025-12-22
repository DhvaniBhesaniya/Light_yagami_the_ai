use anyhow::Result;
use kokoro_tiny::TtsEngine;

pub struct SpeechSynthesizer {
    tts: TtsEngine,
}

impl SpeechSynthesizer {
    // Retaining signature compatibility where possible, but kokoro-tiny handles paths
    pub async fn new(_model_path: &str) -> Result<Self> {
        // kokoro-tiny manages its own model path ~/.cache/kokoros
        let tts = TtsEngine::new().await.map_err(|e| anyhow::anyhow!(e))?;
        Ok(Self { tts })
    }

    pub async fn speak(&mut self, text: &str) -> Result<Vec<f32>> {
        // Use default voice "af_sky" for now
        let audio = self
            .tts
            .synthesize(text, Some("af_sky"))
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(audio)
    }
}
