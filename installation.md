# light_yagami_the_ai Installation Guide

This guide provides a comprehensive overview of the project structure, prerequisites, and step-by-step instructions to set up and run the **Light Yagami AI** project on a new system.

## 1. Project Architecture

The following is the folder tree map of the project, identifying key directories and files:

```
Light_yagami_the_ai/
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
├── dependencies/           # External models directory (Created during setup)
│   ├── light_yagami/
│   │   └── models/
│   │       ├── llama_model/
│   │       │   └── mistral-7b-q4_k_m.gguf      # Main LLM Model
│   │       └── kokoro_voices_model/
│   │           ├── kokoro-v1.0.onnx            # TTS Model (ONNX)
│   │           └── voices-v1.0.bin             # TTS Voices Data
│   └── vosk/
│       └── vosk-model-en-in-0.5/               # STT Model (Vosk Directory)
├── libs/                   # Dynamic libraries for Vosk
│   └── vosk-linux-x86_64-0.3.45/
│       └── libvosk.so
└── logs/                   # Log files (Created automatically)
```

## 2. Prerequisites

Before installing the project, ensure your system meets the following requirements:

### System Dependencies (Linux/Ubuntu)
You need to install development libraries for audio (ALSA), SSL (OpenSSL), and standard build tools.

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev libssl-dev pkg-config build-essential
```

### Rust & Cargo
Ensure you have Rust installed. If not, install it via rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## 3. Installation Steps

Follow these steps to set up the project from scratch.

### Step 1: Clone the Repository
```bash
git clone <repository_url>
cd Light_yagami_the_ai
```

### Step 2: Create Directory Structure for Models
We need to create the specific directory structure that `config/config.json` expects.

```bash
mkdir -p dependencies/light_yagami/models/llama_model
mkdir -p dependencies/light_yagami/models/kokoro_voices_model
mkdir -p dependencies/vosk
mkdir -p libs
```

### Step 3: Download Models

You need to download three main components: The LLM model (Mistral), the Speech-to-Text model (Vosk), and the Text-to-Speech model (Kokoro).

#### A. Download LLM (Mistral-7B - Quantized)
Download `mistral-7b-instruct-v0.2.Q4_K_M.gguf` and rename it to `mistral-7b-q4_k_m.gguf`.

*   **URL:** [Hugging Face - TheBloke/Mistral-7B-Instruct-v0.2-GGUF](https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf?download=true)
*   **Destination:** `dependencies/light_yagami/models/llama_model/mistral-7b-q4_k_m.gguf`

```bash
wget "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf?download=true" -O dependencies/light_yagami/models/llama_model/mistral-7b-q4_k_m.gguf
```

#### B. Download Speech-to-Text Model (Vosk)
We use the lightweight Indian English model (`vosk-model-en-in-0.5`). You can choose others from the [Vosk Models page](https://alphacephei.com/vosk/models).

*   **URL:** [Vosk Model (en-in-0.5)](https://alphacephei.com/vosk/models/vosk-model-en-in-0.5.zip)
*   **Destination:** `dependencies/vosk/` (Unzip here)

```bash
wget https://alphacephei.com/vosk/models/vosk-model-en-in-0.5.zip
unzip vosk-model-en-in-0.5.zip -d dependencies/vosk/
rm vosk-model-en-in-0.5.zip
```
*After unzipping, ensure the folder `dependencies/vosk/vosk-model-en-in-0.5` exists and contains files like `model.conf`, `graph`, etc.*

#### C. Download Text-to-Speech Model (Kokoro)
Kokoro requires two files: the ONNX model and the voices binary.

1.  **Kokoro ONNX Model:**
    *   **URL:** [kokoro-v1.0.onnx](https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/kokoro-v1.0.onnx)
    *   **Destination:** `dependencies/light_yagami/models/kokoro_voices_model/kokoro-v1.0.onnx`

2.  **Voices Binary:**
    *   **URL:** [voices-v1.0.bin](https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/voices-v1.0.bin)
    *   **Destination:** `dependencies/light_yagami/models/kokoro_voices_model/voices-v1.0.bin`

```bash
wget "https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/kokoro-v1.0.onnx" -O dependencies/light_yagami/models/kokoro_voices_model/kokoro-v1.0.onnx
wget "https://github.com/thewh1teagle/kokoro-onnx/releases/download/model-files-v1.0/voices-v1.0.bin" -O dependencies/light_yagami/models/kokoro_voices_model/voices-v1.0.bin
```

### Step 4: Set up Libs (Vosk Dependencies)

The project requires the Vosk dynamic library (`libvosk.so`) to verify speech recognition locally.

1.  **Download Vosk Library:**
    *   **URL:** [vosk-linux-x86_64-0.3.45.zip](https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip)
    *   **Destination:** `libs/` (Unzip here)

```bash
wget https://github.com/alphacep/vosk-api/releases/download/v0.3.45/vosk-linux-x86_64-0.3.45.zip -O libs/vosk-linux-x86_64-0.3.45.zip
unzip libs/vosk-linux-x86_64-0.3.45.zip -d libs/
rm libs/vosk-linux-x86_64-0.3.45.zip
```

*Ensure that `libs/vosk-linux-x86_64-0.3.45/libvosk.so` exists.*

> [!NOTE]
> The project's `build.rs` is configured to automatically link to this directory. If you upgrade Vosk, make sure to update `build.rs` and the folder name in `libs/`.

2.  **Environment Variables (Optional but recommended for execution):**
    For running the binary directly (without `cargo run`), you might need to set `LD_LIBRARY_PATH`:
    ```bash
    export LD_LIBRARY_PATH=$(pwd)/libs/vosk-linux-x86_64-0.3.45:$LD_LIBRARY_PATH
    ```


## 4. Configuration

The project uses `config/config.json`. Ensure the paths in this file match where you placed the models.

Default `config.json`:
```json
{
  "logger": {
    "logFileSize": 17408,
    "path": "./logs",
    "all_logs_common_file_path": "logs/step/All-Logs.log",
    "roller_filepath_pattern": "logs/LogsFiles/All-Logs_{}.log",
    "roller_max_count": 999
  },
  "laamaModelPath": "dependencies/light_yagami/models/llama_model/mistral-7b-q4_k_m.gguf",
  "voiceModelPath": "dependencies/vosk/vosk-model-en-in-0.5",
  "voiceTtsModelPath": "dependencies/light_yagami/models/kokoro_voices_model/kokoro-v1.0.onnx"
}
```

## 5. Building and Running

### Build the Project
```bash
cargo build --release
```

### Run the Application
To start the interactive CLI:
```bash
cargo run --release
```

To run in a specific mode (e.g., chat):
```bash
cargo run --release -- chat
```

To run voice mode:
```bash
cargo run --release -- voice
```
