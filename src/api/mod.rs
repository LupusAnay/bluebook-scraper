pub use crate::api::error::Error;
use reqwest::{Client, Proxy};
use select::document::Document;

pub mod error;

const SEARCH_CLASSES_URL: &str =
    "http://www.thebluebook.com/products/bluesearchtechnology/search-companies.html";
const COMPANY_SEARCH_URL: &str = "http://www.thebluebook.com/search.html";
const COMPANY_PAGE_URL: &str = "http://www.thebluebook.com/iProView";
const CATEGORIES_SEARCH_URL: &str = "http://www.thebluebook.com/ajax/profile/proViewCSIClassByDiv/";

pub type ApiResult = Result<Document, Error>;

async fn get_random_user_agent() -> &'static str {
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36"
}

pub async fn client_factory() -> Client {
    let proxy =
        Proxy::http("socks5://127.0.0.1:9050").expect("Cannot build proxy for reqwest client");

    Client::builder()
        .proxy(proxy)
        .cookie_store(true)
        .build()
        .expect("Cannot build reqwest client")
}

pub async fn get_search_page(
    client: &Client,
    class_id: i32,
    page_number: i32,
    city: &str,
) -> ApiResult {
    let response = client
        .get(COMPANY_SEARCH_URL)
        .query(&[("class", class_id), ("region", 1), ("page", page_number)])
        .query(&[
            ("city", city),
            ("geographicalarea", "New+York+City"),
            ("searchsrc", "index"),
            ("regionLabel", city),
        ])
        .header("User-Agent", get_random_user_agent().await)
        .send()
        .await?
        .error_for_status()?;

    let page = Document::from(response.text().await?.as_str());
    Ok(page)
}

pub async fn get_company_page(client: &Client, company_id: i32) -> ApiResult {
    let response = client
        .get(format!("{}/{}", COMPANY_PAGE_URL, company_id).as_str())
        .header("User-Agent", get_random_user_agent().await)
        .send()
        .await?
        .error_for_status()?;

    let page = Document::from(response.text().await?.as_str());
    Ok(page)
}

pub async fn get_locations_page(client: Client, company_id: i32) -> ApiResult {
    let response = client
        .get(format!("{}/{}/locations-contacts/", COMPANY_PAGE_URL, company_id).as_str())
        .header("User-Agent", get_random_user_agent().await)
        .send()
        .await?
        .error_for_status()?;

    let page = Document::from(response.text().await?.as_str());
    Ok(page)
}

pub async fn get_categories_page(
    client: &Client,
    company_id: i32,
    div_id: &str,
    head_id: Option<&str>,
) -> ApiResult {
    let company_id_str = company_id.to_string();
    let mut params = vec![("qp", company_id_str.as_str()), ("id", div_id), ("v", "7")];

    match head_id {
        Some(id) => params.push(("headingID", id)),
        None => (),
    }

    let request = client
        .post(CATEGORIES_SEARCH_URL)
        .form(&params)
        .header("User-Agent", get_random_user_agent().await)
        .header("X-Requested-With", "XMLHttpRequest")
        .header("Referer", "http://www.thebluebook.com/iProView/400516")
        .build()?;
    let response = client.execute(request).await?.error_for_status()?;

    let page = Document::from(response.text().await?.as_str());
    Ok(page)
}

#[cfg(test)]
mod tests {
    use crate::api::*;

    #[tokio::test]
    async fn test_get_categories() {
        let client = client_factory().await;
        let page = get_categories_page(&client, 400516, "17", None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_categories_with_heading_id() {
        let mut client = client_factory().await;

        let page = get_categories_page(&mut client, 400516, "17", Some("642"))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_page() {
        let client = client_factory().await;
        let page = get_search_page(&client, 4030, 1, "New York, NY")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_company_page() {
        let client = client_factory().await;
        let page = get_company_page(&client, 400516).await.unwrap();
    }

    #[tokio::test]
    async fn test_locations_page() {
        let client = client_factory().await;
        let page = get_locations_page(client, 400516).await.unwrap();
    }
}
