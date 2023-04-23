use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::path::PathBuf;
use path_macro::path;
use crate::config::MAVEN_GROUP;
use crate::model::project_options::ProjectOptions;

pub fn scaffold_module(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    create_dir_all(&options.resources_path(module))?;

    create_dir_all(&options.code_path(module))?;

    Ok(())
}