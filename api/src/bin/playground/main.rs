use uuid::{Uuid, timestamp::Timestamp};
use std::time::{ UNIX_EPOCH, SystemTime};
fn main() {
    let uuid = Uuid::new_v4();
    println!("UUID: {uuid}");
}