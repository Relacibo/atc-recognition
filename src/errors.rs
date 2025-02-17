use thiserror::Error;
#[derive(Error, Debug)]
pub enum WhisperAtcError {
    #[error("Invalid direction!")]
    InvalidDirection(u32),
    #[error("Invalid altitute!")]
    InvalidAltitute(i32),
    #[error("Invalid turn!")]
    InvalidTurn(u32),
    #[error("Serde Json (de)serialization failed!")]
    SerdeDeserialize(#[from] serde_json::Error),
    #[error("Std Io Error!")]
    StdIo(#[from] std::io::Error),
}
