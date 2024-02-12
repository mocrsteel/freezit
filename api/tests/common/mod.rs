pub mod db;
pub mod db_data;

use std::sync::atomic::AtomicU16;

static DB_COUNT: AtomicU16 = AtomicU16::new(0);
