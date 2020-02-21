use crate::api::{client_factory, get_page};
use crate::error::Error;
use crate::parser::{get_page_ids, get_pages_count};
use reqwest::Client;
use serde_json::from_reader;
use std::fs::File;

mod api;
mod error;
mod parser;

const DEFAULT_CITY: &str = "New York, NY";

#[tokio::main]
async fn main() {
    let client = client_factory().await;
    println!("Hello, world!");
}

async fn get_companies_ids_from_class(client: Client, class_id: i32) -> Result<Vec<i32>, Error> {
    let page = get_page(client, class_id, 1, DEFAULT_CITY).await?;
    let pages_count = get_pages_count(&page).await?;
    let first_page_ids = get_page_ids(&page).await?;

    unimplemented!()
}

async fn read_classes() -> Result<Vec<i32>, Error> {
    let file = File::open("classes.json")?;
    let classes: Vec<i32> = from_reader(file)?;
    Ok(classes)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[tokio::test(threaded_scheduler)]
    async fn test_read_classes() {
        let classes = read_classes().await.unwrap();
        assert_eq!(classes.len(), 612)
    }
}
