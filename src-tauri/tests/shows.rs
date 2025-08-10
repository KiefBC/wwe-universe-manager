use serial_test::serial;

use wwe_universe_manager_lib::db::{internal_create_show, internal_get_shows, internal_get_wrestlers_for_show};

mod test_helpers;
use test_helpers::*;

#[test]
#[serial]
fn test_create_show_success() {
    let test_data = TestData::new();
    let new_show = create_test_show();

    // Cleanup any existing test data
    test_data.cleanup_shows(&new_show.name);

    let mut conn = test_data.get_connection();
    let show = internal_create_show(&mut conn, &new_show.name, &new_show.description)
        .expect("Failed to create show");

    assert_eq!(show.name, new_show.name);
    assert_eq!(show.description, new_show.description);
    assert!(show.id > 0);

    // Cleanup
    test_data.cleanup_shows(&new_show.name);
}

#[test]
#[serial]
fn test_create_multiple_shows() {
    let test_data = TestData::new();
    let show1_name = "Monday Night Raw";
    let show1_desc = "Monday night wrestling show";
    let show2_name = "SmackDown";
    let show2_desc = "Friday night wrestling show";

    // Cleanup any existing test data
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);

    let mut conn = test_data.get_connection();

    let show1 =
        internal_create_show(&mut conn, show1_name, show1_desc).expect("Failed to create show 1");

    let show2 =
        internal_create_show(&mut conn, show2_name, show2_desc).expect("Failed to create show 2");

    assert_ne!(show1.id, show2.id);
    assert_eq!(show1.name, show1_name);
    assert_eq!(show2.name, show2_name);
    assert_eq!(show1.description, show1_desc);
    assert_eq!(show2.description, show2_desc);

    // Cleanup
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);
}

#[test]
#[serial]
fn test_get_shows() {
    let test_data = TestData::new();
    let show1_name = "Test Show 1";
    let show1_desc = "First test show";
    let show2_name = "Test Show 2";
    let show2_desc = "Second test show";

    // Cleanup any existing test data
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);

    let mut conn = test_data.get_connection();

    // Get initial count
    let initial_shows = internal_get_shows(&mut conn).expect("Failed to get initial shows");
    let initial_count = initial_shows.len();

    // Create test shows
    internal_create_show(&mut conn, show1_name, show1_desc).expect("Failed to create show 1");
    internal_create_show(&mut conn, show2_name, show2_desc).expect("Failed to create show 2");

    // Get all shows
    let shows = internal_get_shows(&mut conn).expect("Failed to get shows");

    assert_eq!(shows.len(), initial_count + 2);

    // Verify our test shows are in the results
    let show1_found = shows
        .iter()
        .any(|s| s.name == show1_name && s.description == show1_desc);
    let show2_found = shows
        .iter()
        .any(|s| s.name == show2_name && s.description == show2_desc);

    assert!(show1_found, "Show 1 not found in results");
    assert!(show2_found, "Show 2 not found in results");

    // Cleanup
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);
}

#[test]
#[serial]
fn test_create_show_with_empty_description() {
    let test_data = TestData::new();
    let show_name = "Show With Empty Description";

    // Cleanup any existing test data
    test_data.cleanup_shows(show_name);

    let mut conn = test_data.get_connection();
    let show = internal_create_show(&mut conn, show_name, "")
        .expect("Failed to create show with empty description");

    assert_eq!(show.name, show_name);
    assert_eq!(show.description, "");

    // Cleanup
    test_data.cleanup_shows(show_name);
}

#[test]
#[serial]
fn test_create_show_with_long_description() {
    let test_data = TestData::new();
    let show_name = "Show With Long Description";
    let long_description = "This is a very long description that goes on and on and on to test how the database handles longer text fields. It should work fine since we're using TEXT type in SQLite which can handle large amounts of text data without any issues.".repeat(5);

    // Cleanup any existing test data
    test_data.cleanup_shows(show_name);

    let mut conn = test_data.get_connection();
    let show = internal_create_show(&mut conn, show_name, &long_description)
        .expect("Failed to create show with long description");

    assert_eq!(show.name, show_name);
    assert_eq!(show.description, long_description);

    // Cleanup
    test_data.cleanup_shows(show_name);
}

