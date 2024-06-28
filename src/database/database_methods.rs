use anyhow::Context;
use mongodb::bson::{doc, oid::ObjectId};
use std::str::FromStr;

use crate::models::profile::Profile;

use super::{Database, DatabaseTrait};

impl DatabaseTrait for Database {
    async fn get_profile(&self, user_id: &str) -> Result<Option<Profile>, anyhow::Error> {
        self.client
            .database("testing")
            .collection::<Profile>("profile")
            .find_one(
                mongodb::bson::doc! {
                    "_id": ObjectId::from_str(&user_id).unwrap()
                },
                None,
            )
            .await
            .context("Failed to find profile")
    }

    async fn post_profile(
        &self,
        profile: &Profile,
    ) -> Result<mongodb::results::InsertOneResult, anyhow::Error> {
        self.client
            .database("testing")
            .collection::<Profile>("profile")
            .insert_one(profile, None)
            .await
            .context("Failed to insert profile")
    }

    async fn patch_profile(&self, profile: &Profile) -> Option<anyhow::Error> {
        match self.client
        .database("testing")
        .collection::<&Profile>("profile")
        .update_one(
            doc! {"_id": profile._id},
            {
                doc! {"$set": doc! {"name": &profile.name, "email": &profile.email, "age": &profile.age, "address": &profile.address, "phone": &profile.phone, "bio": &profile.bio}}
            },
            None,
        )
            .await.context("Failed to update profile") {
                Ok(_) => None,
                Err(e) => Some(e),
            }
    }
}
