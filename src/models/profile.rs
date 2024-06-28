use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::serialize_object_id;

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub bio: String,
}
