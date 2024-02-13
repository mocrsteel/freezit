//! Custom handlers and functions related to queries from the frontend towards the API.

use std::fmt;
use std::str::FromStr;
use chrono::{Local, Months, NaiveDate};
use serde::{de, Deserializer, Deserialize};

/// Handler to capture empty query parameters as None.
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

/// Struct containing all relevant datetime information.
/// Allows parsing from the NaiveDate as stored in the database as well as the expiration time.
///
/// All dates returned are in [NaiveDate].
#[derive(Debug)]
pub struct ExpirationData {
    /// Date into storage of [crate::models::Storage], [Local] Tz.
    pub date_in: NaiveDate,
    /// Expiration date of [crate::models::Storage] item, [Local] Tz.
    pub date_expires: NaiveDate,
    /// Expiration time of [crate::models::Product], in [Months]
    pub expiration_months: Months,
    /// Time until expiration. Value < 0 if the product has already expired.
    pub expires_in_days: i64,
}

impl ExpirationData {
    /// Takes input date stamp in UTC from database and returns object with useful data for storage calculations.
    pub fn new(date_in: NaiveDate, expiration_months: i32) -> Self {
        // Work in UTC to avoid errors.
        let today = Local::now().date_naive();
        // let date_in = DateTime::<Utc>::from_naive_utc_and_offset(
        //     date_in.and_hms_opt(0, 0, 0).unwrap(),
        //     Utc
        // );
        let expiration_months = Months::new(expiration_months as u32);
        let date_expires = date_in.checked_add_months(expiration_months).unwrap();
        let expires_in_days = date_expires.signed_duration_since(today.to_owned()).num_days();

        // Convert to local timezone for output to frontend.
        Self {
            date_in,
            date_expires,
            expiration_months,
            expires_in_days,
        }
    }
}

#[cfg(test)]
mod expiration_data {
    use super::*;

    #[test]
    fn returns_correct_data_expired() {
        let date_in = NaiveDate::parse_from_str("2023-01-01", "%Y-%m-%d").unwrap();
        let expiration_months = 12;
        let expiration_data = ExpirationData::new(date_in, expiration_months);

        let expected_date_expires = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d").unwrap();
        assert_eq!(expiration_data.expiration_months, Months::new(expiration_months as u32));
        assert_eq!(expiration_data.date_expires, expected_date_expires);
        assert!(expiration_data.expires_in_days < 0);
    }

    #[test]
    fn returns_correct_non_expired_data() {
        let date_in = Local::now().checked_sub_months(Months::new(6)).unwrap().date_naive();
        let expiration_months = 12;
        let expiration_data = ExpirationData::new(date_in, expiration_months);

        let expected_date_expires = date_in.checked_add_months(Months::new(expiration_months as u32)).unwrap();

        assert_eq!(expiration_data.expiration_months, Months::new(expiration_months as u32));
        assert_eq!(expiration_data.date_expires, expected_date_expires);
        assert!(expiration_data.expires_in_days > 0);
    }
}