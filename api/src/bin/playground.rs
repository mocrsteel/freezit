//! Playground file.

use uuid::{Uuid};

fn main() {
    for i in 0..3 {
        let uuid = Uuid::parse_str("525122f2-3001-4280-9d58-4f83f2748f61").unwrap();
        println!("UUID {i}: {uuid}");
    }
}
