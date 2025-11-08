//! Application binary launcher

use api::app;


#[tokio::main]
async fn main() {
    app().await;
    // Code references for database interactions with Diesel, delete later.

    // use api::schema::products::dsl::*;
    //
    // let conn = &mut establish_connection();
    //
    // // let new_products: Vec<NewProduct> = vec![
    // //     NewProduct {
    // //         name: "Brocoli",
    // //         expiration_months: Some(16)
    // //     },
    // //     NewProduct {
    // //         name: "Asperges",
    // //         expiration_months: Some(16)
    // //     }
    // // ];
    // // diesel::insert_into(products)
    // //     .values(new_products)
    // //     .returning(Product::as_returning())
    // //     .get_result(conn)
    // //     .expect("Error saving new product");
    // fn get_product_id(conn: &mut PgConnection, query_name: &str) -> Result<Option<i32>, Error> {
    //     products
    //         .select(product_id)
    //         .filter(name.eq(query_name))
    //         .get_result(conn)
    //         .optional()
    // }
    //
    // let results = products
    //     .select(Product::as_select())
    //     .load(conn)
    //     .expect("Error loading products");
    // for product in results {
    //     println!("{} expires after {} months", product.name, product.expiration_months);
    // }
    //
    // if let Ok(Some(test_name)) = get_product_id(conn, "Brocoli") {
    //     println!("{}", test_name);
    // } else {
    //     println!("Could not find Brocoli");
    // }
    // if let Ok(Some(false_name)) = get_product_id(conn, "Non existent") {
    //     println!("{}", false_name);
    // } else {
    //     println!("Could not find 'Non existent'");
    // }
    //
    // if let Ok(Some(new_product_id)) = get_product_id(conn, "Brocoli") {
    //     let new_storage_item = NewStorageItem {
    //         product_id: new_product_id,
    //         weight_grams: 525.2,
    //         date_in: Local::now().date_naive(),
    //         available: true,
    //     };
    //     diesel::insert_into(storage)
    //         .values(new_storage_item)
    //         .returning(Storage::as_returning())
    //         .get_result(conn)
    //         .expect("Could not add new storage item!");
    // } else {
    //     println!("Could not find the product in the database.");
    // }
}