use chrono::{Months, NaiveDate};

fn main() {
    use chrono::NaiveDate;

    let test_str = "2023-04-01";
    let date_parsed = NaiveDate::parse_from_str(test_str, "%Y-%m-%d").unwrap();
    let date_add_months = date_parsed.checked_add_months(Months::new(12)).unwrap();

    dbg!(date_parsed);
    dbg!(date_add_months);

    println!("playground");
}