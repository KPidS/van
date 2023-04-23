use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use anyhow::anyhow;
use crate::RESOURCES;

pub fn copy_resource(
    resource_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    fs::write(
        target_path,
        RESOURCES.get_file(resource_path)
            .ok_or(anyhow!("Failed to resolve resource"))?
            .contents(),
    )?;
    Ok(())
}

pub fn copy_resources(
    resources: Vec<(impl AsRef<Path>, impl AsRef<Path>)>
) -> anyhow::Result<()> {
    resources.into_iter().map(|resource| {
        copy_resource(resource.0, resource.1)
    }).collect::<anyhow::Result<Vec<()>>>().map(|_| ())
}