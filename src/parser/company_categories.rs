use crate::api::get_categories_page;
use crate::data::Category;
use crate::parser::{Error, ReText};
use futures::future::try_join_all;
use regex::Regex;
use reqwest::Client;
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Name},
};

pub async fn get_company_categories(
    client: &Client,
    page: &Document,
    company_id: i32,
) -> Result<Vec<Category>, Error> {
    let trade_code_node = page
        .find(Attr("id", "tradeCodeDiv"))
        .next()
        .ok_or(Error::CannotFindNode)?;

    let div_ids: Vec<&str> = trade_code_node
        .find(Name("a"))
        .map(|node| node.attr("data-divid").unwrap_or(""))
        .collect();

    let mut categories = Vec::new();

    let mut trade_code_node_categories: Vec<String> = trade_code_node
        .find(ReText(
            &Regex::new(r"^.*\d{2} [1-9]{2} [1-9]{2}.*$").unwrap(),
        ))
        .map(|node| node.find(Name("strong")).next())
        .filter_map(|node| node.map(|node| node.text()))
        .collect();

    let category_calls = div_ids
        .iter()
        .map(|id| collect_categories_from_div_id(&client, id, company_id));

    categories.append(&mut trade_code_node_categories);
    categories.append(
        &mut try_join_all(category_calls)
            .await?
            .iter()
            .flat_map(|cat| cat.clone())
            .collect(),
    );

    Ok(categories)
}

async fn collect_categories_from_div_id(
    client: &Client,
    div_id: &str,
    company_id: i32,
) -> Result<Vec<Category>, Error> {
    let meta_categories = get_categories_page(&client, company_id, div_id, None).await?;
    let heading_and_div_ids: Vec<(&str, &str)> = meta_categories
        .find(Name("a"))
        .map(|node| {
            (
                node.attr("data-divid").unwrap_or(""),
                node.attr("data-headid").unwrap_or(""),
            )
        })
        .collect();

    let mut categories_array = try_join_all(
        heading_and_div_ids
            .iter()
            .map(|(did, hid)| collect_subcategories(&client, company_id, did, hid)),
    )
    .await?;

    let categories = categories_array
        .iter()
        .flat_map(|cat| cat.clone())
        .collect();

    Ok(categories)
}

fn match_regex(regex: Regex, node: &Node) -> bool {
    let text = node.text();
    regex.is_match(&text)
}

async fn collect_subcategories(
    client: &Client,
    company_id: i32,
    div_id: &str,
    head_id: &str,
) -> Result<Vec<Category>, Error> {
    let categories_page = get_categories_page(&client, company_id, div_id, Some(head_id)).await?;
    let categories = categories_page
        .find(ReText(&Regex::new(r"^.*\d{2} \d{2} \d{2}.*$").unwrap()))
        .map(|node| node.find(Name("strong")).next())
        .filter_map(|node| node.map(|node| node.text()))
        .collect();
    Ok(categories)
}

#[cfg(test)]
mod tests {
    use crate::api::{client_factory, get_categories_page, get_company_page};
    use crate::parser::company_categories::*;

    #[tokio::test]
    async fn test_collect_subcategories() {
        let client = client_factory().await;
        get_company_page(&client, 922369).await.unwrap();
        get_categories_page(&client, 922369, "11", None)
            .await
            .unwrap();
        let categories = collect_subcategories(&client, 922369, "11", "1669")
            .await
            .unwrap();
        assert_eq!(categories, vec!["11 22 00.00.01"])
    }

    #[tokio::test]
    async fn test_get_company_categories() {
        let client = client_factory().await;
        let page = Document::from(include_str!("../../test-data/company.html"));
        let categories = get_company_categories(&client, &page, 922369)
            .await
            .unwrap();

        assert_eq!(
            categories,
            vec![
                "07 01 60",
                "07 01 60.71",
                "07 01 60.91",
                "07 01 60.92",
                "07 06 60",
                "07 60 00.00.01",
                "11 22 00.00.01",
                "11 47 00.00.01",
                "23 01 20",
                "23 01 30",
                "23 01 50",
                "23 01 60",
                "23 01 70",
                "23 01 80",
                "23 06 20",
                "23 06 30",
                "23 06 30.13",
                "23 06 30.16",
                "23 06 30.19",
                "23 06 50",
                "23 06 50.13",
                "23 06 60",
                "23 06 60.13",
                "23 06 60.16",
                "23 06 70",
                "23 06 70.13",
                "23 06 70.16",
                "23 06 80",
                "23 06 80.13",
                "23 06 80.16",
                "23 20 00.00.01",
                "23 23 16",
                "23 23 19",
                "23 23 23",
                "23 24 13",
                "23 30 00.00.01",
                "23 31 13",
                "23 31 13.13",
                "23 31 13.16",
                "23 31 13.19",
                "23 31 16",
                "23 31 19",
                "23 32 13",
                "23 32 36",
                "23 32 39",
                "23 32 43",
                "23 33 19",
                "23 33 23",
                "23 34 13",
                "23 34 16",
                "23 34 23",
                "23 35 16",
                "23 35 16.13",
                "23 35 16.16",
                "23 37 16",
                "23 38 13",
                "23 38 13.13",
                "23 38 13.16",
                "23 38 16",
                "23 41 43",
                "23 41 46",
                "23 43 13",
                "23 43 16",
                "23 43 23",
                "23 51 13.13",
                "23 51 43",
                "23 51 43.13",
                "23 60 00.00.01",
                "23 61 23",
                "23 62 13",
                "23 62 23",
                "23 63 13",
                "23 63 23",
                "23 63 33",
                "23 64 13",
                "23 64 13.13",
                "23 64 13.16",
                "23 64 16",
                "23 64 19",
                "23 64 23",
                "23 65 33",
                "23 71 13",
                "23 71 13.13",
                "23 71 13.23",
                "23 71 16",
                "23 71 19",
                "23 71 19.13",
                "23 71 19.16",
                "23 71 19.19",
                "23 71 19.23",
                "23 71 19.26",
                "28 35 13",
                "28 35 23",
                "28 35 33",
                "33 81 33",
                "42 22 13",
                "42 22 16",
                "42 22 19",
                "42 22 23",
                "42 22 26",
                "43 11 13",
                "43 11 13.13",
                "43 11 13.16",
                "43 11 23"
            ]
        )
    }
}
