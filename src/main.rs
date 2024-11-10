use cargo_metadata::MetadataCommand;
use clap::Parser;

mod bindings;
mod cli;
mod package_json;

fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

    let bindings_dir = bindings::get_bindings_dir(&args.path_to_crate);

    bindings::generate_bindings(&args.path_to_crate)?;
    bindings::generate_index_dts(&bindings_dir)?;

    let cargo_package = MetadataCommand::new()
        .manifest_path(args.path_to_crate.join("Cargo.toml"))
        .exec()?;
    let package = cargo_package
        .root_package()
        .ok_or_else(|| anyhow::anyhow!("Failed to get the root package"))?;

    package_json::generate_package_json(&bindings_dir, package)?;

    Ok(())
}
