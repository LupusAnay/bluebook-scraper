use crate::data::{CompanyInfo, CompanyProfile};
pub use crate::parser::error::Error;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

mod company_contacts;
mod company_info;
mod company_locations;
pub mod error;

pub async fn get_page_ids(page: &Document) -> Result<Vec<i32>, Error> {
    let mut ids = Vec::new();
    for link_node in page.find(Class("cname")) {
        let link = link_node.attr("href").ok_or(Error::WrongFormatNode)?;
        let id_text = link
            .split_terminator("/")
            .last()
            .ok_or(Error::WrongFormatNode)?;
        let id = id_text.parse()?;
        ids.push(id)
    }
    Ok(ids)
}

pub async fn get_pages_count(page: &Document) -> Result<i32, Error> {
    let count_node = page
        .find(Class("pager-outer-wrapper").descendant(Class("dropdown")))
        .next()
        .ok_or(Error::CannotFindNode)?;
    let count_node_text = count_node.text();
    let count_string = count_node_text
        .split_whitespace()
        .last()
        .ok_or(Error::WrongFormatNode)?;
    let count: i32 = count_string.parse()?;
    Ok(count)
}

pub async fn get_company_profile(page: &Document) -> Result<CompanyProfile, Error> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::data::CompanyInfo;
    use crate::parser::*;
    use select::document::Document;

    #[tokio::test]
    async fn test_get_pages_count() {
        let page = Document::from(include_str!("../../test-data/search.html"));
        let count = get_pages_count(&page).await.unwrap();
        assert_eq!(count, 6)
    }

    #[tokio::test]
    async fn test_get_page_ids() {
        let page = Document::from(include_str!("../../test-data/search.html"));
        let ids = get_page_ids(&page).await.unwrap();
        assert_eq!(
            vec![
                1469248, 798320, 1646479, 341786, 319839, 329488, 1334271, 1579409, 1710450,
                1350802,
            ],
            ids
        )
    }
}
