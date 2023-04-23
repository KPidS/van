use std::ffi::OsStr;
use std::fs::{create_dir_all, write};
use path_macro::path;
use crate::error::VanError;
use crate::model::plasmo_lib_options::PlasmoLibFeature;
use crate::model::project_options::ProjectOptions;
use crate::RESOURCES;

pub fn scaffold_config(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Ok(());
    };

    if !plasmo_lib_options.features.contains(&PlasmoLibFeature::Config) {
        return Ok(());
    };

    scaffold_config_toml(module, options)?;

    scaffold_config_class(module, options)?;

    Ok(())
}

pub fn scaffold_config_class(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Ok(());
    };

    let imports = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Database) {
        true => vec![
            "import com.plasmoverse.lib.database.DatabaseConfig".to_string()
        ],
        false => vec![],
    }.join("\n");

    let values = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Database) {
        true => vec![
            "val databaseConfig: DatabaseConfig".to_string()
        ],
        false => vec![],
    }
        .into_iter()
        .map(|value| format!("\t{}", value))
        .collect::<Vec<_>>();

    let class = vec![
        "data class Config(".to_string(),
        values.join(",\n"),
        ")".to_string(),
    ].join("\n");

    let contents = vec![
        imports,
        class,
    ].join("\n\n");

    let config_kt_path = path!(options.code_path(module) / "Config.kt");

    write(config_kt_path, contents)?;

    Ok(())
}

pub fn scaffold_config_toml(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Ok(());
    };

    let config_resource_path = "templates/config/toml/database_config.toml";

    let database_config = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Database) {
        true => Some(
            RESOURCES.get_file(config_resource_path).ok_or(
                VanError::ResourceNotFound(config_resource_path.into())
            )?.contents_utf8().ok_or(
                VanError::ResourceDecode
            )?.to_string()
        ),
        false => None,
    };

    let contents = Vec::<String>::new().into_iter().chain(database_config).collect::<String>();

    let config_toml_path = path!(options.resources_path(module) / "config.toml");

    write(config_toml_path, contents)?;

    Ok(())
}
