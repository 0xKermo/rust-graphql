use crate::graph_schemas::schemas::{Erc721, FetchErc721, Pagination};
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Collection};
use std::io::Error;

pub struct TokenModel {}

impl TokenModel {
    pub async fn get_erc721_tokens(
        col: Collection<Erc721>,
        pagination: Pagination,
    ) -> Result<Vec<Erc721>, Error> {
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();
        let mut cursors = col
            .find(None, options)
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

    pub async fn get_user_tokens(
        col: Collection<Erc721>,
        input: FetchErc721,
        pagination: Pagination,
    ) -> Result<Vec<Erc721>, Error> {
        let filter = doc! {"owner": input.owner.unwrap().to_string()};
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();
        let mut results = col
            .find(filter, options)
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

    pub async fn get_token(col: Collection<Erc721>, input: FetchErc721) -> Result<Erc721, Error> {
        let token_id_low = input.token_id.as_ref().unwrap().low.to_string();
        let token_id_high = input.token_id.unwrap().high.to_string();
        let filter = doc! {"contract_address":input.contract_address.unwrap().to_string() ,"token_id.low": token_id_low, "token_id.high": token_id_high};
        let result = col
            .find_one(filter, None)
            .await
            .expect("Error getting token");
        Ok(result.unwrap())
    }
}
