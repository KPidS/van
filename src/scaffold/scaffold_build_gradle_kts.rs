use std::ffi::OsStr;
use std::fs::write;
use path_macro::path;
use sailfish::TemplateOnce;
use serde::de::Unexpected::Option;
use crate::model::paper_options::PaperOptions;
use crate::model::project_options::ProjectOptions;
use crate::model::project_type::ProjectType;
use crate::templates::hybrid_common::HybridCommonTemplate;
use crate::templates::hybrid_paper::HybridPaperTemplate;
use crate::templates::hybrid_root::HybridRootTemplate;
use crate::templates::hybrid_velocity::HybridVelocityTemplate;
use crate::templates::libs::LibsTemplate;
use crate::templates::paper::PaperTemplate;
use crate::templates::velocity::VelocityTemplate;

pub fn scaffold_build_gradle_kts(
    options: &ProjectOptions
) -> anyhow::Result<()> {

    let libs_contents = LibsTemplate {
        plasmo_lib: options.plasmo_lib.is_some(),
        run_paper: match &options.paper {
            None => false,
            Some(paper) => paper.run_server,
        },
        project_type: options.project_type.clone(),
    }.render_once()?;

    write(path!(options.path / "libs.versions.toml"), libs_contents)?;

    match &options.project_type {
        ProjectType::Paper => scaffold_paper_build_gradle_kts(options)?,
        ProjectType::Velocity => scaffold_velocity_build_gradle_kts(options)?,
        ProjectType::Hybrid => {
            scaffold_hybrid_build_gradle_kts(options)?
            // scaffold_common_src(Some(OsStr::new("common")), options)?;
            // scaffold_paper_src(Some(OsStr::new("paper")), options)?;
            // scaffold_velocity_src(Some(OsStr::new("velocity")), options)?;
        }
    }

    Ok(())
}

pub fn scaffold_paper_build_gradle_kts(
    options: &ProjectOptions
) -> anyhow::Result<()> {

    let contents = PaperTemplate {
        maven_publish: options.maven_publish,
        plasmo_lib: options.plasmo_lib.is_some(),
        run_paper: match &options.paper {
            None => false,
            Some(paper) => paper.run_server,
        },
    }.render_once()?;

    write(path!(options.path / "build.gradle.kts"), contents)?;

    Ok(())
}

pub fn scaffold_velocity_build_gradle_kts(
    options: &ProjectOptions
) -> anyhow::Result<()> {

    let contents = VelocityTemplate {
        maven_publish: options.maven_publish,
        plasmo_lib: options.plasmo_lib.is_some(),
    }.render_once()?;

    write(path!(options.path / "build.gradle.kts"), contents)?;

    Ok(())
}

pub fn scaffold_hybrid_build_gradle_kts(
    options: &ProjectOptions
) -> anyhow::Result<()> {

    let maven_publish = options.maven_publish;
    let plasmo_lib = options.plasmo_lib.is_some();
    let run_paper = match &options.paper {
        None => false,
        Some(paper) => paper.run_server,
    };

    let root_contents = HybridRootTemplate {
        maven_publish,
    }.render_once()?;

    write(path!(options.path / "build.gradle.kts"), root_contents)?;

    let common_contents = HybridCommonTemplate.render_once()?;

    write(path!(options.path / "common" / "build.gradle.kts"), common_contents)?;

    let paper_contents = HybridPaperTemplate{
        maven_publish,
        plasmo_lib,
        run_paper,
    }.render_once()?;

    write(path!(options.path / "paper" / "build.gradle.kts"), paper_contents)?;

    let velocity_contents = HybridVelocityTemplate {
        maven_publish,
        plasmo_lib,
        run_paper,
    }.render_once()?;

    write(path!(options.path / "velocity" / "build.gradle.kts"), velocity_contents)?;

    Ok(())
}