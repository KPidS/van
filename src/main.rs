#![feature(iter_intersperse)]

use std::time::Instant;
use dialoguer::theme::ColorfulTheme;
use include_dir::{Dir, include_dir};
use lazy_static::lazy_static;
use pipe_trait::Pipe;
use crate::config::PLASMO_LIB_METADATA_URL;
use crate::maven::metadata::MavenMetadata;
use crate::model::paper_options::PaperOptions;
use crate::model::project_options::ProjectOptions;
use crate::model::project_type::ProjectType;
use crate::scaffold::scaffold_project::scaffold_project;

mod model;
mod scaffold;
mod utils;
mod error;
mod config;
mod maven;
pub mod templates;

static RESOURCES: Dir = include_dir!("resources");

lazy_static! {
    pub static ref DIALOGUE_THEME: ColorfulTheme = ColorfulTheme::default();
}

fn main() -> anyhow::Result<()> {

    let options = ProjectOptions::from_dialogue()?;

    // let metadata = reqwest::blocking::get(PLASMO_LIB_METADATA_URL)?
    //     .text()?
    //     .pipe(|string| serde_xml_rs::from_str::<MavenMetadata>(string.as_str()))?;
    //
    // println!("PlasmoLib version: {}", metadata.versioning.latest);

    let instant = Instant::now();

    scaffold_project(options)?;

    println!("Took: {} ms", instant.elapsed().as_millis());

    Ok(())
}
