
#[derive(thiserror::Error, Debug)]
pub enum VanError {
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    #[error("Failed to decode resource to UTF-8")]
    ResourceDecode,
}