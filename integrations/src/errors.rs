#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("Missing module info: {0}")]
    MissingModule(String),

    #[error("Substreams output is empty")]
    EmptyOutput,

    #[error("Substreams output does not match cached output")]
    OutputMismatch,

    #[error("Substreams error: {0}")]
    Substream(String),
}
