mod manager;

use abi::{Error, Reservation, ReservationQuery};
use async_trait::async_trait;
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
    async fn reserve(&self, rsvp: Reservation) -> Result<Reservation, Error>;
    /// change reservation status (if current status is pending, change it to confirmed)
    async fn change_status(&self, id: ReservationId) -> Result<Reservation, Error>;
    /// update note
    async fn update_note(&self, id: ReservationId, note: String) -> Result<Reservation, Error>;
    /// delete reservation
    async fn delete(&self, id: ReservationId) -> Result<Reservation, Error>;
    /// get reservation by id
    async fn get(&self, id: ReservationId) -> Result<Reservation, Error>;
    /// query reservation
    async fn query(&self, query: ReservationQuery) -> Result<Reservation, Error>;
}
