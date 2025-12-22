# Kira AI 🦀🤖

**AI-Powered Offline CLI Assistant**

![Status](https://img.shields.io/badge/Status-Active-brightgreen)
![Rust](https://img.shields.io/badge/Built_With-Rust-orange)
![AI](https://img.shields.io/badge/AI-Mistral_7B-blue)
![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey)

Kira AI is an advanced, offline-first CLI assistant designed for Linux. It integrates local LLMs (Mistral 7B) with voice capabilities (Vosk STT and Kokoro TTS) to provide a private, high-performance AI experience without relying on cloud services.

> **Project Info**
> - **Architecture:** Production-Grade
> - **Tech Stack:** Rust, llama.cpp (Mistral 7B), Kokoro TTS, Vosk STT, Tokio (Async)
> - **Installation:** [View Installation Guide](installation.md)
> - **Roadmap:** [View Development Roadmap](ai-cli-assistant-roadmap.md)

---

## 📁 Project Architecture

The following structure outlines the key components of the Kira AI project:

```
kira_ai/
├── Cargo.toml              # Rust project dependencies and metadata
├── config/
│   └── config.json         # Configuration file for models, paths, and logs
├── src/                    # Source code
│   ├── main.rs             # Entry point of the application
│   ├── app.rs              # Main application logic (modes)
│   ├── llm/                # LLM (Mistral) integration
│   ├── voice/              # Voice modules (STT, TTS, Audio)
│   ├── utils/              # Utilities (Config, Logger, CLI, etc.)
│   └── bin/                # Standalone binaries (for testing)
├── dependencies/           # External models directory
│   └── models/
│   │     ├── llama_model/
│   │     │   └── mistral-7b-q4_k_m.gguf      # Main LLM Model
│   │     └── kokoro_voices_model/
│   │         ├── kokoro-v1.0.onnx            # TTS Model (ONNX)
│   │         └── voices-v1.0.bin             # TTS Voices Data
│   └── vosk/
│       └── vosk-model-en-in-0.5/               # STT Model (Vosk Directory)
├── libs/                   # Dynamic libraries for Vosk
│   └── vosk-linux-x86_64-0.3.45/
│       └── libvosk.so
└── logs/                   # Log files (Created automatically)
```

---

## 🛠️ Prerequisites

Ensure your system (Linux/Ubuntu preferred) meets the following requirements before installation.

### 1. System Dependencies
Install development libraries for audio, SSL, and build tools:

```bash
sudo apt-get update
sudo apt-get install -y \
  espeak-ng \
  libespeak-ng-dev \
  libasound2-dev \
  libssl-dev \
  pkg-config \
  clang \
  libclang-dev \
  cmake \
  build-essential
```

### 2. Rust & Cargo
Install Rust if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. Environment Configuration
Set these variables to ensure the build system finds the system libraries:

```bash
export ESPEAK_NG_USE_SYSTEM=1
export PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig
```

---

## 🚀 Installation Guide

### Step 1: Clone the Repository
```bash
git clone <repository_url>
cd kira_ai
```

### Step 2: Create Directory Structure
Set up the folders required for models and libraries:
```bash
mkdir -p dependencies/models/llama_model
mkdir -p dependencies/models/kokoro_voices_model
mkdir -p dependencies/vosk
mkdir -p libs
```

### Step 3: Download Models
Kira AI requires three local models to function.

#### A. LLM (Mistral-7B)
Download `mistral-7b-instruct-v0.2.Q4_K_M.gguf`:
```bash
wget "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf?download=true" -O dependencies/models/llama_model/mistral-7b-q4_k_m.gguf
```

#### B. Speech-to-Text (Vosk)
Download the lightweight English model:
```bash
wget https://alphacephei.com/vosk/models/vosk-model-en-in-0.5.zip
unzip vosk-model-en-in-0.5.zip -d dependencies/vosk/
rm vosk-model-en-in-0.5.zip
```

#### C. Text-to-Speech (Kokoro)
Download the ONNX model and voices binary:
```bash
wget "https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/kokoro-v1.0.onnx" -O dependencies/models/kokoro_voices_model/kokoro-v1.0.onnx
wget "https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/voices-v1.0.bin" -O dependencies/models/kokoro_voices_model/voices-v1.0.bin
```

### Step 4: Setup Vosk Library
Download the Vosk dynamic library for local linking:
```bash
wget https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip -O libs/vosk-linux-x86_64-0.3.45.zip
unzip libs/vosk-linux-x86_64-0.3.45.zip -d libs/
rm libs/vosk-linux-x86_64-0.3.45.zip
```
> **Note:** The `build.rs` script links to this directory automatically.

---

## ⚙️ Configuration

The project is controlled via `config/config.json`. Ensure the paths match your setup.

**Default `config.json`:**
```json
{
  "logger": {
    "logFileSize": 17408,
    "path": "./logs",
    "all_logs_common_file_path": "logs/step/All-Logs.log",
    "roller_filepath_pattern": "logs/LogsFiles/All-Logs_{}.log",
    "roller_max_count": 999
  },
  "laamaModelPath": "dependencies/models/llama_model/mistral-7b-q4_k_m.gguf",
  "voiceModelPath": "dependencies/vosk/vosk-model-en-in-0.5",
  "voiceTtsModelPath": "dependencies/models/kokoro_voices_model/kokoro-v1.0.onnx"
}
```

---

## 🏃 Usage

### Build
Compile the project in release mode for best performance:
```bash
cargo build --release
```

### Run
**Interactive Mode:**
```bash
cargo run --release
```

**Chat Only Mode:**
```bash
cargo run --release -- chat
```

**Voice Mode:**
```bash
cargo run --release -- voice
```

> **Tip:** If running the binary directly (outside cargo), you may need to set the library path:
> `export LD_LIBRARY_PATH=$(pwd)/libs/vosk-linux-x86_64-0.3.45:$LD_LIBRARY_PATH`
