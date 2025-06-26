use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::sync::Once;

use wwe_universe_manager_lib::db::establish_connection;
use wwe_universe_manager_lib::models::*;

static INIT: Once = Once::new();

pub fn setup_test_db() -> Pool<ConnectionManager<SqliteConnection>> {
    INIT.call_once(|| {
        env_logger::init();
    });
    establish_connection()
}

pub struct TestData {
    pub pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Default for TestData {
    fn default() -> Self {
        Self::new()
    }
}

impl TestData {
    pub fn new() -> Self {
        Self {
            pool: setup_test_db(),
        }
    }

    pub fn get_connection(
        &self,
    ) -> diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>> {
        self.pool.get().expect("Failed to get connection from pool")
    }

    #[allow(dead_code)]
    pub fn cleanup_users(&self, user_username: &str) {
        use wwe_universe_manager_lib::schema::users::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(users.filter(username.eq(user_username)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_wrestlers(&self, wrestler_name: &str) {
        use wwe_universe_manager_lib::schema::wrestlers::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(wrestlers.filter(name.eq(wrestler_name)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_titles(&self, title_name: &str) {
        use wwe_universe_manager_lib::schema::titles::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(titles.filter(name.eq(title_name)))
            .execute(&mut conn)
            .ok();
    }

    #[allow(dead_code)]
    pub fn cleanup_shows(&self, show_name: &str) {
        use wwe_universe_manager_lib::schema::shows::dsl::*;
        let mut conn = self.get_connection();
        diesel::delete(shows.filter(name.eq(show_name)))
            .execute(&mut conn)
            .ok();
    }
}

#[allow(dead_code)]
pub fn create_test_user() -> NewUser {
    NewUser {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
    }
}

#[allow(dead_code)]
pub fn create_test_wrestler() -> NewWrestler {
    NewWrestler {
        name: "Test Wrestler".to_string(),
        gender: "Male".to_string(),
        wins: 0,
        losses: 0,
    }
}

#[allow(dead_code)]
pub fn create_test_title() -> NewTitle {
    NewTitle {
        name: "Test Title".to_string(),
        current_holder_id: None,
    }
}

#[allow(dead_code)]
pub fn create_test_show() -> NewShow {
    NewShow {
        name: "Test Show".to_string(),
        description: "A test show for testing purposes".to_string(),
    }
}
