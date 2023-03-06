use crate::graph_schemas::schemas::{Events, FetchEvent, Pagination};
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions, Collection};
use std::io::Error;

pub struct EventModel {}

impl EventModel {
    pub async fn get_events(
        col: Collection<Events>,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();
        let mut cursors = col
            .find(None, options)
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
        col: Collection<Events>,
        contract_address: &String,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let filter = doc! {"contract_address": contract_address.to_string()};
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();

        let mut results = col
            .find(filter, options)
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

    pub async fn get_user_events(
        col: Collection<Events>,
        owner_address: String,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let filter =
            doc! {"$or":[{"from": owner_address.to_string()},{"to": owner_address.to_string()}]};
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();

        let mut results = col
            .find(filter, options)
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

    pub async fn get_token_events(
        col: Collection<Events>,
        input: FetchEvent,
        pagination: Pagination,
    ) -> Result<Vec<Events>, Error> {
        let token_id_low = input.token_id.as_ref().unwrap().low.to_string();
        let token_id_high = input.token_id.unwrap().high.to_string();
        let filter = doc! {"contract_address": input.contract_address,"token_id.low": token_id_low,"token_id.high":token_id_high};
        let options = FindOptions::builder()
            .skip(pagination.skip.unwrap_or_default())
            .limit(pagination.limit.unwrap_or_default())
            .build();
        let mut results = col
            .find(filter, options)
            .await
            .expect("Error getting token events");
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
}
