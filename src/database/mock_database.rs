use anyhow::anyhow;
use mongodb::bson::oid::ObjectId;

use crate::models::profile::Profile;

use super::DatabaseTrait;

pub struct MockDatabase {
    pub should_return_none: bool,
    pub should_return_error: bool,
}

impl DatabaseTrait for MockDatabase {
    async fn get_profile(&self, _user_id: &str) -> Result<Option<Profile>, anyhow::Error> {
        if self.should_return_none {
            return Ok(None);
        }
        if self.should_return_error {
            return Err(anyhow!("Error"));
        }
        Ok(Some(Profile {
            _id: Some(ObjectId::new()),
            address: "address".to_string(),
            name: "name".to_string(),
            bio: "Bio".to_string(),
            email: "email".to_string(),
            phone: "phone".to_string(),
            age: 1,
        }))
    }

    async fn post_profile(
        &self,
        _profile: &Profile,
    ) -> Result<mongodb::results::InsertOneResult, anyhow::Error> {
        todo!()
    }

    async fn patch_profile(&self, _profile: &Profile) -> Option<anyhow::Error> {
        if self.should_return_error {
            return Some(anyhow!("Error"));
        }
        // create a mongodb result with the number of documents modified
        None
    }
}
