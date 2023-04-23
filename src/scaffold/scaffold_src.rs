use std::ffi::OsStr;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use path_macro::path;
use crate::error::VanError;
use crate::model::plasmo_lib_options::PlasmoLibFeature;
use crate::model::project_options::ProjectOptions;
use crate::model::project_type::ProjectType;
use crate::RESOURCES;
use crate::scaffold::scaffold_config::{scaffold_config};
use crate::scaffold::scaffold_locale::scaffold_locale;
use crate::scaffold::scaffold_module::{scaffold_module};
use crate::scaffold::scaffold_plugin_main::{scaffold_paper_main, scaffold_velocity_main};

pub fn scaffold_src(options: &ProjectOptions) -> anyhow::Result<()> {

    match &options.project_type {
        ProjectType::Paper => scaffold_paper_src(None, options)?,
        ProjectType::Velocity => scaffold_velocity_src(None, options)?,
        ProjectType::Hybrid => {
            scaffold_common_src(Some(OsStr::new("common")), options)?;
            scaffold_paper_src(Some(OsStr::new("paper")), options)?;
            scaffold_velocity_src(Some(OsStr::new("velocity")), options)?;
        }
    }

    Ok(())
}

fn scaffold_paper_src(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    scaffold_module(module, options)?;

    scaffold_config(module, options)?;

    match options.project_type {
        ProjectType::Hybrid => Ok(()),
        _ => scaffold_locale(module, options)
    }?;

    scaffold_paper_main(module, options)?;

    Ok(())
}

fn scaffold_velocity_src(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    scaffold_module(module, options)?;

    scaffold_config(module, options)?;

    match options.project_type {
        ProjectType::Hybrid => Ok(()),
        _ => scaffold_locale(module, options)
    }?;

    scaffold_velocity_main(module, options)?;

    Ok(())
}

fn scaffold_common_src(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    scaffold_module(module, options)?;

    match options.project_type {
        ProjectType::Hybrid => scaffold_locale(module, options),
        _ => Ok(()),
    }?;

    Ok(())
}