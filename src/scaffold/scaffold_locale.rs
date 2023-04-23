use std::ffi::OsStr;
use std::fs::{create_dir_all, write};
use path_macro::path;
use crate::model::plasmo_lib_options::PlasmoLibFeature;
use crate::model::project_options::ProjectOptions;

const LOCALE_LIST: [&str; 2] = ["en_us", "ru_ru"];

pub fn scaffold_locale(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Ok(());
    };

    if !plasmo_lib_options.features.contains(&PlasmoLibFeature::Locale) {
        return Ok(());
    };

    let locale_path = path!(options.resources_path(module) / "locale");

    create_dir_all(&locale_path)?;

    let list_path = path!(locale_path / "list");

    write(list_path, LOCALE_LIST.join("\n"))?;

    for locale in LOCALE_LIST {
        let locale_toml_path = path!(locale_path / format!("{locale}.toml"));
        write(locale_toml_path, "")?;
    }

    Ok(())
}