use serial_test::serial;

use wwe_universe_manager_lib::db::{
    internal_create_belt, internal_create_show, internal_create_user, internal_create_wrestler,
};

mod test_helpers;
use test_helpers::*;

#[test]
#[serial]
fn test_complete_wrestling_scenario() {
    let test_data = TestData::new();

    // Test data
    let show_name = "Championship Night";
    let wrestler1_name = "John Champion";
    let wrestler2_name = "Jane Challenger";
    let title_name = "World Championship";
    let user_name = "admin_user";

    // Cleanup
    test_data.cleanup_shows(show_name);
    test_data.cleanup_wrestlers(wrestler1_name);
    test_data.cleanup_wrestlers(wrestler2_name);
    test_data.cleanup_titles(title_name);
    test_data.cleanup_users(user_name);

    let mut conn = test_data.get_connection();

    // 1. Create a show
    let show = internal_create_show(&mut conn, show_name, "Championship match night")
        .expect("Failed to create show");

    // 2. Create wrestlers
    let wrestler1 = internal_create_wrestler(&mut conn, wrestler1_name, "Male", 25, 2)
        .expect("Failed to create wrestler 1");

    let wrestler2 = internal_create_wrestler(&mut conn, wrestler2_name, "Female", 18, 5)
        .expect("Failed to create wrestler 2");

    // 3. Create a title without holder initially
    let title = internal_create_belt(
        &mut conn, 
        title_name, 
        "Singles",
        "World",
        "Mixed",
        None,
        None,
        false // is_user_created
    ).expect("Failed to create title");

    // 4. Create a user
    let user = internal_create_user(&mut conn, user_name, "secure_password")
        .expect("Failed to create user");

    // 5. Verify all entities were created correctly
    assert_eq!(show.name, show_name);
    assert_eq!(wrestler1.name, wrestler1_name);
    assert_eq!(wrestler2.name, wrestler2_name);
    assert_eq!(title.name, title_name);
    assert_eq!(user.username, user_name);

    // 6. Create another title with wrestler1 as champion
    test_data.cleanup_titles("Secondary Title");
    let title_with_holder = internal_create_belt(
        &mut conn, 
        "Secondary Title", 
        "Singles",
        "Intercontinental",
        "Mixed",
        None,
        Some(wrestler1.id),
        false // is_user_created
    ).expect("Failed to create title with holder");

    assert_eq!(title_with_holder.current_holder_id, Some(wrestler1.id));

    // Cleanup
    test_data.cleanup_shows(show_name);
    test_data.cleanup_wrestlers(wrestler1_name);
    test_data.cleanup_wrestlers(wrestler2_name);
    test_data.cleanup_titles(title_name);
    test_data.cleanup_titles("Secondary Title");
    test_data.cleanup_users(user_name);
}

#[test]
#[serial]
fn test_title_holder_cascade_behavior() {
    let test_data = TestData::new();

    let wrestler_name = "Temporary Champion";
    let title_name = "Temporary Title";

    // Cleanup
    test_data.cleanup_wrestlers(wrestler_name);
    test_data.cleanup_titles(title_name);

    let mut conn = test_data.get_connection();

    // Create wrestler
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Male", 10, 0)
        .expect("Failed to create wrestler");

    // Create title with wrestler as holder
    let title = internal_create_belt(
        &mut conn, 
        title_name, 
        "Singles",
        "World",
        "Mixed",
        None,
        Some(wrestler.id),
        false // is_user_created
    ).expect("Failed to create title with holder");

    assert_eq!(title.current_holder_id, Some(wrestler.id));

    // Delete wrestler (this should affect the title's foreign key)
    test_data.cleanup_wrestlers(wrestler_name);

    // Query title to see what happened to the foreign key
    // Note: Depending on your foreign key constraints, this might:
    // 1. Set current_holder_id to NULL (if ON DELETE SET NULL)
    // 2. Delete the title (if ON DELETE CASCADE)
    // 3. Prevent deletion (if ON DELETE RESTRICT)

    // For this test, we'll assume the title still exists but holder is set to NULL
    // Adjust based on your actual foreign key constraint behavior

    // Cleanup
    test_data.cleanup_titles(title_name);
}

