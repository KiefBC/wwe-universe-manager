use serial_test::serial;

use wwe_universe_manager_lib::db::internal_create_wrestler;

mod test_helpers;
use test_helpers::*;

#[test]
#[serial]
fn test_create_wrestler_success() {
    let test_data = TestData::new();
    let new_wrestler = create_test_wrestler();

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(&new_wrestler.name);

    let mut conn = test_data.get_connection();
    let wrestler = internal_create_wrestler(
        &mut conn,
        &new_wrestler.name,
        &new_wrestler.gender,
        new_wrestler.wins,
        new_wrestler.losses,
    )
    .expect("Failed to create wrestler");

    assert_eq!(wrestler.name, new_wrestler.name);
    assert_eq!(wrestler.gender, new_wrestler.gender);
    assert_eq!(wrestler.wins, new_wrestler.wins);
    assert_eq!(wrestler.losses, new_wrestler.losses);
    assert!(wrestler.id > 0);

    // Cleanup
    test_data.cleanup_wrestlers(&new_wrestler.name);
}

#[test]
#[serial]
fn test_create_wrestler_with_wins_and_losses() {
    let test_data = TestData::new();
    let wrestler_name = "Veteran Wrestler";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Female", 15, 5)
        .expect("Failed to create wrestler");

    assert_eq!(wrestler.name, wrestler_name);
    assert_eq!(wrestler.gender, "Female");
    assert_eq!(wrestler.wins, 15);
    assert_eq!(wrestler.losses, 5);

    // Cleanup
    test_data.cleanup_wrestlers(wrestler_name);
}

#[test]
#[serial]
fn test_create_multiple_wrestlers() {
    let test_data = TestData::new();
    let wrestler1_name = "Wrestler One";
    let wrestler2_name = "Wrestler Two";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler1_name);
    test_data.cleanup_wrestlers(wrestler2_name);

    let mut conn = test_data.get_connection();

    let wrestler1 = internal_create_wrestler(&mut conn, wrestler1_name, "Male", 10, 2)
        .expect("Failed to create wrestler 1");

    let wrestler2 = internal_create_wrestler(&mut conn, wrestler2_name, "Female", 8, 3)
        .expect("Failed to create wrestler 2");

    assert_ne!(wrestler1.id, wrestler2.id);
    assert_eq!(wrestler1.name, wrestler1_name);
    assert_eq!(wrestler2.name, wrestler2_name);

    // Cleanup
    test_data.cleanup_wrestlers(wrestler1_name);
    test_data.cleanup_wrestlers(wrestler2_name);
}

#[test]
#[serial]
fn test_create_wrestler_with_different_genders() {
    let test_data = TestData::new();

    let genders = ["Male", "Female", "Other"];

    for (i, gender) in genders.iter().enumerate() {
        let wrestler_name = format!("Test Wrestler {}", i);

        // Cleanup any existing test data
        test_data.cleanup_wrestlers(&wrestler_name);

        let mut conn = test_data.get_connection();
        let wrestler = internal_create_wrestler(&mut conn, &wrestler_name, gender, 0, 0)
            .expect("Failed to create wrestler");

        assert_eq!(wrestler.gender, *gender);

        // Cleanup
        test_data.cleanup_wrestlers(&wrestler_name);
    }
}

#[test]
#[serial]
fn test_create_wrestler_negative_stats_should_work() {
    let test_data = TestData::new();
    let wrestler_name = "Negative Stats Wrestler";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();

    // Note: Depending on your business logic, you might want to prevent negative stats
    // This test assumes they are allowed, adjust as needed
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Male", -1, -1)
        .expect("Failed to create wrestler with negative stats");

    assert_eq!(wrestler.wins, -1);
    assert_eq!(wrestler.losses, -1);

    // Cleanup
    test_data.cleanup_wrestlers(wrestler_name);
}
