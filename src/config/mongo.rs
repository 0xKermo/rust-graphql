use crate::schemas::project_schema::{CollectionInfo, Erc721, Events, FetchErc721};
use dotenv::dotenv;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document},
    Client, Collection, Database,
};
use std::{env, io::Error};

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
        let client = Client::with_uri_str(uri.as_str())
            .await
            .unwrap();
        let db = client.database("moso");
        DBMongo { db }
    }

    fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        data_source.db.collection(collection_name)
    }

    pub async fn get_collections(&self) -> Result<Vec<CollectionInfo>, Error> {
        let col = DBMongo::col_helper::<CollectionInfo>(&self, "contracts");
        let mut cursors = col
            .find(None, None)
            .await
            .expect("Error getting list of collections");
        let mut collections: Vec<CollectionInfo> = Vec::new();
        while let Some(collection) = cursors
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            collections.push(collection)
        }
        Ok(collections)
    }

    pub async fn get_collection(&self, contract_address: &String) -> Result<CollectionInfo, Error> {
        let filter = doc! {"address": contract_address.to_string()};
        let col = DBMongo::col_helper::<CollectionInfo>(&self, "contracts");
        let pipeline = vec![
            doc! { "$match": filter.clone()
            },
            doc! { "$lookup": {
                "from": "erc1115_tokens",
                "localField": "address",
                "foreignField": "contract_address",
                "as": "erc1115_tokens"
            }},
            doc! {
              "$set": {
                  "total_supply": {
                    "$size": "$erc1115_tokens"
                  }
                }
            },
            // doc! {
            //   "$project": {
            //       "erc1155_tokens": 0
            //     }
            // },
        ];

        let mut results = col
            .aggregate(pipeline, None)
            .await
            .expect("Error getting collection");
        let res = results
            .try_next()
            .await
            .expect("Error mapping through cursor")
            .unwrap();
        println!("res {:?}", res);
        let collection: CollectionInfo = from_document(res).unwrap();
        Ok(collection)
    }

    pub async fn get_events(&self) -> Result<Vec<Events>, Error> {
        let col = DBMongo::col_helper::<Events>(&self, "events");
        let mut cursors = col
            .find(None, None)
            .await
            .expect("Error getting list of events");
        let mut events: Vec<Events> = Vec::new();
        while let Some(event) = cursors
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            events.push(event)
        }
        Ok(events)
    }

    pub async fn get_collection_events(
        &self,
        contract_address: &String,
    ) -> Result<Vec<Events>, Error> {
        let filter = doc! {"contract_address": contract_address.to_string()};
        let col = DBMongo::col_helper::<Events>(&self, "events");
        let mut results = col
            .find(filter, None)
            .await
            .expect("Error getting collection events");
        let mut events: Vec<Events> = Vec::new();
        while let Some(event) = results
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            events.push(event)
        }
        Ok(events)
    }

    pub async fn get_user_events(&self, owner_address: String) -> Result<Vec<Events>, Error> {
        let filter =
            doc! {"$or":[{"from": owner_address.to_string()},{"to": owner_address.to_string()}]};
        let col = DBMongo::col_helper::<Events>(&self, "events");
        let mut results = col
            .find(filter, None)
            .await
            .expect("Error getting user events");
        let mut events: Vec<Events> = Vec::new();
        while let Some(event) = results
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            events.push(event)
        }
        Ok(events)
    }

    pub async fn get_erc721_tokens(&self) -> Result<Vec<Erc721>, Error> {
        let col = DBMongo::col_helper::<Erc721>(&self, "erc721_tokens");
        let mut cursors = col
            .find(None, None)
            .await
            .expect("Error getting list of erc721 tokens");
        let mut erc721: Vec<Erc721> = Vec::new();
        while let Some(erc721_token) = cursors
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            println!("{:?}", erc721_token);
            erc721.push(erc721_token)
        }
        Ok(erc721)
    }

    pub async fn get_user_tokens(&self, input: FetchErc721) -> Result<Vec<Erc721>, Error> {
        let filter = doc! {"owner": input.owner.unwrap().to_string()};
        let col = DBMongo::col_helper::<Erc721>(&self, "erc721_tokens");
        let mut results = col
            .find(filter, None)
            .await
            .expect("Error getting user tokens");
        let mut erc721: Vec<Erc721> = Vec::new();
        while let Some(erc721_token) = results
            .try_next()
            .await
            .expect("Error mapping through cursor")
        {
            erc721.push(erc721_token)
        }
        Ok(erc721)
    }

    pub async fn get_token(&self, input: FetchErc721) -> Result<Erc721, Error> {
        let token_id_low = input.token_id.as_ref().unwrap().low.to_string();
        let token_id_high = input.token_id.unwrap().high.to_string();
        let filter = doc! {"contract_address":input.contract_address.unwrap().to_string() ,"token_id.low": token_id_low, "token_id.high": token_id_high};
        let col = DBMongo::col_helper::<Erc721>(&self, "erc721_tokens");
        let result = col
            .find_one(filter, None)
            .await
            .expect("Error getting token");
        Ok(result.unwrap())
    }
}
