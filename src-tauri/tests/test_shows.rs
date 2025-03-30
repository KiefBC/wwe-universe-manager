use log::info;
use serial_test::serial;

// Import from the correct library crate
// The library is defined in src-tauri with the name wwe_universe_manager_lib
extern crate wwe_universe_manager_lib;
use wwe_universe_manager_lib::db::create_show;

// Import the common module as a separate module
mod common;
use common::setup_test_show;

#[test]
#[serial]
fn test_create_show() {
    let (mut conn, new_show) = setup_test_show();
    info!("Creating new Show");
    let show = create_show(&mut conn, new_show).expect("Show not created");

    assert_eq!(show.name, "Testing");
    assert_ne!(show.name, "Testing1");
}