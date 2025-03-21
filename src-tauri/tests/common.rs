// Common test utilities and setup functions
use diesel::prelude::*;
use log::info;

// Import from the crate we're testing - only what we actually use
use wwe_universe_manager_lib::db::{establish_connection};
use wwe_universe_manager_lib::models::{NewTitle, NewUser, NewWrestler, Title, User, Wrestler};

// Helper function to reset and establish the connection
#[allow(dead_code)]
pub fn setup_test_user<'a>() -> (SqliteConnection, NewUser<'a>) {
    let mut conn = establish_connection();
    let test_user = NewUser {
        username: "Testing",
        password: "Testing",
    };

    reset_test_user(&mut conn, &test_user);
    (conn, test_user)
}

#[allow(dead_code)]
pub fn setup_test_wrestler<'a>() -> (SqliteConnection, NewWrestler<'a>) {
    let mut conn = establish_connection();
    let test_wrestler = NewWrestler {
        name: "Testing",
        gender: "Male Test",
    };

    reset_test_wrestler(&mut conn, &test_wrestler);
    (conn, test_wrestler)
}

#[allow(dead_code)]
pub fn setup_test_belt<'a>() -> (SqliteConnection, NewTitle<'a>) {
    let mut conn = establish_connection();
    let test_belt = NewTitle { name: "Testing" };

    reset_test_belt(&mut conn, &test_belt);
    (conn, test_belt)
}

// Resets the test user by deleting it if it exists
#[allow(dead_code)]
fn reset_test_user(conn: &mut SqliteConnection, test_user: &NewUser) {
    // Import DSL items locally to avoid naming conflicts
    use wwe_universe_manager_lib::schema::users::dsl::{password, username, users};

    if let Ok(user) = users
        .filter(username.eq(test_user.username))
        .filter(password.eq(test_user.password))
        .first::<User>(conn)
    {
        println!("Deleting user: {:?}", user);
    }

    let result = diesel::delete(users.filter(username.eq(test_user.username)))
        .filter(password.eq(test_user.password))
        .execute(conn)
        .expect("Error deleting test user");

    info!("Deleted {} user", result);
}

#[allow(dead_code)]
fn reset_test_wrestler(conn: &mut SqliteConnection, test_wrestler: &NewWrestler) {
    // Import DSL items locally
    use wwe_universe_manager_lib::schema::wrestlers::dsl::{name as wrestler_name, wrestlers};

    if let Ok(wrestler) = wrestlers
        .filter(wrestler_name.eq(test_wrestler.name))
        .first::<Wrestler>(conn)
    {
        println!("Deleting wrestler: {:?}", wrestler);
    }

    let result = diesel::delete(wrestlers.filter(wrestler_name.eq(test_wrestler.name)))
        .execute(conn)
        .expect("Error deleting test wrestler");

    info!("Deleted {} wrestler", result);
}

#[allow(dead_code)]
fn reset_test_belt(conn: &mut SqliteConnection, test_belt: &NewTitle) {
    // Import DSL items locally
    use wwe_universe_manager_lib::schema::titles::dsl::{name as belt_name, titles};

    if let Ok(belt) = titles
        .filter(belt_name.eq(test_belt.name))
        .first::<Title>(conn)
    {
        println!("Deleting belt: {:?}", belt);
    }

    let result = diesel::delete(titles.filter(belt_name.eq(test_belt.name)))
        .execute(conn)
        .expect("Error deleting test belt");

    info!("Deleted {} belt", result);
}