#[test]
#[serial]
fn test_shows_are_ordered_by_id() {
    let test_data = TestData::new();
    let show1_name = "Ordered Show 1";
    let show2_name = "Ordered Show 2";
    let show3_name = "Ordered Show 3";

    // Cleanup any existing test data
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);
    test_data.cleanup_shows(show3_name);

    let mut conn = test_data.get_connection();

    // Create shows in sequence
    let show1 =
        internal_create_show(&mut conn, show1_name, "First show").expect("Failed to create show 1");
    let show2 = internal_create_show(&mut conn, show2_name, "Second show")
        .expect("Failed to create show 2");
    let show3 =
        internal_create_show(&mut conn, show3_name, "Third show").expect("Failed to create show 3");

    // Get all shows
    let shows = internal_get_shows(&mut conn).expect("Failed to get shows");

    // Find positions of our test shows
    let pos1 = shows.iter().position(|s| s.id == show1.id);
    let pos2 = shows.iter().position(|s| s.id == show2.id);
    let pos3 = shows.iter().position(|s| s.id == show3.id);

    assert!(pos1.is_some());
    assert!(pos2.is_some());
    assert!(pos3.is_some());

    // Verify they are ordered by ID (ascending)
    assert!(pos1.unwrap() < pos2.unwrap());
    assert!(pos2.unwrap() < pos3.unwrap());

    // Cleanup
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);
    test_data.cleanup_shows(show3_name);
}

#[test]
#[serial]
fn test_get_shows_for_wrestler() {
    use wwe_universe_manager_lib::db::{internal_create_wrestler, internal_assign_wrestler_to_show, internal_get_shows_for_wrestler};
    
    let test_data = TestData::new();
    let show1_name = "Test Show A";
    let show1_desc = "First test show for wrestler assignment";
    let show2_name = "Test Show B"; 
    let show2_desc = "Second test show for wrestler assignment";
    let wrestler_name = "Test Wrestler Show Assignment";

    // Cleanup any existing test data
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();

    // Create test shows and wrestler
    let show1 = internal_create_show(&mut conn, show1_name, show1_desc)
        .expect("Failed to create show 1");
    let show2 = internal_create_show(&mut conn, show2_name, show2_desc)
        .expect("Failed to create show 2");
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Mixed", 0, 0)
        .expect("Failed to create wrestler");

    // Initially, wrestler should not be assigned to any shows
    let initial_shows = internal_get_shows_for_wrestler(&mut conn, wrestler.id)
        .expect("Failed to get initial shows for wrestler");
    assert_eq!(initial_shows.len(), 0);

    // Assign wrestler to first show
    internal_assign_wrestler_to_show(&mut conn, show1.id, wrestler.id)
        .expect("Failed to assign wrestler to show 1");

    let shows_after_first = internal_get_shows_for_wrestler(&mut conn, wrestler.id)
        .expect("Failed to get shows after first assignment");
    assert_eq!(shows_after_first.len(), 1);
    assert_eq!(shows_after_first[0].id, show1.id);
    assert_eq!(shows_after_first[0].name, show1_name);

    // Assign wrestler to second show
    internal_assign_wrestler_to_show(&mut conn, show2.id, wrestler.id)
        .expect("Failed to assign wrestler to show 2");

    let shows_after_second = internal_get_shows_for_wrestler(&mut conn, wrestler.id)
        .expect("Failed to get shows after second assignment");
    
    // With exclusive assignment logic, wrestler should only be on one show (the latest assignment)
    assert_eq!(shows_after_second.len(), 1);
    assert_eq!(shows_after_second[0].id, show2.id);
    assert_eq!(shows_after_second[0].name, show2_name);
    
    // Verify wrestler was transferred from show1 (should not be in show1's roster anymore)
    let show1_roster = internal_get_wrestlers_for_show(&mut conn, show1.id)
        .expect("Failed to get show1 roster after transfer");
    assert!(!show1_roster.iter().any(|w| w.id == wrestler.id), 
           "Wrestler should be transferred from show1");

    // Cleanup
    test_data.cleanup_shows(show1_name);
    test_data.cleanup_shows(show2_name);
    test_data.cleanup_wrestlers(wrestler_name);
}
