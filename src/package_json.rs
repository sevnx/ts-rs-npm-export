use cargo_metadata::Package;
use serde_json::json;
use std::fs::File;
use std::path::Path;

pub fn generate_package_json(
    path_to_crate: &Path,
    bindings_dir: &Path,
    package: &Package,
) -> anyhow::Result<()> {
    let package_json = json!({
        "name": format!("@types/{}", package.name),
        "version": package.version,
        "author": package.authors.join(", "),
        "types": "index.d.ts"
    });

    let package_json_path = path_to_crate.join(bindings_dir.join("package.json"));
    let package_json_file = File::create(package_json_path)?;
    serde_json::to_writer_pretty(package_json_file, &package_json)?;

    Ok(())
}
