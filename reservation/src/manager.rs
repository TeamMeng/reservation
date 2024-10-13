use crate::{ReservationError, ReservationId, ReservationManager, Rsvp};
use abi::{convert_to_utc_time, Reservation, ReservationQuery};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, ReservationError> {
        if rsvp.start.is_none() || rsvp.end.is_none() {
            return Err(ReservationError::InvalidTime);
        }

        // let status = abi::ReservationStatus::try_from(rsvp.status)
        //     .unwrap_or(abi::ReservationStatus::Pending);

        let start = convert_to_utc_time(rsvp.start.unwrap());
        let end = convert_to_utc_time(rsvp.end.unwrap());

        if start <= end {
            return Err(ReservationError::InvalidTime);
        }

        let timespan: PgRange<DateTime<Utc>> = (start..end).into();
        //generate a insert sql for the reservation
        let id = sqlx::query(
            "
            INSERT INTO reservation (user_id, resource_id, timespan, note, status)
            VALUES
            ($1, $2, $3, $4, $5) RETURNING id
            ",
        )
        .bind(rsvp.id)
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(rsvp.status)
        .fetch_one(&self.pool)
        .await?
        .get(0);

        rsvp.id = id;
        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn update_note(
        &self,
        _id: ReservationId,
        _note: String,
    ) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<Reservation, ReservationError> {
        todo!()
    }

    async fn query(&self, _query: ReservationQuery) -> Result<Reservation, ReservationError> {
        todo!()
    }
}
