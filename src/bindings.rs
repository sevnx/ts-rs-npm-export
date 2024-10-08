use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn get_bindings_dir(base_dir: &Path) -> PathBuf {
    env::var("TS_RS_EXPORT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| base_dir.join("bindings/"))
}

pub fn generate_bindings(path_to_crate: &Path) -> Result<(), std::io::Error> {
    let status = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(path_to_crate.join("Cargo.toml"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to generate the bindings",
        ));
    }

    Ok(())
}

pub fn generate_index_dts(bindings_dir: &Path) -> Result<(), std::io::Error> {
    let exports: Vec<_> = fs::read_dir(bindings_dir)?
        .filter_map(Result::ok)
        .filter_map(|p| {
            p.path()
                .file_name()
                .and_then(std::ffi::OsStr::to_str)
                .map(str::to_owned)
        })
        .filter(|f| !f.contains("index") && f.ends_with(".d.ts"))
        .map(|f| format!("/// <reference path=\"./{f}\""))
        .collect();

    let mut file = File::create(bindings_dir.join("index.d.ts"))?;
    file.write_all(exports.join("\n").as_bytes())?;

    Ok(())
}
