use crate::data::ServiceArea;
use crate::parser::Error;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::collections::HashSet;
/*
states = page.select('#gsaCarousel')
if states:
    states = states[0].find_all('strong')
    states = [s.text.split(':')[0].strip() for s in states]
    areas = {}
    for state in states:
        areas[state] = []
        counties = page.select(f'#{state.replace(" ", "_", 1)}Counties')

        if len(counties) < 1:
            continue

        counties = counties[0].select('em, .col-6')

        for county in counties:
            areas[state].append(county.text.strip())
    return areas
*/
async fn get_company_service_areas(page: &Document) -> Result<Vec<ServiceArea>, Error> {
    let gsa_carousel = page
        .find(Attr("id", "gsaCarousel"))
        .next()
        .ok_or(Error::CannotFindNode)?;
    let states: HashSet<String> = gsa_carousel
        .find(Name("strong"))
        .filter_map(|node| {
            let text = node.text();
            let trimmed = text.split(":").next().map(|t| t.trim().to_string());
            trimmed
        })
        .collect();
    let mut areas = Vec::new();
    for state in states.iter() {
        let selector = format!("{}Counties", state.replacen(" ", "_", 1));
        let maybe_counties = page.find(Attr("id", selector.as_str())).next();
        let counties = match maybe_counties {
            Some(counties) => counties.find(Class("col-6")).map(|n| n.text().trim().to_string()).collect(),
            None => continue,
        };
        areas.push(ServiceArea {cities: counties, state_name: state.to_string()})
    }
    Ok(areas)
}

#[cfg(test)]
mod tests {
    use crate::parser::company_service_areas::*;
    use select::document::Document;
    use crate::data::ServiceArea;

    #[tokio::test]
    async fn test_get_company_service_areas() {
        let page = Document::from(include_str!("../../test-data/company.html"));
        let service_areas = get_company_service_areas(&page).await.unwrap();
        assert_eq!(service_areas, [
            ServiceArea {
                state_name: "New York".to_string(),
                cities: vec![
                    "Bronx".to_string(),
                    "Kings".to_string(),
                    "New York".to_string(),
                    "Queens".to_string(),
                    "Richmond".to_string(),
                ],
            },
        ])
    }
}