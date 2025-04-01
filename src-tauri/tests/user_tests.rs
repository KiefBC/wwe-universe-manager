use serial_test::serial;

// Import directly from the library
extern crate wwe_universe_manager_lib;
use wwe_universe_manager_lib::auth::{check_user_exists, internal_verify_credentials};
use wwe_universe_manager_lib::db::internal_create_user;

// Import the common module functions
mod common;
use common::setup_test_user;

#[test]
#[serial]
// Test to create a new user
fn test_create_user() {
    let (pool, new_user) = setup_test_user();
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get connection from pool");
    let user = internal_create_user(&mut conn, &new_user.username, &new_user.password).expect("User not created");

    assert_eq!(user.username, "Testing");
    assert_eq!(user.password, "Testing");
}

#[test]
#[serial]
// Test to check if user exists
fn test_check_user_exists() {
    let (pool, new_user) = setup_test_user();
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get connection from pool");
    internal_create_user(&mut conn, &new_user.username, &new_user.password).expect("User creation failed");

    let username_check: &str = "Testing";
    let username_check1: &str = "Testing1";

    let result = check_user_exists(&mut conn, username_check);
    let result1 = check_user_exists(&mut conn, username_check1);

    assert!(result);
    assert!(!result1);
}

#[test]
#[serial]
// Test to verify credentials of a user
fn test_verify_credentials() {
    let (pool, new_user) = setup_test_user();
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get connection from pool");
    internal_create_user(&mut conn, &new_user.username, &new_user.password).expect("User creation failed");

    let username_check: &str = "Testing";
    let password_check: &str = "Testing";
    let password_check1: &str = "Testing1";

    assert!(internal_verify_credentials(
        &mut conn,
        username_check,
        password_check
    ));
    assert!(!internal_verify_credentials(
        &mut conn,
        username_check,
        password_check1
    ));
}
