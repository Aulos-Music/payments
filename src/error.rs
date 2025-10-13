use thiserror::Error;

#[derive(Error, Debug)]
pub enum SetupError {
    #[error("Tokio error {0}")]
    TokioError(#[from] std::io::Error),

    #[error("Client error")]
    ClientError,

    #[error("Unknown error during setup")]
    Unknown,
}
