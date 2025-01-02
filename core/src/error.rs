use thiserror::Error;

/// This enum defines every base error for a actor handler
#[derive(Error, Debug)]
pub enum ActorError {
    #[error("task cancelled by actor")]
    Cancelled,
    #[error(transparent)]
    Failed(#[from] anyhow::Error),
}
