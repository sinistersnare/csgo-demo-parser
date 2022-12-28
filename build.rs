use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use walkdir::WalkDir;

const README_CONTENTS: &str = " # DO NOT EDIT ANY FILES IN THIS DIRECTORY #

All files in this directory are generated
";

/// Find all *.proto files in the `in_dir` and add them to the list of files
fn get_all_protos(in_dir: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut protos = Vec::new();
    let proto_ext = Some(Path::new("proto").as_os_str());
    for entry in WalkDir::new(&in_dir) {
        let path = entry?.into_path();
        if path.extension() == proto_ext {
            // Re-run this build.rs if any of the files in the protos dir change
            println!(
                "cargo:rerun-if-changed={}",
                path.to_str().context("Bad UTF8")?
            );
            protos.push(path);
        }
    }

    Ok(protos)
}

fn main() -> anyhow::Result<()> {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let in_dir = PathBuf::from(&base_dir).join("protobufs");
    let out_dir = PathBuf::from(&base_dir).join("src/protos");

    // Re-run this build.rs if the protos dir changes (i.e. a new file is added)
    println!(
        "cargo:rerun-if-changed={}",
        in_dir.to_str().context("Bad UTF8")?
    );
    let protos = get_all_protos(in_dir.clone())?;

    generate_protos(in_dir, &out_dir, &protos)?;

    // Write a README.
    let readme_file = out_dir.join("README.md");
    let mut f = File::create(readme_file)?;
    write!(f, "{}", README_CONTENTS)?;

    Ok(())
}

fn generate_protos(in_dir: PathBuf, out_dir: &PathBuf, protos: &[PathBuf]) -> anyhow::Result<()> {
    use pb_rs::types::FileDescriptor;
    use pb_rs::ConfigBuilder;

    // Delete all old generated files before re-generating new ones
    if out_dir.exists() {
        std::fs::remove_dir_all(out_dir)?;
    }

    std::fs::DirBuilder::new().create(out_dir)?;
    let config_builder = ConfigBuilder::new(protos, None, Some(out_dir), &[in_dir])?.build();
    FileDescriptor::run(&config_builder)?;

    Ok(())
}
