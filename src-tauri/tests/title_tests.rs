use log::info;
use serial_test::serial;

// Import directly from the library
use wwe_universe_manager_lib::db::internal_create_belt;

// Import the common module functions
mod common;
use common::setup_test_belt;

#[test]
#[serial]
// Test to create a new Belt
fn test_create_belt() {
    println!("test_create_belt!!!!!!!!!!");
    let (pool, new_belt) = setup_test_belt();
    info!("Creating new Belt");
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let belt = internal_create_belt(&mut conn, &new_belt.name).expect("Belt not created");

    assert_eq!(belt.name, "Testing");
    assert_ne!(belt.name, "Testing1");
}
