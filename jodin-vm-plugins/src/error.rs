use thiserror::Error;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Dynamically loaded error: {0}")]
    FunctionError(String),
    #[error("Requested label not register (label = {0})")]
    LabelNotRegister(String),
    #[error(transparent)]
    LibraryError(#[from] libloading::Error),
}
