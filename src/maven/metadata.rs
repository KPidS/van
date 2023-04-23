use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MavenMetadata {
    pub versioning: MavenVersioning,
}

#[derive(Deserialize)]
pub struct MavenVersioning {
    pub latest: String,
}