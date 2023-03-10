use crate::{
    database::mongo::DBMongo,
    graph_schemas::schemas::{
        FetchEvent,CollectionInfo,Events,Erc721Token,FilterErc721Token,FetchCollection, Pagination, UserProfile
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
    //owner mutation
    async fn update_collectionProfile(&self, ctx: &Context<'_>, input: FetchEvent) -> FieldResult<Events> {
        // let db = &ctx.data_unchecked::<DBMongo>();
        let new_owner = Events {
            _id: None,
            contract_address:None,
            operator:None,
            from:None,
            to:None,
            amount:None,
            token_id    :None,
            block_number:None,
            transaction_hash:None,
            event_type:None,
            contract_type  :None,

            
        };
        // let owner = db.create_owner(new_owner).await.unwrap();
        Ok(new_owner)
    }

    // async fn create_project(
    //     &self,
    //     ctx: &Context<'_>,
    //     input: CreateProject,
    // ) -> FieldResult<Project> {
    //     let db = &ctx.data_unchecked::<DBMongo>();
    //     let new_project = Project {
    //         _id: None,
    //         owner_id: input.owner_id,
    //         name: input.name,
    //         description: input.description,
    //         status: input.status,
    //     };
    //     let project = db.create_project(new_project).await.unwrap();
    //     Ok(project)
    // }
}

pub type ProjectSchema = Schema<Query, Mutation, EmptySubscription>;