use std::fs;
use std::path::Path;
use std::process::Command;

const CARGO: &'static str = "[package]
name = \"COBOL-to-Rust\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]";

pub fn generate(output: String) -> std::io::Result<()> {
    if Path::new("out").is_dir() {
        if let Err(e) = fs::remove_dir_all("out") {
            panic!("{}", e);
        }
    }

    fs::create_dir_all("out/src")?;
    fs::write("out/src/main.rs", output)?;
    fs::write("out/Cargo.toml", CARGO)?;

    Command::new("cargo")
        .args(["fmt", "--manifest-path", "out/Cargo.toml"])
        .output()?;

    Ok(())
}
