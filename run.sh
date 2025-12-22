#!/bin/bash
export LIBRARY_PATH=$(pwd)/libs/vosk-linux-x86_64-0.3.45:$LIBRARY_PATH
export LD_LIBRARY_PATH=$(pwd)/libs/vosk-linux-x86_64-0.3.45:$LD_LIBRARY_PATH
cargo run --bin light_yagami_the_ai -- "$@"
