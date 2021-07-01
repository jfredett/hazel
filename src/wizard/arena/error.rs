use super::*;
use crate::*;

pub type ArenaResult<T> = Result<T, ArenaError>;

#[derive(Error, Debug)]
pub enum ArenaError {
    #[error("Failed to find file at: {0}")]
    InvalidPath(PathBuf),
    #[error(transparent)]
    OtherError(#[from] std::io::Error),
    #[error(transparent)]
    BincodeError(#[from] bincode::Error) 
}
