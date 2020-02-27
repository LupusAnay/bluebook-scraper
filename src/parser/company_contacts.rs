use crate::data::CompanyContact;
use crate::parser::Error;
use reqwest::Client;
use select::document::Document;
use select::predicate::Class;

pub async fn get_contacts(client: Client, page: &Document) -> Result<Vec<CompanyContact>, Error> {
    let address = page
        .find(Class("keyContactSection"))
        .next()
        .ok_or(Error::CannotFindNode)?;
    unimplemented!()
}
