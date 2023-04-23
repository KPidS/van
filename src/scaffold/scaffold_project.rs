use std::fs::create_dir_all;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use crate::model::project_options::ProjectOptions;
use crate::RESOURCES;
use crate::scaffold::scaffold_build_gradle_kts::scaffold_build_gradle_kts;
use crate::scaffold::scaffold_gradle_properties::scaffold_gradle_properties;
use crate::scaffold::scaffold_root::scaffold_root;
use crate::scaffold::scaffold_settings_gradle_kts::scaffold_settings_gradle_kts;
use crate::scaffold::scaffold_src::scaffold_src;

pub fn scaffold_project(options: ProjectOptions) -> anyhow::Result<()> {

    scaffold_root(&options)?;

    scaffold_settings_gradle_kts(&options)?;

    scaffold_gradle_properties(&options)?;

    scaffold_src(&options)?;

    scaffold_build_gradle_kts(&options)?;




    //
    // let gradlew = RESOURCES.get_file("gradlew").unwrap();
    //
    // let gradlew = gradlew.contents_utf8().unwrap();

    // println!("{}", gradlew);

    Ok(())

}