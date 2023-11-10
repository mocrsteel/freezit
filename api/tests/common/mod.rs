pub mod db;
pub mod db_data;

use std::sync::atomic::AtomicI16;

static DB_COUNT: AtomicI16 = AtomicI16::new(0);
