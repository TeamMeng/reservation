use crate::{ReservationId, ReservationManager, Rsvp};
use abi::{Error, Reservation, ReservationQuery, ReservationStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, PgPool, Row};

#[async_trait]
impl Rsvp for ReservationManager {
    async fn reserve(&self, mut rsvp: Reservation) -> Result<Reservation, Error> {
        rsvp.validate()?;

        let status =
            abi::ReservationStatus::try_from(rsvp.status).unwrap_or(ReservationStatus::Pending);

        let timespan: PgRange<DateTime<Utc>> = rsvp.get_timestamp().into();

        // execute the sql
        let id = sqlx::query(
            "INSERT INTO rsvp.reservations (user_id, resource_id, timespan, note, status) VALUES ($1, $2, $3, $4, $5::rsvp.reservation_status) RETURNING id"
        )
        .bind(rsvp.user_id.clone())
        .bind(rsvp.resource_id.clone())
        .bind(timespan)
        .bind(rsvp.note.clone())
        .bind(status.to_string())
        .fetch_one(&self.pool)
        .await?.get(0);

        rsvp.id = id;

        Ok(rsvp)
    }

    async fn change_status(&self, _id: ReservationId) -> Result<Reservation, Error> {
        todo!()
    }

    async fn update_note(&self, _id: ReservationId, _note: String) -> Result<Reservation, Error> {
        todo!()
    }

    async fn delete(&self, _id: ReservationId) -> Result<Reservation, Error> {
        todo!()
    }

    async fn get(&self, _id: ReservationId) -> Result<Reservation, Error> {
        todo!()
    }

    async fn query(&self, _query: ReservationQuery) -> Result<Reservation, Error> {
        todo!()
    }
}

impl ReservationManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use abi::ReservationConflictInfo;
//     use anyhow::Result;
//     use chrono::FixedOffset;
//     use sqlx_db_tester::TestPg;
//     use std::path::Path;

//     #[tokio::test]
//     async fn reserve_should_work_for_valid_window() -> Result<()> {
//         let db = TestPg::new(
//             "postgres://postgres:postgres@localhost:5432".to_string(),
//             Path::new("../migrations"),
//         );
//         let manager = ReservationManager::new(db.get_pool().await);
//         let start: DateTime<FixedOffset> = "2023-12-25T15:00:00-0700".parse()?;
//         let end: DateTime<FixedOffset> = "2023-12-28T12:00:00-0700".parse()?;
//         let rsvp = Reservation::new_pending(
//             "mengid",
//             "ocean-view-room-713",
//             start,
//             end,
//             "I'll arrive at 3pm. Please help to upgrage to execuitive room if possible",
//         );

//         let rsvp = manager.reserve(rsvp).await?;
//         assert_eq!(rsvp.id, 0);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn reserve_conflict_reservation_should_reject() -> Result<()> {
//         let db = TestPg::new(
//             "postgres://postgres:postgres@localhost:5432".to_string(),
//             Path::new("../migrations"),
//         );
//         let manager = ReservationManager::new(db.get_pool().await);

//         let rsvp1 = Reservation::new_pending(
//             "mengid",
//             "ocean-view-room-713",
//             "2023-12-25T15:00:00-0700".parse()?,
//             "2023-12-28T12:00:00-0700".parse()?,
//             "hello",
//         );
//         let rsvp2 = Reservation::new_pending(
//             "aliceid",
//             "ocean-view-room-713",
//             "2023-12-26T15:00:00-0700".parse()?,
//             "2023-12-30:00:00-0700".parse()?,
//             "hello",
//         );
//         let _rsvp1 = manager.reserve(rsvp1).await?;
//         let err = manager.reserve(rsvp2).await.unwrap_err();
//         if let Error::ConflictReservation(ReservationConflictInfo::Parsed(info)) = err {
//             assert_eq!(info.old.rid, "ocean-view-room-713");
//             assert_eq!(info.old.start.to_rfc3339(), "2022-12-25T22:00:00+00:00");
//             assert_eq!(info.old.end.to_rfc3339(), "2022-12-28T19:00:00+00:00");
//         } else {
//             panic!("expect conflict reservation error");
//         }
//         Ok(())
//     }
// }
