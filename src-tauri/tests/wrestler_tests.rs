use log::info;
use serial_test::serial;
use wwe_universe_manager_lib::db::internal_create_wrestler;

mod common;
use common::setup_test_wrestler;

#[test]
#[serial]
// Test to create a new Wrestler
fn test_create_wrestler() {
    let (pool, new_wrestler) = setup_test_wrestler();
    info!("Creating new Wrestler");
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get connection from pool");
    // Call the internal function directly
    let wrestler = internal_create_wrestler(&mut conn, &new_wrestler.name, &new_wrestler.gender)
        .expect("Wrestler not created");

    assert_eq!(wrestler.name, "Testing");
    assert_ne!(wrestler.name, "Testing1");
}
