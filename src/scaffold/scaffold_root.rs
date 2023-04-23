use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use anyhow::anyhow;
use path_macro::path;
use crate::model::project_options::ProjectOptions;
use crate::RESOURCES;
use crate::utils::copy_resource::{copy_resource, copy_resources};

pub fn scaffold_root(options: &ProjectOptions) -> anyhow::Result<()> {

    if options.path.exists() {
        let path_str = options.path.as_os_str().to_str().unwrap_or("unknown");
        Err(anyhow!("Folder '{}' already exists. Can't create the project", path_str))?;
    }

    create_dir_all(&options.path)?;

    copy_resources(vec![
        ("gradlew", path!(&options.path / "gradlew")),
        ("gradlew.bat", path!(&options.path / "gradlew.bat")),
        (".gitignore", path!(&options.path / ".gitignore")),
        ("README.md", path!(&options.path / "README.md")),
    ])?;

    Ok(())
}