use serial_test::serial;

use wwe_universe_manager_lib::db::{internal_create_wrestler, internal_create_enhanced_wrestler, internal_create_signature_move, internal_get_wrestlers};

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

#[test]
#[serial]
fn test_create_enhanced_wrestler_success() {
    let test_data = TestData::new();
    let new_wrestler = create_test_enhanced_wrestler();

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(&new_wrestler.name);

    let mut conn = test_data.get_connection();
    let wrestler = internal_create_enhanced_wrestler(
        &mut conn,
        &new_wrestler.name,
        new_wrestler.real_name.as_ref().unwrap(),
        new_wrestler.nickname.as_ref().unwrap(),
        &new_wrestler.gender,
        new_wrestler.wins,
        new_wrestler.losses,
        new_wrestler.height.as_ref().unwrap(),
        new_wrestler.weight.as_ref().unwrap(),
        new_wrestler.debut_year.unwrap(),
        new_wrestler.strength.unwrap(),
        new_wrestler.speed.unwrap(),
        new_wrestler.agility.unwrap(),
        new_wrestler.stamina.unwrap(),
        new_wrestler.charisma.unwrap(),
        new_wrestler.technique.unwrap(),
        new_wrestler.biography.as_ref().unwrap(),
        false, // is_user_created
    )
    .expect("Failed to create enhanced wrestler");

    assert_eq!(wrestler.name, new_wrestler.name);
    assert_eq!(wrestler.gender, new_wrestler.gender);
    assert_eq!(wrestler.wins, new_wrestler.wins);
    assert_eq!(wrestler.losses, new_wrestler.losses);
    assert_eq!(wrestler.real_name, new_wrestler.real_name);
    assert_eq!(wrestler.nickname, new_wrestler.nickname);
    assert_eq!(wrestler.height, new_wrestler.height);
    assert_eq!(wrestler.weight, new_wrestler.weight);
    assert_eq!(wrestler.debut_year, new_wrestler.debut_year);
    // Promotion field removed - wrestlers are now global entities
    assert_eq!(wrestler.strength, new_wrestler.strength);
    assert_eq!(wrestler.speed, new_wrestler.speed);
    assert_eq!(wrestler.agility, new_wrestler.agility);
    assert_eq!(wrestler.stamina, new_wrestler.stamina);
    assert_eq!(wrestler.charisma, new_wrestler.charisma);
    assert_eq!(wrestler.technique, new_wrestler.technique);
    assert_eq!(wrestler.biography, new_wrestler.biography);
    assert!(wrestler.id > 0);

    // Cleanup
    test_data.cleanup_wrestlers(&new_wrestler.name);
}

#[test]
#[serial]
fn test_create_signature_move_success() {
    let test_data = TestData::new();
    let wrestler_name = "Wrestler with Moves";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();
    
    // First create a wrestler
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Male", 0, 0)
        .expect("Failed to create wrestler");

    // Then create a signature move for them
    let signature_move = internal_create_signature_move(
        &mut conn,
        wrestler.id,
        "Test Finisher",
        "primary"
    )
    .expect("Failed to create signature move");

    assert_eq!(signature_move.wrestler_id, wrestler.id);
    assert_eq!(signature_move.move_name, "Test Finisher");
    assert_eq!(signature_move.move_type, "primary");
    assert!(signature_move.id.is_some());

    // Cleanup
    test_data.cleanup_signature_moves(wrestler.id);
    test_data.cleanup_wrestlers(wrestler_name);
}

#[test]
#[serial]
fn test_create_multiple_signature_moves() {
    let test_data = TestData::new();
    let wrestler_name = "Wrestler with Multiple Moves";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();
    
    // First create a wrestler
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Female", 10, 3)
        .expect("Failed to create wrestler");

    // Create multiple signature moves
    let moves = vec![
        ("Primary Finisher", "primary"),
        ("Secondary Finisher", "primary"),
        ("Setup Move", "secondary"),
        ("Signature Strike", "secondary"),
    ];

    let mut created_moves = Vec::new();
    for (move_name, move_type) in moves {
        let signature_move = internal_create_signature_move(
            &mut conn,
            wrestler.id,
            move_name,
            move_type
        )
        .expect("Failed to create signature move");
        
        assert_eq!(signature_move.wrestler_id, wrestler.id);
        assert_eq!(signature_move.move_name, move_name);
        assert_eq!(signature_move.move_type, move_type);
        created_moves.push(signature_move);
    }

    // Verify all moves were created with different IDs
    assert_eq!(created_moves.len(), 4);
    for i in 0..created_moves.len() {
        for j in (i+1)..created_moves.len() {
            assert_ne!(created_moves[i].id, created_moves[j].id);
        }
    }

    // Cleanup
    test_data.cleanup_signature_moves(wrestler.id);
    test_data.cleanup_wrestlers(wrestler_name);
}

