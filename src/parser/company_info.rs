use crate::data::CompanyInfo;
use crate::parser::Error;
use futures::join;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub async fn get_company_info(page: &Document) -> Result<CompanyInfo, Error> {
    let (name_result, rating, website_result, founded, size, duns, other) = join!(
        get_company_name(&page),
        get_company_rating(&page),
        get_company_website(&page),
        get_company_foundation_date(&page),
        get_company_size(&page),
        get_company_duns(&page),
        get_company_other_info(&page)
    );

    let website = website_result?;
    let name = name_result?;

    Ok(CompanyInfo {
        name,
        rating,
        website,
        founded,
        size,
        duns,
        other,
    })
}

async fn get_company_name(page: &Document) -> Result<String, Error> {
    let name_node = page.find(Name("h1")).next().ok_or(Error::CannotFindNode)?;

    let name_node_text = name_node.text();
    let name = name_node_text
        .trim()
        .split_terminator("   ")
        .next()
        .ok_or(Error::WrongFormatNode)?;
    Ok(name.into())
}

async fn get_company_rating(page: &Document) -> Option<String> {
    let rating_node = page.find(Name("h1")).next()?;

    let rating_node_text = rating_node.text();
    let rating = rating_node_text.trim().split_terminator("   ");

    if rating.clone().count() > 1 {
        rating.last().map(|rating| rating.to_string())
    } else {
        None
    }
}

async fn get_company_website(page: &Document) -> Result<String, Error> {
    let website_node = page
        .find(Class("company-2").descendant(Class("extTrk")))
        .next()
        .ok_or(Error::CannotFindNode)?;
    let website = website_node.attr("href").ok_or(Error::WrongFormatNode)?;
    Ok(website.into())
}

async fn get_company_foundation_date(page: &Document) -> Option<String> {
    for node in page.find(Attr("id", "infoSection").descendant(Name("strong"))) {
        if node.text().starts_with("Founded") {
            return node.next().map(|node| node.text().trim().into());
        }
    }
    None
}

async fn get_company_size(page: &Document) -> Option<String> {
    for node in page.find(Attr("id", "infoSection").descendant(Name("strong"))) {
        if node.text().starts_with("Size") {
            return node.next().map(|node| {
                node.text()
                    .trim()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .into()
            });
        }
    }
    None
}

async fn get_company_duns(page: &Document) -> Option<String> {
    for node in page.find(Attr("id", "infoSection").descendant(Name("strong"))) {
        if node.text().starts_with("DUNS") {
            return node.next().map(|node| node.text().trim().into());
        }
    }
    None
}

async fn get_company_other_info(page: &Document) -> Vec<String> {
    let other_nodes = page.find(
        Attr("id", "infoSection")
            .descendant(Name("div"))
            .descendant(Name("div"))
            .descendant(Name("span")),
    );
    other_nodes
        .map(|node| node.text().trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::data::CompanyInfo;
    use crate::parser::company_info::*;
    use crate::parser::*;
    use select::document::Document;

    #[tokio::test]
    async fn test_get_company_other_info() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let other_info = get_company_other_info(&page).await;
        let empty_vec: Vec<String> = vec![];
        assert_eq!(other_info, empty_vec)
    }

    #[tokio::test]
    async fn test_get_company_foundation_date() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let foundation_date = get_company_foundation_date(&page).await.unwrap();
        assert_eq!(foundation_date, "1986")
    }

    #[tokio::test]
    async fn test_get_company_size() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let size = get_company_size(&page).await.unwrap();
        assert_eq!(size, "1-4 Employees".to_string())
    }

    #[tokio::test]
    async fn test_get_company_duns() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let duns = get_company_duns(&page).await;
        assert_eq!(duns, None)
    }

    #[tokio::test]
    async fn test_get_company_website() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let website = get_company_website(&page).await.unwrap();
        assert_eq!(website, "https://www.skyviewac.com".to_string())
    }

    #[tokio::test]
    async fn test_get_company_name() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let name = get_company_name(&page).await.unwrap();
        assert_eq!("Skyview Air Conditioning & Heating Corp.".to_string(), name)
    }

    #[tokio::test]
    async fn test_get_company_rating() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let rating = get_company_rating(&page).await;
        assert_eq!(rating, None)
    }

    #[tokio::test]
    async fn test_get_company_info() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let info = get_company_info(&page).await.unwrap();
        assert_eq!(
            info,
            CompanyInfo {
                name: "Skyview Air Conditioning & Heating Corp.".to_string(),
                rating: None,
                website: "https://www.skyviewac.com".to_string(),
                founded: Some("1986".to_string()),
                size: Some("1-4 Employees".to_string()),
                duns: None,
                other: vec![]
            }
        )
    }
}
