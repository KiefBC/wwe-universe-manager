// Common test utilities and setup functions
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use log::info;

// Import from the crate we're testing - only what we actually use
use wwe_universe_manager_lib::db::establish_connection;
use wwe_universe_manager_lib::models::{NewTitle, NewUser, NewWrestler, Title, User, Wrestler, NewShow, Show};

// Helper function to reset and establish the connection
#[allow(dead_code)]
pub fn setup_test_user() -> (Pool<ConnectionManager<SqliteConnection>>, NewUser) {
    let pool = establish_connection();
    let test_user = NewUser {
        username: "Testing".to_string(),
        password: "Testing".to_string(),
    };

    let mut conn = pool.get().expect("Failed to get connection from pool");
    reset_test_user(&mut conn, &test_user);
    (pool, test_user)
}

#[allow(dead_code)]
pub fn setup_test_show() -> (Pool<ConnectionManager<SqliteConnection>>, NewShow) {
    let pool = establish_connection();
    let test_show = NewShow { name: "Testing".to_string(), description: "Testing Description".to_string() };

    let mut conn = pool.get().expect("Failed to get connection from pool");
    reset_test_show(&mut conn, &test_show);
    (pool, test_show)
}

#[allow(dead_code)]
pub fn setup_test_wrestler() -> (Pool<ConnectionManager<SqliteConnection>>, NewWrestler) {
    let pool = establish_connection();
    let test_wrestler = NewWrestler {
        name: "Testing".to_string(),
        gender: "Male Test".to_string(),
    };

    let mut conn = pool.get().expect("Failed to get connection from pool");
    reset_test_wrestler(&mut conn, &test_wrestler);
    (pool, test_wrestler)
}

#[allow(dead_code)]
pub fn setup_test_belt() -> (Pool<ConnectionManager<SqliteConnection>>, NewTitle) {
    let pool = establish_connection();
    let test_belt = NewTitle { name: "Testing".to_string() };

    let mut conn = pool.get().expect("Failed to get connection from pool");
    reset_test_belt(&mut conn, &test_belt);
    (pool, test_belt)
}

// Resets the test user by deleting it if it exists
#[allow(dead_code)]
fn reset_test_user(conn: &mut SqliteConnection, test_user: &NewUser) {
    // Import DSL items locally to avoid naming conflicts
    use wwe_universe_manager_lib::schema::users::dsl::{password, username, users};

    if let Ok(user) = users
        .filter(username.eq(&test_user.username))
        .filter(password.eq(&test_user.password))
        .first::<User>(conn)
    {
        println!("Deleting user: {:?}", user);
    }

    let result = diesel::delete(users.filter(username.eq(&test_user.username)))
        .filter(password.eq(&test_user.password))
        .execute(conn)
        .expect("Error deleting test user");

    info!("Deleted {} user", result);
}

#[allow(dead_code)]
fn reset_test_wrestler(conn: &mut SqliteConnection, test_wrestler: &NewWrestler) {
    // Import DSL items locally
    use wwe_universe_manager_lib::schema::wrestlers::dsl::{name as wrestler_name, wrestlers};

    if let Ok(wrestler) = wrestlers
        .filter(wrestler_name.eq(&test_wrestler.name))
        .first::<Wrestler>(conn)
    {
        println!("Deleting wrestler: {:?}", wrestler);
    }

    let result = diesel::delete(wrestlers.filter(wrestler_name.eq(&test_wrestler.name)))
        .execute(conn)
        .expect("Error deleting test wrestler");

    info!("Deleted {} wrestler", result);
}

#[allow(dead_code)]
fn reset_test_belt(conn: &mut SqliteConnection, test_belt: &NewTitle) {
    // Import DSL items locally
    use wwe_universe_manager_lib::schema::titles::dsl::{name as belt_name, titles};

    if let Ok(belt) = titles
        .filter(belt_name.eq(&test_belt.name))
        .first::<Title>(conn)
    {
        println!("Deleting belt: {:?}", belt);
    }

    let result = diesel::delete(titles.filter(belt_name.eq(&test_belt.name)))
        .execute(conn)
        .expect("Error deleting test belt");

    info!("Deleted {} belt", result);
}

#[allow(dead_code)]
fn reset_test_show(conn: &mut SqliteConnection, test_show: &NewShow) {
    // Import DSL items locally
    use wwe_universe_manager_lib::schema::shows::dsl::{name as show_name, shows};

    if let Ok(show) = shows
        .filter(show_name.eq(&test_show.name))
        .first::<Show>(conn)
    {
        println!("Deleting show: {:?}", show);
    }

    let result = diesel::delete(shows.filter(show_name.eq(&test_show.name)))
        .execute(conn)
        .expect("Error deleting test show");

    info!("Deleted {} show", result);
}


