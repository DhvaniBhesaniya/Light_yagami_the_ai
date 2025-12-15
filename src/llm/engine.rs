use anyhow::Result;
use llama_cpp::standard_sampler::StandardSampler;
use llama_cpp::{LlamaModel, LlamaParams, LlamaSession, SessionParams};
use std::sync::Mutex;

pub struct LLMEngine {
    model: LlamaModel,
    session: Mutex<LlamaSession>,
}

impl LLMEngine {
    pub fn new(model_path: &str) -> Result<Self> {
        let path = if model_path.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                model_path.replace("~", home.to_str().unwrap_or(""))
            } else {
                model_path.to_string()
            }
        } else {
            model_path.to_string()
        };

        let params = LlamaParams::default();
        let model = LlamaModel::load_from_file(&path, params)?;
        let session = model.create_session(SessionParams::default())?;

        Ok(Self {
            model,
            session: Mutex::new(session),
        })
    }

    pub fn predict(&self, prompt: &str) -> Result<String> {
        let mut session = self.session.lock().unwrap();

        let formatted_prompt = format!("[INST] {} [/INST]", prompt);
        session.advance_context(&formatted_prompt)?;

        let completions = session
            .start_completing_with(StandardSampler::default(), 1024)?
            .into_strings();

        let mut result = String::new();
        for token in completions {
            result.push_str(&token);
        }

        Ok(result)
    }

    pub fn stream<F>(&self, prompt: &str, mut callback: F) -> Result<()>
    where
        F: FnMut(&str) -> bool,
    {
        let mut session = self.session.lock().unwrap();

        let formatted_prompt = format!("[INST] {} [/INST]", prompt);
        session.advance_context(&formatted_prompt)?;

        let completions = session
            .start_completing_with(StandardSampler::default(), 1024)?
            .into_strings();

        for token in completions {
            if !callback(&token) {
                break;
            }
        }
        Ok(())
    }
}
