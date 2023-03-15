use crate::{
    database::mongo::DBMongo,
    graph_schemas::schemas::{
        FetchEvent,CollectionInfo,Events,Erc721Token,FilterErc721Token,CollectionProfile,FetchCollection, Pagination, UserProfile
    },
};
use async_graphql::{Context, EmptySubscription, FieldResult, Object, Schema};

pub struct Query;

#[Object(extends)]
impl Query {
    //collection query
    async fn get_collections(&self, ctx: &Context<'_>,limit_input:Pagination) -> FieldResult<Vec<CollectionInfo>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let collections = db.get_collections(limit_input).await.unwrap();
        Ok(collections)
    }
    
    async fn get_collection(&self, ctx: &Context<'_>,input:FetchCollection) -> FieldResult<CollectionInfo> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let collection = db.get_collection(
            &input).await.unwrap();
        Ok(collection)
    }
    async fn get_collection_profile(&self, ctx: &Context<'_>,input:FetchCollection) -> FieldResult<CollectionProfile> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let collection = db.get_collection_profile(
            &input).await.unwrap();
        Ok(collection)
    }

    // event query
    async fn get_events(&self, ctx: &Context<'_>,limit_input:Pagination) -> FieldResult<Vec<Events>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let events = db.get_events(limit_input).await.unwrap();
        Ok(events)
    }
    
    async fn get_collection_events(&self, ctx: &Context<'_>,input:FetchEvent, limit_input:Pagination) -> FieldResult<Vec<Events>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let collection_events = db.get_collection_events(
            &input.contract_address.unwrap(), limit_input).await.unwrap();
        Ok(collection_events)
    }

    async fn get_user_events(&self, ctx: &Context<'_>,input:FetchEvent, limit_input:Pagination) -> FieldResult<Vec<Events>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let tokens = db.get_user_events(input.owner.unwrap(), limit_input).await.unwrap();
        Ok(tokens)
    }

    async fn get_token_events(&self, ctx: &Context<'_>,input:FetchEvent, limit_input:Pagination) -> FieldResult<Vec<Events>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let tokens = db.get_token_events(input, limit_input).await.unwrap();
        Ok(tokens)
    }
    // token query
    async fn get_tokens(&self, ctx: &Context<'_>, limit_input:Pagination) -> FieldResult<Vec<Erc721Token>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let tokens = db.get_erc721_tokens(limit_input).await.unwrap();
        Ok(tokens)
    }

    async fn get_token(&self, ctx: &Context<'_>,input:FilterErc721Token) -> FieldResult<Erc721Token> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let token = db.get_token(input).await.unwrap();
        Ok(token)
    }

    async fn get_user_tokens(&self, ctx: &Context<'_>,input:FilterErc721Token, limit_input:Pagination) -> FieldResult<Vec<Erc721Token>> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let tokens = db.get_user_tokens(input, limit_input).await.unwrap();
        Ok(tokens)
    }

    async fn get_user_profile(&self, ctx: &Context<'_>,input:FetchCollection) -> FieldResult<UserProfile> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let user_profile = db.get_user_profile(input).await.unwrap();
        Ok(user_profile)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    //collection mutation
    async fn update_collection_profile(&self, ctx: &Context<'_>, input: CollectionProfile) -> FieldResult<CollectionProfile> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let collection_profile = db.update_collection_profile(input).await.unwrap();
        Ok(collection_profile)
    }
    // user mutation
    async fn update_user_profile(&self, ctx: &Context<'_>, input: UserProfile) -> FieldResult<UserProfile> {
        let db = &ctx.data_unchecked::<DBMongo>();
        let user_profile = db.update_user_profile(input).await.unwrap();
        Ok(user_profile)
    }

}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;