use cargo_metadata::Package;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn generate_package_json(bindings_dir: &Path, package: &Package) -> anyhow::Result<()> {
    let package_json = json!({
        "name": format!("@types/{}", package.name),
        "version": package.version,
        "author": package.authors.join(", "),
        "types": "index.d.ts"
    });

    let package_json_path = bindings_dir.join("package.json");
    let mut package_json_file = File::create(package_json_path.clone()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to create package.json file at {}: {}",
            package_json_path.display(),
            e
        )
    })?;

    serde_json::to_writer_pretty(&mut package_json_file, &package_json).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write package.json file at {}: {}",
            package_json_path.display(),
            e
        )
    })?;

    package_json_file.flush().map_err(|e| {
        anyhow::anyhow!(
            "Failed to flush package.json file at {}: {}",
            package_json_path.display(),
            e
        )
    })?;

    Ok(())
}
