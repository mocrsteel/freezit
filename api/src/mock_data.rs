//! Module containing mock data that matches the database schemas for this API.
//! 
//! Generated using ChatGPT.

use crate::models::StorageTuple;

/// Data for [Drawer](crate::models::EmailWhitelist) model.
pub static EMAIL_WHITELIST: [(i32, &str); 5] = [
    (1, "alice@example.com"),
    (2, "bob@example.com"),
    (3, "charlie@example.com"),
    (4, "diana@example.com"),
    (5, "ethan@example.com"),
];

/// Data for [Storage](crate::models::Storage) model.
pub static STORAGE: [StorageTuple; 10] = [
    (1, 1, 450.0, "2025-02-10", "2025-05-15", 1, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (2, 2, 300.0, "2025-01-05", "", 3, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
    (3, 3, 780.5, "2025-03-12", "2025-06-20", 5, "64d57bc8-bbf5-47df-bfa9-91b70a9395c4"),
    (4, 4, 250.3, "2024-12-18", "", 6, "ae84fba1-f95a-4df4-bf5d-41a1b176b331"),
    (5, 5, 640.8, "2025-04-22", "2025-09-12", 7, "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (6, 6, 520.4, "2025-06-01", "", 8, "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (7, 7, 890.2, "2025-03-14", "2025-08-01", 9, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (8, 8, 412.0, "2025-02-25", "", 10, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
    (9, 5, 220.0, "2025-05-11", "2025-07-15", 2, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (10, 3,540.5, "2025-01-19", "",  4, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
];

/// Data for the [Product](crate::models::Product) model.
pub static PRODUCTS: [(i32, &str, i32, &str); 8] = [
    (1, "Broccoli", 12, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (2, "Groentensoep", 16, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
    (3, "Spruiten", 12, "64d57bc8-bbf5-47df-bfa9-91b70a9395c4"),
    (4, "Pastinaaksoep", 12, "ae84fba1-f95a-4df4-bf5d-41a1b176b331"),
    (5, "Puree", 18, "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (6, "Kippenballetjes", 6, "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (7, "Spaghettisaus", 12, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (8, "Hamburgers", 6, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
];

/// Data for the [Drawer](crate::models::Drawer) model.
pub static DRAWERS: [(i32, &str, i32, &str); 10] = [
    (1, "Top Shelf", 1, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (2, "Bottom Shelf", 1, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (3, "Left Drawer", 2, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
    (4, "Right Drawer", 2, "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
    (5, "Middle Rack", 3, "64d57bc8-bbf5-47df-bfa9-91b70a9395c4"),
    (6, "Door Bin", 4, "ae84fba1-f95a-4df4-bf5d-41a1b176b331"),
    (7, "Deep Drawer", 5, "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (8, "Snack Tray", 5, "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (9, "Ice Tray", 6, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (10, "Bottom Bin", 6, "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
];

/// Data for the [Freezer](crate::models::Freezer) model.
pub static FREEZERS: [(i32, &str, &str); 6] = [
    (1, "Main Freezer", "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
    (2, "Basement Freezer", "4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"),
    (3, "Garage Freezer", "64d57bc8-bbf5-47df-bfa9-91b70a9395c4"),
    (4, "Pantry Freezer", "ae84fba1-f95a-4df4-bf5d-41a1b176b331"),
    (5, "Mini Freezer", "cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"),
    (6, "Office Freezer", "d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"),
];

use uuid::uuid;

/// Data for the [User](crate::models::User) model.
pub static USERS: [(uuid::Uuid, &str, &str, i32); 5] = [
    (uuid!("d7a3b84f-7a61-4c6e-b6dc-23b47b87d3d1"), "Alice Johnson", "alice@example.com", 0),
    (uuid!("4f1bca35-65d9-4cc4-8a62-9621cf3b8ff4"), "Bob Smith", "bob@example.com", 0),
    (uuid!("64d57bc8-bbf5-47df-bfa9-91b70a9395c4"), "Charlie Evans", "charlie@example.com", 0),
    (uuid!("ae84fba1-f95a-4df4-bf5d-41a1b176b331"), "Diana Cooper", "diana@example.com", 0),
    (uuid!("cb39ad25-d6f3-4f9c-80ea-5d2eb71b0d82"), "Ethan Brown", "ethan@example.com", 0),
];
