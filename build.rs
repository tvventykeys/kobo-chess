use std::{env, fs, path::PathBuf};

fn main() {
    const KEY: &str = "POCKET_STARTER_PACK";
    println!("cargo:rerun-if-env-changed={KEY}");
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("Cargo output directory"))
        .join("pocket-starter.idx");
    if let Some(source) = env::var_os(KEY) {
        let source = fs::canonicalize(source).expect("read explicitly selected starter pack");
        println!("cargo:rerun-if-changed={}", source.display());
        assert!(
            fs::metadata(&source).expect("starter pack metadata").len() <= 8 * 1024 * 1024,
            "starter pack exceeds 8 MiB"
        );
        fs::copy(source, output).expect("stage starter pack");
    } else {
        fs::write(output, []).expect("stage demo-only marker");
    }
}
