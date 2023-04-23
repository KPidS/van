use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use convert_case::{Case, Casing};
use dialoguer::{Confirm, Input};
use path_macro::path;
use crate::config::MAVEN_GROUP;
use crate::DIALOGUE_THEME;
use crate::model::paper_options::PaperOptions;
use crate::model::plasmo_lib_options::PlasmoLibOptions;
use crate::model::project_type::ProjectType;

pub struct ProjectOptions {
    pub name: String,
    pub pascal_name: String,
    pub flat_name: String,
    pub snake_name: String,
    pub path: PathBuf,
    pub project_type: ProjectType,
    pub paper: Option<PaperOptions>,
    pub plasmo_lib: Option<PlasmoLibOptions>,
    pub maven_publish: bool,
}

impl ProjectOptions {

    pub fn main_path(&self, module: Option<&OsStr>) -> PathBuf {
        self.path
            .iter()
            .chain(module)
            .chain(std::iter::once(OsStr::new("src")))
            .chain(std::iter::once(OsStr::new("main")))
            .collect::<PathBuf>()
    }

    pub fn resources_path(&self, module: Option<&OsStr>) -> PathBuf {
        path!(self.main_path(module) / "resources")
    }

    pub fn code_path(&self, module: Option<&OsStr>) -> PathBuf {
        self.main_path(module)
            .clone()
            .into_iter()
            .chain(std::iter::once(OsStr::new("kotlin")))
            .chain(MAVEN_GROUP.split(".").map(|str| OsStr::new(str)))
            .chain(std::iter::once(OsStr::new(&self.flat_name)))
            .collect::<PathBuf>()
    }

    pub fn from_dialogue() -> anyhow::Result<Self> {

        let name = Input::<String>::with_theme(&*DIALOGUE_THEME)
            .with_prompt("Project Name")
            .default("Test".into())
            .interact_text()?;

        let project_type = ProjectType::from_dialogue()?;

        let paper = match &project_type {
            ProjectType::Paper => Some(PaperOptions::from_dialogue()?),
            _ => None,
        };

        let include_plasmo_lib = Confirm::with_theme(&*DIALOGUE_THEME)
            .default(true)
            .with_prompt("Include PlasmoLib")
            .interact()?;

        let plasmo_lib = match include_plasmo_lib {
            true => Some(PlasmoLibOptions::from_dialogue()?),
            false => None,
        };

        let maven_publish = Confirm::with_theme(&*DIALOGUE_THEME)
            .with_prompt("Create Maven Publication")
            .default(false)
            .interact()?;

        let pascal_name = name.to_case(Case::Pascal);

        let flat_name = name.to_case(Case::Flat);

        let snake_name = name.to_case(Case::Snake);

        let path = path!(std::env::current_dir()? / &pascal_name);

        Ok(Self {
            name,
            pascal_name,
            flat_name,
            snake_name,
            path,
            project_type,
            paper,
            plasmo_lib,
            maven_publish,
        })
    }
}