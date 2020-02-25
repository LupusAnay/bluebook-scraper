use select::document::Document;
use crate::data::CompanyContact;
use crate::parser::Error;
use reqwest::Client;

pub async fn get_contacts(client: Client, &page: Document) -> Result<Vec<CompanyContact>, Error> {
    unimplemented!()
}
