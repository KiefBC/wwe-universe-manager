use serial_test::serial;

use wwe_universe_manager_lib::auth::{check_user_exists, internal_verify_credentials};
use wwe_universe_manager_lib::db::internal_create_user;

mod test_helpers;
use test_helpers::*;

#[test]
#[ignore]
#[serial]
fn test_create_user_success() {
    let test_data = TestData::new();
    let new_user = create_test_user();

    // Cleanup any existing test data
    test_data.cleanup_users(&new_user.username);

    let mut conn = test_data.get_connection();
    let user = internal_create_user(&mut conn, &new_user.username, &new_user.password)
        .expect("Failed to create user");

    assert_eq!(user.username, new_user.username);
    assert_eq!(user.password, new_user.password);
    assert!(user.id > 0);

    // Cleanup
    test_data.cleanup_users(&new_user.username);
}

#[test]
#[ignore]
#[serial]
fn test_create_duplicate_user_fails() {
    let test_data = TestData::new();
    let new_user = create_test_user();

    // Cleanup any existing test data
    test_data.cleanup_users(&new_user.username);

    let mut conn = test_data.get_connection();

    // Create first user
    internal_create_user(&mut conn, &new_user.username, &new_user.password)
        .expect("Failed to create first user");

    // Try to create duplicate user - should fail due to unique constraint
    let result = internal_create_user(&mut conn, &new_user.username, &new_user.password);
    assert!(result.is_err(), "Creating duplicate user should fail");

    // Cleanup
    test_data.cleanup_users(&new_user.username);
}

#[test]
#[ignore]
#[serial]
fn test_check_user_exists() {
    let test_data = TestData::new();
    let new_user = create_test_user();

    // Cleanup any existing test data
    test_data.cleanup_users(&new_user.username);

    let mut conn = test_data.get_connection();

    // User should not exist initially
    assert!(!check_user_exists(&mut conn, &new_user.username));

    // Create user
    internal_create_user(&mut conn, &new_user.username, &new_user.password)
        .expect("Failed to create user");

    // User should now exist
    assert!(check_user_exists(&mut conn, &new_user.username));

    // Non-existent user should return false
    assert!(!check_user_exists(&mut conn, "non_existent_user"));

    // Cleanup
    test_data.cleanup_users(&new_user.username);
}

#[test]
#[ignore]
#[serial]
fn test_verify_credentials() {
    let test_data = TestData::new();
    let new_user = create_test_user();

    // Cleanup any existing test data
    test_data.cleanup_users(&new_user.username);

    let mut conn = test_data.get_connection();

    // Create user
    internal_create_user(&mut conn, &new_user.username, &new_user.password)
        .expect("Failed to create user");

    // Valid credentials should work
    assert!(internal_verify_credentials(
        &mut conn,
        &new_user.username,
        &new_user.password
    ));

    // Invalid password should fail
    assert!(!internal_verify_credentials(
        &mut conn,
        &new_user.username,
        "wrong_password"
    ));

    // Invalid username should fail
    assert!(!internal_verify_credentials(
        &mut conn,
        "wrong_user",
        &new_user.password
    ));

    // Both invalid should fail
    assert!(!internal_verify_credentials(
        &mut conn,
        "wrong_user",
        "wrong_password"
    ));

    // Cleanup
    test_data.cleanup_users(&new_user.username);
}

#[test]
#[ignore]
#[serial]
fn test_create_user_with_empty_fields() {
    let test_data = TestData::new();
    let mut conn = test_data.get_connection();

    // Test empty username
    let _result = internal_create_user(&mut conn, "", "password");
    // Note: This might succeed or fail depending on your constraints
    // Adjust assertion based on your business logic

    // Test empty password
    let _result = internal_create_user(&mut conn, "username", "");
    // Note: This might succeed or fail depending on your constraints
    // Adjust assertion based on your business logic

    // Cleanup any created users
    test_data.cleanup_users("");
    test_data.cleanup_users("username");
}
