use crate::models::profile::Profile;

pub mod database_methods;

pub trait DatabaseTrait {
    async fn get_profile(&self, user_id: &str) -> Result<Option<Profile>, mongodb::error::Error>;
    async fn post_profile(
        &self,
        profile: &Profile,
    ) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error>;
    async fn patch_profile(
        &self,
        profile: &Profile,
    ) -> Result<mongodb::results::UpdateResult, mongodb::error::Error>;
}

pub struct Database {
    pub client: mongodb::Client,
}

impl Database {
    pub fn new(client: mongodb::Client) -> Self {
        Database { client }
    }
}
