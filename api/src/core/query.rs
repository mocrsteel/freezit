//! Custom handlers and functions related to queries from the frontend towards the API.

use std::fmt;
use std::str::FromStr;
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
