use serial_test::serial;

// Import directly from the library
extern crate wwe_universe_manager_lib;
use wwe_universe_manager_lib::auth::{check_user_exists, verify_credentials};
use wwe_universe_manager_lib::db::create_user;

// Import the common module functions
mod common;
use common::setup_test_user;

#[test]
#[serial]
// Test to create a new user
fn test_create_user() {
    let (mut conn, new_user) = setup_test_user();
    let user = create_user(&mut conn, new_user).expect("User not created");

    assert_eq!(user.username, "Testing");
    assert_eq!(user.password, "Testing");
}

#[test]
#[serial]
// Test to check if user exists
fn test_check_user_exists() {
    let (mut conn, new_user) = setup_test_user();
    create_user(&mut conn, new_user);

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
    let (mut conn, new_user) = setup_test_user();
    create_user(&mut conn, new_user);

    let username_check: &str = "Testing";
    let password_check: &str = "Testing";
    let password_check1: &str = "Testing1";

    assert!(verify_credentials(
        username_check.to_string(),
        password_check.to_string()
    ));
    assert!(!verify_credentials(
        username_check.to_string(),
        password_check1.to_string()
    ));
}
