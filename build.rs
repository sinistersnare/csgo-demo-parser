use std::path::{Path, PathBuf};

use anyhow::Context;
use walkdir::WalkDir;

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

    // Re-run this build.rs if the protos dir changes (i.e. a new file is added)
    println!(
        "cargo:rerun-if-changed={}",
        in_dir.to_str().context("Bad UTF8")?
    );
    let protos = get_all_protos(in_dir.clone())?;

    prost_build::compile_protos(&protos, &[&in_dir])?;

    Ok(())
}
