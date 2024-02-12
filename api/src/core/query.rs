//! Custom handlers and functions related to queries from the frontend towards the API.

use std::fmt;
use std::str::FromStr;
use chrono::{DateTime, Local, Months, NaiveDate, Utc};
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
/// Allows parsing from the NaiveDate as stored in the database in Utc as well as the expiration time.
///
/// All datetime returned are in [Local].
#[derive(Debug)]
pub struct ExpirationData {
    /// Date into storage of [crate::models::Storage], [Local] Tz.
    pub date_in: DateTime<Local>,
    /// Expiration date of [crate::models::Storage] item, [Local] Tz.
    pub date_expires: DateTime<Local>,
    /// Expiration time of [crate::models::Product], in [Months]
    pub expiration_months: Months,
    /// Time until expiration. Value < 0 if the product has already expired.
    pub expires_in_days: i64,
}

impl ExpirationData {
    /// Takes input date stamp in UTC from database and returns object with useful data for storage calculations.
    pub fn new(date_in: NaiveDate, expiration_months: i32) -> Self {
        // Work in UTC to avoid errors.
        let now = Utc::now();
        let date_in = DateTime::<Utc>::from_naive_utc_and_offset(
            date_in.and_hms_opt(0, 0, 0).unwrap(),
            Utc
        );
        let expiration_months = Months::new(expiration_months as u32);
        let date_expires = date_in.checked_add_months(expiration_months).unwrap();
        let expires_in_days = date_expires.signed_duration_since(now.to_owned()).num_days();

        // Convert to local timezone for output to frontend.
        let date_in = DateTime::<Local>::from(date_in);
        let date_expires = DateTime::<Local>::from(date_expires);
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
        let now = Local::now();
        let date_in = DateTime::parse_from_str(
            format!("2023/01/01 13:00:00 {}", now.offset().to_string().replace(':', "")).as_str(), "%Y/%m/%d %H:%M:%S %z",
        ).unwrap();
        let naive_date_db = date_in.naive_utc().date();
        let expiration_months = 12;
        let expiration_data = ExpirationData::new(naive_date_db, expiration_months);


        let expected_date_expires = DateTime::parse_from_str(
            format!("2024/01/01 13:00:00 {}", now.offset().to_string().replace(':', "")).as_str(), "%Y/%m/%d %H:%M:%S %z",
        ).unwrap();
        assert_eq!(expiration_data.expiration_months, Months::new(expiration_months as u32));
        assert_eq!(expiration_data.date_expires.naive_utc().date(), expected_date_expires.naive_utc().date());
        assert!(expiration_data.expires_in_days < 0);
    }

    #[test]
    fn returns_correct_non_expired_data() {
        let date_in = Local::now().checked_sub_months(Months::new(6)).unwrap();
        let naive_date_db = date_in.naive_utc().date();
        let expiration_months = 12;
        let expiration_data = ExpirationData::new(naive_date_db, expiration_months);

        let expected_date_expires_naive = naive_date_db.checked_add_months(Months::new(expiration_months as u32)).unwrap();
        let expected_date_expires_dt = DateTime::<Local>::from_naive_utc_and_offset(
            expected_date_expires_naive.and_hms_opt(0, 0, 0).unwrap(),
            Local::now().offset().to_owned()
        );

        assert_eq!(expiration_data.expiration_months, Months::new(expiration_months as u32));
        assert_eq!(expiration_data.date_expires.naive_utc(), expected_date_expires_dt.naive_utc());
        assert!(expiration_data.expires_in_days > 0);
    }
}