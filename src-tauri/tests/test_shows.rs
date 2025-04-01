use log::info;
use serial_test::serial;

// Import from the correct library crate
// The library is defined in src-tauri with the name wwe_universe_manager_lib
extern crate wwe_universe_manager_lib;
// Import the internal function directly
use wwe_universe_manager_lib::db::internal_create_show;
// We still need NewShow for setup_test_show
// use wwe_universe_manager_lib::models::NewShow;

// Import the common module as a separate module
mod common;
use common::setup_test_show;

#[test]
#[serial]
fn test_create_show() {
    // setup_test_show returns pool and the NewShow struct
    let (pool, new_show) = setup_test_show();
    info!("Creating new Show");
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get connection from pool");
    // Call the internal function with the connection and fields from new_show
    let show = internal_create_show(&mut conn, &new_show.name, &new_show.description)
        .expect("Show not created");

    assert_eq!(show.name, new_show.name);
    assert_eq!(show.description, new_show.description);
}