use crate::graph_schemas::schemas::{CollectionInfo, FetchCollection, Pagination};
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document},
    options::FindOptions,
    Collection,
};
use std::io::Error;
pub struct CollectionModel {}

impl CollectionModel {
    pub async fn get_collections(
        collection: Collection<CollectionInfo>,
        pagination: Pagination,
    ) -> Result<Vec<CollectionInfo>, Error> {
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();

        let mut cursors = collection
            .find(None, options)
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

    pub async fn get_collection(
        collection: Collection<CollectionInfo>,
        input: &FetchCollection,
    ) -> Result<CollectionInfo, Error> {
        let contract_address = input.address.as_ref().unwrap().to_string();
        let filter = doc! {"address": contract_address};
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
        ];

        let mut results = collection
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
}
