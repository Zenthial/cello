use std::fs;
use std::path::Path;
use std::process::Command;

pub fn generate(output: String, file_name: String) -> std::io::Result<()> {
    if !Path::new("out").is_dir() {
        fs::create_dir_all("out/src")?;
    }

    let name = file_name.split(".").into_iter().next().unwrap();

    #[cfg(dev)]
    let cargo_toml = format!(
        "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
conum = {{ path = \"../../conum/\" }}
    ",
        name
    );
    #[cfg(not(dev))]
    let cargo_toml = format!(
        "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
conum = \"*\"
    ",
        name
    );

    fs::write("out/src/main.rs", output)?;
    fs::write("out/Cargo.toml", cargo_toml)?;

    Command::new("cargo")
        .args(["fmt", "--manifest-path", "out/Cargo.toml"])
        .output()?;

    Ok(())
}
