use crate::data::CompanyContact;
use crate::parser::Error;
use reqwest::Client;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

pub async fn get_contacts(page: &Document) -> Result<Vec<CompanyContact>, Error> {
    let mut contacts = Vec::new();
    let contacts_node = page
        .find(Attr("id", "keyContactSection"))
        .next()
        .ok_or(Error::CannotFindNode)?;

    for node in contacts_node.find(Class("row").child(Name("div"))) {
        let data_node = node
            .find(Class("media-body"))
            .next()
            .ok_or(Error::CannotFindNode)?;
        let name = data_node
            .find(Name("strong"))
            .next()
            .map(|n| n.text().trim().to_string())
            .ok_or(Error::WrongFormatNode)?;
        let position = data_node
            .find(Name("small"))
            .next()
            .map(|n| n.text().trim().to_string())
            .unwrap_or("".to_string());
        let phone = data_node
            .find(Class("phoneDisp"))
            .next()
            .ok_or(Error::CannotFindNode)?
            .text()
            .trim()
            .to_string();

        contacts.push(CompanyContact {
            name,
            position,
            phone,
        })
    }

    Ok(contacts)
}

#[cfg(test)]
mod tests {
    use crate::data::CompanyContact;
    use crate::parser::company_contacts::*;

    #[tokio::test]
    async fn test_get_contacts() {
        let contacts_page = Document::from(include_str!("../../test-data/contacts.html"));

        let contacts = get_contacts(&contacts_page).await.unwrap();
        assert_eq!(
            vec![
                CompanyContact {
                    name: "Chris Hansen".into(),
                    position: "Pres.".into(),
                    phone: "(845) 562-3332".into(),
                },
                CompanyContact {
                    name: "Richard Rebusmen".into(),
                    position: "".into(),
                    phone: "(845) 562-3332".into(),
                },
            ],
            contacts
        )
    }
}
