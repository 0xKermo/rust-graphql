use crate::graph_schemas::schemas::{
    CollectionInfo, Erc721Token, Events, FilterErc721Token,CollectionProfile,FetchCollection, FetchEvent, Pagination, UserProfile,
};
use dotenv::dotenv;
use mongodb::{
    Client, Collection, Database,
};

use std::{env, io::Error};
use crate::models::{
    collection::CollectionModel,
    event::EventModel,
    token::TokenModel,
    user_profile::UserModel
};

//imports goes here
pub struct DBMongo {
    db: Database,
}

impl DBMongo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODBURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri.as_str()).await.unwrap();
        let db = client.database("moso");
        DBMongo { db }
    }

    pub fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    pub async fn get_collections(
        &self,
        pagination: Pagination,
    ) -> Result<Vec<CollectionInfo>, Error> {
        let col = DBMongo::col_helper::<CollectionInfo>(&self, "contracts");
        CollectionModel::get_collections(col, pagination).await
    }

    pub async fn get_collection(&self, input: &FetchCollection) -> Result<CollectionInfo, Error> {
        let col = DBMongo::col_helper::<CollectionInfo>(&self, "contracts");
        CollectionModel::get_collection(col, input).await
    }

    pub async fn get_collection_profile(
        &self,
        input: &FetchCollection,
    ) -> Result<CollectionProfile, Error> {
        let col = DBMongo::col_helper::<CollectionProfile>(&self, "collection_profile");
        CollectionModel::get_collection_profile(col, input).await
    }

    pub async fn get_events(&self, pagination: Pagination) -> Result<Vec<Events>, Error> {
        let col = DBMongo::col_helper::<Events>(&self, "events");
        EventModel::get_events(col, pagination).await
    }

    pub async fn get_collection_events(
        &self,
        contract_address: &String,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let col = DBMongo::col_helper::<Events>(&self, "events");
        EventModel::get_collection_events(col, contract_address, pagination).await
    }

    pub async fn get_user_events(
        &self,
        owner_address: String,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let col = DBMongo::col_helper::<Events>(&self, "events");
        EventModel::get_user_events(col, owner_address, pagination).await
    }

    pub async fn get_token_events(
        &self,
        input: FetchEvent,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let col = DBMongo::col_helper::<Events>(&self, "events");
        EventModel::get_token_events(col, input, pagination).await
   
    }

    pub async fn get_erc721_tokens(
        &self,
        pagination: Pagination,
    ) -> Result<Vec<Erc721Token>, Error> {
        let col = DBMongo::col_helper::<Erc721Token>(&self, "erc721_tokens");
        TokenModel::get_erc721_tokens(col, pagination).await
    }

    pub async fn get_user_tokens(
        &self,
        input: FilterErc721Token,
        pagination: Pagination,
    ) -> Result<Vec<Erc721Token>, Error> {
        let col = DBMongo::col_helper::<Erc721Token>(&self, "erc721_tokens");
        TokenModel::get_user_tokens(col, input, pagination).await
    }

    pub async fn get_token(&self, input: FilterErc721Token) -> Result<Erc721Token, Error> {
        let col = DBMongo::col_helper::<Erc721Token>(&self, "erc721_tokens");
        TokenModel::get_token(col, input).await
    }

    pub async fn get_user_profile(&self, input: FetchCollection) -> Result<UserProfile, Error> {
        let col = DBMongo::col_helper::<UserProfile>(&self, "user_profile");
        UserModel::get_user_profile(col, input).await
    }

    // POST




    // PUT

    pub async fn update_collection_profile(
        &self,
        input: &CollectionProfile,
    ) -> Result<CollectionProfile, Error> {
        let col = DBMongo::col_helper::<CollectionProfile>(&self, "collection_profile");
        CollectionModel::update_collection_profile(col, input).await
    }


}
