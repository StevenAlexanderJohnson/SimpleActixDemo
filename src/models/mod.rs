pub mod profile;
pub mod user;

use mongodb::bson::oid::ObjectId;

// Serialize an ObjectId to a string
fn serialize_object_id<S>(oid: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match oid {
        Some(x) => {
            let s = x.to_string();
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}
