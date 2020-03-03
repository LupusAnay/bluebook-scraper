use crate::data::CompanyLocation;
use crate::parser::Error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, Predicate};

pub async fn get_locations(page: &Document) -> Result<Vec<CompanyLocation>, Error> {
    let mut locations = Vec::new();
    let locations_node = page
        .find(
            Name("h2")
                .and(Class("pageTitle"))
                .and(|n: &Node| n.text().trim().starts_with("Locations")),
        )
        .next()
        .and_then(|n| n.parent())
        .ok_or(Error::CannotFindNode)?;

    for node in locations_node.find(Class("row").child(Name("div").and(Class("col-12")))) {
        let address = node
            .find(Class("col-md"))
            .next()
            .and_then(|n| n.find(Name("div")).next())
            .ok_or(Error::WrongFormatNode)?
            .text()
            .trim()
            .to_string();

        let phone_number = node
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
    Ok(locations)
}

#[cfg(test)]
mod tests {
    use crate::data::CompanyLocation;
    use crate::parser::company_locations::*;

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
