mod error;
mod manager;

use abi::{Reservation, ReservationQuery};
use async_trait::async_trait;
pub use error::ReservationError;
use sqlx::PgPool;

pub type ReservationId = String;
pub type UserId = String;
pub type ResourceId = String;

#[derive(Debug)]
pub struct ReservationManager {
    pool: PgPool,
}

#[async_trait]
pub trait Rsvp {
    /// make a reservation
    async fn reserve(&self, rsvp: Reservation) -> Result<Reservation, ReservationError>;
    /// change reservation status (if current status is pending, change it to confirmed)
    async fn change_status(&self, id: ReservationId) -> Result<Reservation, ReservationError>;
    /// update note
    async fn update_note(
        &self,
        id: ReservationId,
        note: String,
    ) -> Result<Reservation, ReservationError>;
    /// delete reservation
    async fn delete(&self, id: ReservationId) -> Result<Reservation, ReservationError>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<Reservation, ReservationError>;
    /// query reservation
    async fn query(&self, query: ReservationQuery) -> Result<Reservation, ReservationError>;
}
