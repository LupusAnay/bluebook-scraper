use crate::error::Error;
use reqwest::{Client, Proxy};
use select::document::Document;

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
        .build()
        .expect("Cannot build reqwest client")
}

pub async fn get_search_page(
    client: Client,
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
        .await?;

    let page = Document::from(response.text().await?.as_str());
    Ok(page)
}

pub async fn get_company_page(client: Client, company_id: i32) -> ApiResult {
    let response = client
        .get(format!("{}/{}", COMPANY_PAGE_URL, company_id).as_str())
        .header("User-Agent", get_random_user_agent().await)
        .send()
        .await?;

    let page = Document::from(response.text().await?.as_str());
    Ok(page)
}

#[cfg(test)]
mod tests {
    use crate::api::*;

    #[tokio::test(threaded_scheduler)]
    async fn test_get_page() {
        let client = client_factory().await;
        let page = get_search_page(client, 4030, 1, "New York, NY")
            .await
            .unwrap();
    }

    #[tokio::test(threaded_scheduler)]
    async fn test_company_page() {
        let client = client_factory().await;
        let page = get_company_page(client, 400516).await.unwrap();
    }
}
