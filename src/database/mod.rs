use crate::models::profile::Profile;

pub mod database_methods;
pub mod mock_database;

pub trait DatabaseTrait {
    async fn get_profile(&self, user_id: &str) -> Result<Option<Profile>, anyhow::Error>;
    async fn post_profile(
        &self,
        profile: &Profile,
    ) -> Result<mongodb::results::InsertOneResult, anyhow::Error>;
    async fn patch_profile(&self, profile: &Profile) -> Option<anyhow::Error>;
}

pub struct Database {
    pub client: mongodb::Client,
}

impl Database {
    pub fn new(client: mongodb::Client) -> Self {
        Database { client }
    }
}
