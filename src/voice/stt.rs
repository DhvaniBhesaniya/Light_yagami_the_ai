use anyhow::Result;
use std::sync::{Arc, Mutex};
use vosk::{Model, Recognizer};

pub struct SpeechRecognizer {
    recognizer: Arc<Mutex<Recognizer>>,
}

impl SpeechRecognizer {
    pub fn new(model_path: &str, sample_rate: f32) -> Result<Self> {
        let path = if model_path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                model_path.replace("~", home.to_str().unwrap_or(""))
            } else {
                model_path.to_string()
            }
        } else {
            model_path.to_string()
        };

        let model = Model::new(path).ok_or_else(|| anyhow::anyhow!("Failed to load Vosk model"))?;
        let recognizer = Recognizer::new(&model, sample_rate)
            .ok_or_else(|| anyhow::anyhow!("Failed to create recognizer"))?;

        Ok(Self {
            recognizer: Arc::new(Mutex::new(recognizer)),
        })
    }

    pub fn accept_waveform(&self, data: &[i16]) -> bool {
        let mut recognizer = self.recognizer.lock().unwrap();
        // println!("Processing {} samples", data.len());
        match recognizer.accept_waveform(data) {
            Ok(vosk::DecodingState::Finalized) => {
                println!("Vosk: Finalized");
                true
            }
            Ok(vosk::DecodingState::Running) => {
                // println!("Vosk: Running");
                false
            }
            Ok(vosk::DecodingState::Failed) => {
                eprintln!("Vosk: Failed");
                false
            }
            Err(e) => {
                eprintln!("Vosk Error: {:?}", e);
                false
            }
        }
    }

    pub fn final_result(&self) -> String {
        let mut recognizer = self.recognizer.lock().unwrap();
        let result = recognizer.final_result();

        // Debug logging - assuming CompleteResult implements Debug
        // println!("Vosk Raw Result: {:?}", result);

        if let Some(single) = result.single() {
            if !single.text.is_empty() {
                println!("Vosk Result Text: '{}'", single.text);
            }
            single.text.to_string()
        } else {
            // println!("Vosk Result: No single result");
            String::new()
        }
    }

    pub fn partial_result(&self) -> String {
        let mut recognizer = self.recognizer.lock().unwrap();
        let result = recognizer.partial_result();
        result.partial.to_string()
    }
}
