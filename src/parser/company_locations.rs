use crate::data::CompanyLocation;
use crate::parser::Error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub async fn get_locations(page: &Document) -> Result<Vec<CompanyLocation>, Error> {
    let mut locations = Vec::new();
    for node in page.find(Name("h2").and(Class("pageTitle"))) {
        if node.text().trim().starts_with("Locations") {
            let locations_node = node
                .parent()
                .and_then(|n| n.find(Class("row")).next())
                .ok_or(Error::CannotFindNode)?;

            let address = locations_node
                .find(Class("col-md"))
                .next()
                .and_then(|n| n.find(Name("div")).next())
                .ok_or(Error::WrongFormatNode)?
                .text()
                .trim()
                .to_string();

            let phone_number = locations_node
                .find(Class("phoneDisp"))
                .next()
                .ok_or(Error::WrongFormatNode)?
                .text()
                .trim()
                .to_string();

            locations.push(CompanyLocation {
                address,
                phone_number,
            })
        }
    }
    Ok(locations)
}

async fn get_header_text(header: Node<'_>) -> String {
    let header_node_text = header.text();
    let header_text = header_node_text.trim().to_string();
    header_text
}

#[cfg(test)]
mod tests {
    use crate::data::CompanyLocation;
    use crate::parser::company_locations::*;
    use select::predicate::Name;

    #[tokio::test]
    async fn test_get_locations() {
        let locations_page = Document::from(include_str!("../../test-data/locations.html"));

        let locations = get_locations(&locations_page).await.unwrap();
        assert_eq!(
            locations,
            vec![CompanyLocation {
                address: "155 Park Ave. Amityville, NY 11701".into(),
                phone_number: "(516) 379-1800".into()
            }]
        )
    }
}
