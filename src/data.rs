#[derive(Debug, PartialEq)]
pub struct CompanyLocation {
    address: String,
    phone_number: String,
}

#[derive(Debug, PartialEq)]
pub struct CompanyInfo {
    pub name: String,
    pub rating: Option<String>,
    pub website: String,
    pub founded: Option<String>,
    pub size: Option<String>,
    pub duns: Option<String>,
    pub other: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct CompanyContact {
    name: String,
    position: String,
    phone: String,
}

pub type Category = String;

pub type Project = String;

#[derive(Debug, PartialEq)]
pub struct CompanyProfile {
    pub info: CompanyInfo,
    pub locations: Vec<CompanyLocation>,
    pub contacts: Vec<CompanyContact>,
    pub story: String,
    pub categories: Vec<Category>,
    pub project_experience: Vec<Project>,
    pub emails: Vec<String>,
}