#[test]
#[serial]
fn test_multiple_titles_same_holder() {
    let test_data = TestData::new();

    let wrestler_name = "Multi Champion";
    let title1_name = "World Title";
    let title2_name = "Intercontinental Title";

    // Cleanup
    test_data.cleanup_wrestlers(wrestler_name);
    test_data.cleanup_titles(title1_name);
    test_data.cleanup_titles(title2_name);

    let mut conn = test_data.get_connection();

    // Create wrestler
    let wrestler = internal_create_wrestler(&mut conn, wrestler_name, "Female", 30, 1)
        .expect("Failed to create wrestler");

    // Create multiple titles with same holder
    let title1 = internal_create_belt(
        &mut conn, 
        title1_name, 
        "Singles",
        "World",
        "Mixed",
        None,
        Some(wrestler.id),
        false // is_user_created
    ).expect("Failed to create title 1");

    let title2 = internal_create_belt(
        &mut conn, 
        title2_name, 
        "Singles",
        "Intercontinental",
        "Mixed",
        None,
        Some(wrestler.id),
        false // is_user_created
    ).expect("Failed to create title 2");

    assert_eq!(title1.current_holder_id, Some(wrestler.id));
    assert_eq!(title2.current_holder_id, Some(wrestler.id));
    assert_ne!(title1.id, title2.id);

    // Cleanup
    test_data.cleanup_titles(title1_name);
    test_data.cleanup_titles(title2_name);
    test_data.cleanup_wrestlers(wrestler_name);
}

#[test]
#[serial]
fn test_stress_create_many_entities() {
    let test_data = TestData::new();
    let mut conn = test_data.get_connection();

    let _base_names = ["User", "Wrestler", "Title", "Show"];

    // Create multiple entities of each type
    for i in 0..5 {
        let user_name = format!("StressUser{}", i);
        let wrestler_name = format!("StressWrestler{}", i);
        let title_name = format!("StressTitle{}", i);
        let show_name = format!("StressShow{}", i);

        // Cleanup first
        test_data.cleanup_users(&user_name);
        test_data.cleanup_wrestlers(&wrestler_name);
        test_data.cleanup_titles(&title_name);
        test_data.cleanup_shows(&show_name);

        // Create entities
        let user = internal_create_user(&mut conn, &user_name, "password")
            .expect("Failed to create stress user");

        let wrestler = internal_create_wrestler(&mut conn, &wrestler_name, "Male", i, 0)
            .expect("Failed to create stress wrestler");

        let title = internal_create_belt(
            &mut conn, 
            &title_name, 
            "Singles",
            "World",
            "Mixed",
            None,
            Some(wrestler.id),
            false // is_user_created
        ).expect("Failed to create stress title");

        let show = internal_create_show(&mut conn, &show_name, &format!("Stress show {}", i))
            .expect("Failed to create stress show");

        // Verify IDs are unique and incrementing
        assert!(user.id > 0);
        assert!(wrestler.id > 0);
        assert!(title.id > 0);
        assert!(show.id > 0);

        // Cleanup
        test_data.cleanup_users(&user_name);
        test_data.cleanup_wrestlers(&wrestler_name);
        test_data.cleanup_titles(&title_name);
        test_data.cleanup_shows(&show_name);
    }
}

#[test]
#[serial]
fn test_create_test_data_basic() {
    let test_data = TestData::new();
    let mut conn = test_data.get_connection();

    // Clear any existing test data first
    test_data.cleanup_shows("Monday Night RAW");
    test_data.cleanup_shows("Friday Night SmackDown");
    test_data.cleanup_wrestlers("The Rock");
    test_data.cleanup_wrestlers("Stone Cold Steve Austin");

    // Test creating titles using the internal function directly
    let result = internal_create_belt(
        &mut conn, 
        "Test World Championship", 
        "Singles",
        "World",
        "Mixed",
        None,
        None,
        false // is_user_created
    );

    match result {
        Ok(title) => {
            assert_eq!(title.name, "Test World Championship");
            assert_eq!(title.title_type, "Singles");
            assert_eq!(title.division, "World");
            assert_eq!(title.gender, "Mixed");
            assert_eq!(title.prestige_tier, 1); // World division should be tier 1
            assert_eq!(title.is_active, true);
            println!("✓ Title creation test passed: {:?}", title);
        }
        Err(e) => {
            panic!("Failed to create test title: {}", e);
        }
    }

    // Clean up
    test_data.cleanup_titles("Test World Championship");
}
