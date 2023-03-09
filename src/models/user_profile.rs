use crate::graph_schemas::schemas::{UserProfile,FetchCollection};
use mongodb::{bson::doc, Collection};
use std::io::Error;


pub struct UserModel;

impl UserModel {
    pub async fn get_user_profile(
        col: Collection<UserProfile>,
        input: FetchCollection,
    ) -> Result<UserProfile, Error> {
        let filter = doc! {
            "address": input.address,
        };
        let user_profile = col
            .find_one(filter, None)
            .await
            .expect("Error getting user profile").unwrap();
        Ok(user_profile)
    }
}