mod database;
mod graph_handler;
mod graph_schemas;
mod models;
//add 
use actix_web::{
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use database::mongo::DBMongo;
use graph_handler::handler::{Mutation, ProjectSchema, Query};


//graphql entry
async fn index(schema: Data<ProjectSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //connect to the data source
    let db = DBMongo::init().await;
    let schema_data = Schema::build(Query, Mutation, EmptySubscription)
        .data(db)
        .finish();
    println!("Playground: http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema_data.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
                )
            })
            .bind(("127.0.0.1", 8080))?
            .run() 
            .await
        }