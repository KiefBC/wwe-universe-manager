use log::info;
use serial_test::serial;

// Import directly from the library
use wwe_universe_manager_lib::db::create_belt;

// Import the common module functions
mod common;
use common::setup_test_belt;

#[test]
#[serial]
// Test to create a new Belt
fn test_create_belt() {
    println!("test_create_belt!!!!!!!!!!");
    let (mut conn, new_belt) = setup_test_belt();
    info!("Creating new Belt");
    let belt = create_belt(&mut conn, new_belt).expect("Belt not created");

    assert_eq!(belt.name, "Testing");
    assert_ne!(belt.name, "Testing1");
}
