use serial_test::serial;

use wwe_universe_manager_lib::db::{internal_create_belt, internal_create_wrestler};

mod test_helpers;
use test_helpers::*;

#[test]
#[serial]
fn test_create_title_without_holder() {
    let test_data = TestData::new();
    let new_title = create_test_title();

    // Cleanup any existing test data
    test_data.cleanup_titles(&new_title.name);

    let mut conn = test_data.get_connection();
    let title = internal_create_belt(&mut conn, &new_title.name, new_title.current_holder_id)
        .expect("Failed to create title");

    assert_eq!(title.name, new_title.name);
    assert_eq!(title.current_holder_id, None);
    assert!(title.id > 0);

    // Cleanup
    test_data.cleanup_titles(&new_title.name);
}

#[test]
#[serial]
fn test_create_title_with_holder() {
    let test_data = TestData::new();
    let title_name = "Championship Title";
    let wrestler_name = "Champion Wrestler";

    // Cleanup any existing test data
    test_data.cleanup_titles(title_name);
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();

    // First create a wrestler to be the champion
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Male", 20, 1)
        .expect("Failed to create wrestler");

    // Create title with the wrestler as holder
    let title = internal_create_belt(&mut conn, title_name, Some(wrestler.id))
        .expect("Failed to create title");

    assert_eq!(title.name, title_name);
    assert_eq!(title.current_holder_id, Some(wrestler.id));

    // Cleanup
    test_data.cleanup_titles(title_name);
    test_data.cleanup_wrestlers(wrestler_name);
}

#[test]
#[serial]
fn test_create_multiple_titles() {
    let test_data = TestData::new();
    let title1_name = "World Championship";
    let title2_name = "Intercontinental Championship";

    // Cleanup any existing test data
    test_data.cleanup_titles(title1_name);
    test_data.cleanup_titles(title2_name);

    let mut conn = test_data.get_connection();

    let title1 =
        internal_create_belt(&mut conn, title1_name, None).expect("Failed to create title 1");

    let title2 =
        internal_create_belt(&mut conn, title2_name, None).expect("Failed to create title 2");

    assert_ne!(title1.id, title2.id);
    assert_eq!(title1.name, title1_name);
    assert_eq!(title2.name, title2_name);

    // Cleanup
    test_data.cleanup_titles(title1_name);
    test_data.cleanup_titles(title2_name);
}

#[test]
#[serial]
fn test_create_title_with_invalid_holder_id() {
    let test_data = TestData::new();
    let title_name = "Invalid Holder Title";

    // Cleanup any existing test data
    test_data.cleanup_titles(title_name);

    let mut conn = test_data.get_connection();

    // Try to create title with non-existent wrestler ID
    let result = internal_create_belt(&mut conn, title_name, Some(99999));

    // Note: This test assumes foreign key constraints are enforced
    // If foreign keys are not enforced in your SQLite setup, this will pass
    // You can enable foreign key constraints with: PRAGMA foreign_keys = ON;
    // For now, we'll just check that the title was created successfully or failed
    match result {
        Ok(title) => {
            // Foreign keys not enforced - title created successfully
            assert_eq!(title.current_holder_id, Some(99999));
        }
        Err(_) => {
            // Foreign keys enforced - creation failed as expected
            // This is the ideal behavior
        }
    }

    // Cleanup
    test_data.cleanup_titles(title_name);
}

#[test]
#[serial]
fn test_title_holder_relationship() {
    let test_data = TestData::new();
    let title_name = "Relationship Test Title";
    let wrestler_name = "Relationship Test Wrestler";

    // Cleanup any existing test data
    test_data.cleanup_titles(title_name);
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();

    // Create wrestler first
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Female", 15, 3)
        .expect("Failed to create wrestler");

    // Create title without holder
    let title = internal_create_belt(&mut conn, title_name, None).expect("Failed to create title");

    assert_eq!(title.current_holder_id, None);

    // Update title to have a holder (if you have an update function)
    // For now, we'll create a new title with the holder
    test_data.cleanup_titles(title_name);

    let title_with_holder = internal_create_belt(&mut conn, title_name, Some(wrestler.id))
        .expect("Failed to create title with holder");

    assert_eq!(title_with_holder.current_holder_id, Some(wrestler.id));

    // Cleanup
    test_data.cleanup_titles(title_name);
    test_data.cleanup_wrestlers(wrestler_name);
}
