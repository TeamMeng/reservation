use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),

    #[error("Invalid start or end time for the reservation")]
    InvalidTime,

    #[error("unknown error")]
    Unknown,
}
