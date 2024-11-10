use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn get_bindings_dir(base_dir: &Path) -> PathBuf {
    env::var("TS_RS_EXPORT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| base_dir.join("bindings"))
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
    let ts_files = get_index_content(bindings_dir, None)?;
    let mut file = File::create(bindings_dir.join("index.d.ts"))?;
    file.write_all(ts_files.join("\n").as_bytes())?;
    Ok(())
}

fn get_index_content(
    path: &Path,
    relative_path: Option<&Path>,
) -> Result<Vec<String>, std::io::Error> {
    let mut ts_files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let current_relative = match relative_path {
            Some(rel_path) => rel_path.join(&file_name),
            None => PathBuf::from(&file_name),
        };

        match entry.file_type()? {
            file_type if file_type.is_dir() => {
                ts_files.extend(get_index_content(&entry.path(), Some(&current_relative))?);
            }
            file_type if file_type.is_file() => {
                let file_name = file_name.to_string_lossy();
                if file_name.ends_with(".d.ts") && !file_name.contains("index") {
                    let reference_path =
                        format!("./{}", current_relative.display()).replace('\\', "/");
                    ts_files.push(format!("/// <reference path=\"{}\" />", reference_path));
                }
            }
            _ => {}
        }
    }

    Ok(ts_files)
}
