use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // 1. Find espeak-ng using pkg-config
    let library = pkg_config::Config::new()
        .probe("espeak-ng")
        .expect("Could not find espeak-ng using pkg-config. Ensure libespeak-ng-dev is installed.");

    // 2. Generate bindgen bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Add include paths from pkg-config
        .clang_args(
            library
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.display())),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 3. Write bindings to $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
