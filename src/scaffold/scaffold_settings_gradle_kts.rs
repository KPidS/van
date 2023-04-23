use std::fs::write;
use std::path::Path;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use path_macro::path;
use crate::error::VanError;
use crate::model::project_options::ProjectOptions;
use crate::model::project_type::ProjectType;
use crate::RESOURCES;

pub fn scaffold_settings_gradle_kts(options: &ProjectOptions) -> anyhow::Result<()> {

    let resource_path = "templates/gradle_settings/repositories.gradle.kts";

    let repositories = RESOURCES.get_file(resource_path)
        .ok_or(VanError::ResourceNotFound(resource_path.into()))?
        .contents_utf8()
        .ok_or(VanError::ResourceDecode)?;

    let project_name = format!("rootProject.name = \"{}\"", &options.pascal_name);

    let include = match options.project_type {
        ProjectType::Hybrid => Some(vec![
            "include(\"common\")",
            "include(\"paper\")",
            "include(\"velocity\")",
        ].join("\n")),
        _ => None,
    };

    let contents = vec![
        repositories.to_string(),
        project_name,
    ]
        .into_iter()
        .chain(include)
        .intersperse("\n\n".to_string())
        .collect::<String>();

    write(path!(&options.path / "settings.gradle.kts"), contents)?;

    Ok(())
}