#[test]
#[serial]
fn test_enhanced_wrestler_with_optional_fields() {
    let test_data = TestData::new();
    let wrestler_name = "Minimal Enhanced Wrestler";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler_name);

    let mut conn = test_data.get_connection();
    
    // Create an enhanced wrestler with minimal optional data
    let wrestler = internal_create_enhanced_wrestler(
        &mut conn,
        wrestler_name,
        "Real Name",
        "Nickname",
        "Other",
        12,
        8,
        "5'8\"",
        "175 lbs",
        2018,
        5,
        9,
        7,
        6,
        8,
        9,
        "Short biography.",
        false // is_user_created
    )
    .expect("Failed to create enhanced wrestler");

    assert_eq!(wrestler.name, wrestler_name);
    assert_eq!(wrestler.gender, "Other");
    assert_eq!(wrestler.wins, 12);
    assert_eq!(wrestler.losses, 8);
    assert_eq!(wrestler.real_name, Some("Real Name".to_string()));
    // Promotion field removed - wrestlers are now global entities
    assert_eq!(wrestler.technique, Some(9));

    // Cleanup
    test_data.cleanup_wrestlers(wrestler_name);
}

#[test]
#[serial]
fn test_fetch_wrestlers() {
    let test_data = TestData::new();
    let wrestler1_name = "Fetch Test Wrestler 1";
    let wrestler2_name = "Fetch Test Wrestler 2";
    let wrestler3_name = "Fetch Test Wrestler 3";

    // Cleanup any existing test data
    test_data.cleanup_wrestlers(wrestler1_name);
    test_data.cleanup_wrestlers(wrestler2_name);
    test_data.cleanup_wrestlers(wrestler3_name);

    let mut conn = test_data.get_connection();

    // Test fetching empty list first
    let initial_wrestlers = internal_get_wrestlers(&mut conn)
        .expect("Failed to fetch wrestlers initially");
    let initial_count = initial_wrestlers.len();

    // Create test wrestlers
    let wrestler1 = internal_create_wrestler(&mut conn, wrestler1_name, "Male", 5, 1)
        .expect("Failed to create wrestler 1");
    
    let wrestler2 = internal_create_wrestler(&mut conn, wrestler2_name, "Female", 8, 2)
        .expect("Failed to create wrestler 2");
        
    let wrestler3 = internal_create_wrestler(&mut conn, wrestler3_name, "Other", 3, 0)
        .expect("Failed to create wrestler 3");

    // Fetch all wrestlers
    let all_wrestlers = internal_get_wrestlers(&mut conn)
        .expect("Failed to fetch wrestlers");

    // Should have initial count + 3 new wrestlers
    assert_eq!(all_wrestlers.len(), initial_count + 3);
    
    // Find our test wrestlers in the results (they should be ordered by ID)
    let found_wrestler1 = all_wrestlers.iter().find(|w| w.name == wrestler1_name);
    let found_wrestler2 = all_wrestlers.iter().find(|w| w.name == wrestler2_name);
    let found_wrestler3 = all_wrestlers.iter().find(|w| w.name == wrestler3_name);

    assert!(found_wrestler1.is_some(), "Wrestler 1 should be found in fetch results");
    assert!(found_wrestler2.is_some(), "Wrestler 2 should be found in fetch results");
    assert!(found_wrestler3.is_some(), "Wrestler 3 should be found in fetch results");

    // Verify the fetched wrestlers have correct data
    let found1 = found_wrestler1.unwrap();
    assert_eq!(found1.id, wrestler1.id);
    assert_eq!(found1.name, wrestler1_name);
    assert_eq!(found1.gender, "Male");
    assert_eq!(found1.wins, 5);
    assert_eq!(found1.losses, 1);

    let found2 = found_wrestler2.unwrap();
    assert_eq!(found2.id, wrestler2.id);
    assert_eq!(found2.name, wrestler2_name);
    assert_eq!(found2.gender, "Female");
    assert_eq!(found2.wins, 8);
    assert_eq!(found2.losses, 2);

    let found3 = found_wrestler3.unwrap();
    assert_eq!(found3.id, wrestler3.id);
    assert_eq!(found3.name, wrestler3_name);
    assert_eq!(found3.gender, "Other");
    assert_eq!(found3.wins, 3);
    assert_eq!(found3.losses, 0);

    // Verify wrestlers are ordered by ID (ascending)
    let wrestler_ids: Vec<i32> = all_wrestlers.iter().map(|w| w.id).collect();
    let mut sorted_ids = wrestler_ids.clone();
    sorted_ids.sort();
    assert_eq!(wrestler_ids, sorted_ids, "Wrestlers should be ordered by ID ascending");

    // Cleanup
    test_data.cleanup_wrestlers(wrestler1_name);
    test_data.cleanup_wrestlers(wrestler2_name);
    test_data.cleanup_wrestlers(wrestler3_name);
}
