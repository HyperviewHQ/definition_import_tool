use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Input file does not exist")]
    InputFileDoesNotExist,
}
