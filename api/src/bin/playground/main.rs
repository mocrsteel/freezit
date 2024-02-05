use chrono::{DateTime, Local, Months};

fn main() {
    println!("\n=== Playground ===");

    let expiration_date = DateTime::parse_from_str("2024/09/12 13:00:00 +0100", "%Y/%m/%d %H:%M:%S %z").unwrap();
    dbg!(&expiration_date);

    let local = Local::now().fixed_offset();
    let future = local.checked_add_months(Months::new(6)).unwrap();
    let future_naive = future.naive_utc().date();
    dbg!(&local);
    let future_dt = DateTime::<Local>::from_naive_utc_and_offset(
        future_naive.and_hms_opt(0, 0, 0).unwrap(),
        local.offset().to_owned()
    );
    dbg!(&local.offset());
    dbg!(&future);
    dbg!(&future_naive);
    dbg!(&future_dt);

}