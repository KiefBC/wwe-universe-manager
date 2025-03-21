use log::info;
use serial_test::serial;

// Import from the correct library crate
// The library is defined in src-tauri with the name wwe_universe_manager_lib
extern crate wwe_universe_manager_lib;
use wwe_universe_manager_lib::db::create_wrestler;

// Import the common module as a separate module
mod common;
use common::setup_test_wrestler;

#[test]
#[serial]
// Test to create a new Wrestler
fn test_create_wrestler() {
    let (mut conn, new_wrestler) = setup_test_wrestler();
    info!("Creating new Wrestler");
    let wrestler = create_wrestler(&mut conn, new_wrestler).expect("Wrestler not created");

    assert_eq!(wrestler.name, "Testing");
    assert_ne!(wrestler.name, "Testing1");
}
