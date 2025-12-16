use std::env;
use std::path::PathBuf;

fn main() {
    // Get the project root directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(manifest_dir);

    // Path to the Vosk library
    // We use the specific version folder found in the libs directory
    let vosk_lib_dir = root.join("libs").join("vosk-linux-x86_64-0.3.45");

    if !vosk_lib_dir.exists() {
        println!(
            "cargo:warning=Vosk library directory not found at: {}",
            vosk_lib_dir.display()
        );
    }

    // Tell Cargo where to find the library for linking
    println!("cargo:rustc-link-search=native={}", vosk_lib_dir.display());

    // Tell Cargo to link against libvosk
    println!("cargo:rustc-link-lib=vosk");

    // Tell the linker to add the library directory to the runtime search path (rpath)
    // This allows the binary to find libvosk.so without setting LD_LIBRARY_PATH
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", vosk_lib_dir.display());

    // Rerun this script if the library directory changes (unlikely, but good practice)
    println!("cargo:rerun-if-changed=libs/vosk-linux-x86_64-0.3.45");
}
