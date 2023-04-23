use std::fs::write;
use std::path::Path;
use path_macro::path;
use crate::config::MAVEN_GROUP;
use crate::model::project_options::ProjectOptions;

pub fn scaffold_gradle_properties(
    options: &ProjectOptions
) -> anyhow::Result<()> {

    let contents = [
        format!("mavenGroup={}.{}", MAVEN_GROUP, &options.flat_name),
        format!("mavenArtifactId={}", &options.flat_name),
        format!("buildVersion=1.0.0"),
    ].join("\n");

    write(path!(&options.path / "gradle.properties"), contents)?;

    Ok(())
}