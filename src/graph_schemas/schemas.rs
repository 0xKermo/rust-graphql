use async_graphql::{InputObject, SimpleObject, Enum,EnumType};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Common schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject,InputObject)]
pub struct TokenId {
    pub low: String,
    pub high: String,
}
#[derive(InputObject)]
pub struct FetchTokenId {
    pub low: String,
    pub high: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum DisplayType {
    Number,
    BoostPercentage,
    BoostNumber,
    Date,
}
#[derive(Debug, Clone,Copy, PartialEq, Eq, Serialize, Deserialize,Enum)]

pub enum AttributeValue {
    String,
    Number,
    bool,
    Vec,
    NumberVec,
    VecString,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Attributes {
    display_type: Option<DisplayType>,
    trait_type: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Metadata {
    name: Option<String>,
    description: Option<String>,
    image: Option<String>,
    external_url: Option<String>,
    animation_url: Option<String>,
    // attributes: Option<Vec<Attributes>>,
}
#[derive(InputObject)]
pub struct FetchMetadata {
    name: Option<String>,
    description: Option<String>,
    image: Option<String>,
    external_url: Option<String>,
    animation_url: Option<String>,
    // attributes: Option<Vec<Attributes>>,
}
// Collection schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CollectionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
    owner: Option<String>,
    contract_uri: Option<String>,
    base_uri: Option<String>,
    pub contract_type: Option<String>,
    
    creator_fee: Option<i32>,
    creator_fee_recipient: Option<String>,
    total_listed: Option<i32>,
    volume: Option<String>,
    total_supply: Option<i32>,
    total_owners: Option<i32>,
    unique_owners: Option<i32>,

    best_collection_offer: Option<i32>,

    floor_price: Option<i32>,
    volume_one_day: Option<i32>,
    volume_one_week: Option<i32>,

}

// Filter for collection query
#[derive(InputObject)]
pub struct FetchCollection{
    pub address: Option<String>,
    pub name: Option<String>,
    pub owner: Option<String>,
    pub contract_type: Option<String>,
}

// Events schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Events {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub contract_address: Option<String>,
    pub operator: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub token_id: Option<TokenId>,
    pub amount: Option<i64>,
    pub block_number: Option<i64>,
    pub transaction_hash: Option<String>,
    pub event_type: Option<String>,
    pub contract_type: Option<String>,
}

/*
filter for events schema
*/
#[derive(InputObject)]
pub struct FetchEvent{
    pub _id: Option<ObjectId>,
    pub contract_address: Option<String>,
    pub operator: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub token_id: Option<FetchTokenId>,
    pub amount: Option<i64>,
    pub event_type: Option<String>,
    pub contract_type: Option<String>,
    pub owner: Option<String>,
}

#[derive(Enum, Copy, Clone, Debug,Eq,PartialEq, Serialize, Deserialize)]
pub enum Status {
    Listed,
    Sold,
    Cancelled,
}

// erc721 schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Erc721Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    contract_address: Option<String>,
    token_id: Option<TokenId>,
    owner: Option<String>,
    last_updated: Option<i64>,
    metadata: Option<Metadata>,

    price: Option<i64>,
    last_sale_price: Option<i64>,
    best_offer: Option<i64>,
    best_offer_expires_at: Option<i64>,
    status: Option<Status>,
}

#[derive(InputObject)]
pub struct FilterErc721Token {
    pub _id: Option<ObjectId>,
    pub contract_address: Option<String>,
    pub token_id: Option<FetchTokenId>,
    pub owner: Option<String>,
    pub last_updated: Option<i64>,
    pub metadata: Option<FetchMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Erc1155Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    contract_address: Option<String>,
    token_id: Option<TokenId>,
    owner: Option<String>,
    balance: Option<i64>,
    last_updated: Option<i64>,
    metadata: Option<Metadata>,

    price: Option<i64>,
    last_sale_price: Option<i64>,
    best_offer: Option<i64>,
    best_offer_expires_at: Option<i64>,
    status: Option<Status>,
}

#[derive(InputObject)]
pub struct Pagination {
    pub skip: Option<u64>,
    pub limit: Option<i64>,
}


// User profile schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct UserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_address: Option<String>,
    username: Option<String>,
    profile_image_url: Option<String>,
    banner_image_url: Option<String>,
    bio: Option<String>,
    social: Option<Social>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Social {
    web_site: Option<String>,
    twitter: Option<String>,
    discord: Option<String>,
}

// User profile schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct CollectionProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    contract_address: Option<String>,
    owner: Option<String>,
    profile_image_url: Option<String>,
    banner_image_url: Option<String>,
    bio: Option<String>,
    social: Option<Social>,
}
