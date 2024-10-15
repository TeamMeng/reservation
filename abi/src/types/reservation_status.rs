use crate::ReservationStatus;
use std::fmt::Display;

impl Display for ReservationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReservationStatus::Pending => write!(f, "Pending"),
            ReservationStatus::Blocked => write!(f, "Blocked"),
            ReservationStatus::Confirmed => write!(f, "Confirmed"),
            ReservationStatus::Unknown => write!(f, "Unknown"),
        }
    }
}
