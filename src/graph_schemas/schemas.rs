use async_graphql::{InputObject, SimpleObject, Enum};
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
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,Enum)]

// pub enum AttributeValue {
//     String(String),
//     Number(Number),
//     Bool(bool),
//     StringVec(Vec<String>),
//     NumberVec(Vec<Number>),
//     BoolVec(Vec<bool>),
// }

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
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub owner: Option<String>,
    pub contract_uri: Option<String>,
    pub base_uri: Option<String>,
    pub contract_type: Option<String>,
    pub volume: Option<String>,
    pub total_supply: Option<i32>,
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

// erc721 schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Erc721 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub contract_address: Option<String>,
    pub token_id: Option<TokenId>,
    pub owner: Option<String>,
    pub last_updated: Option<i64>,
    pub metadata: Option<Metadata>,
}

#[derive(InputObject)]
pub struct FetchErc721 {
    pub _id: Option<ObjectId>,
    pub contract_address: Option<String>,
    pub token_id: Option<FetchTokenId>,
    pub owner: Option<String>,
    pub last_updated: Option<i64>,
    pub metadata: Option<FetchMetadata>,
}