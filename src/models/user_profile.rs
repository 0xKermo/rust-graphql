use crate::graph_schemas::schemas::{FetchCollection, UserProfile};
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
            .expect("Error getting user profile")
            .unwrap();
        Ok(user_profile)
    }

    pub async fn update_user_profile(
        col: Collection<UserProfile>,
        input: UserProfile,
    ) -> Result<UserProfile, Error> {
        let filter = doc! {
            "wallet_address": input.wallet_address,
        };
        let update = doc! {
                "$set": {
               "username": input.username,
               "profile_image_url": input.profile_image_url,
               "banner_image_url": input.banner_image_url,
                  "bio": input.bio,
                  "social":{
                    "website": input.social.as_ref().unwrap().web_site.as_ref().unwrap().to_string(),
                    "twitter": input.social.as_ref().unwrap().twitter.as_ref().unwrap().to_string(),
                    "discord": input.social.as_ref().unwrap().discord.as_ref().unwrap().to_string(),
                },
            }
        };
        let user_profile = col
            .find_one_and_update(filter, update, None)
            .await
            .expect("Error updating user profile")
            .unwrap();
        Ok(user_profile)
    }
}
