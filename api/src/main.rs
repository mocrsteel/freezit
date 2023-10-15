use api::connection::establish_connection;

fn main() {
    let conn = establish_connection();
    let test = "Value";
    dbg!(test);
}
