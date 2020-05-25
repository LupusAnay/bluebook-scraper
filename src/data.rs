#[derive(Debug, PartialEq)]
pub struct CompanyLocation {
    pub address: String,
    pub phone_number: String,
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
    pub annual_vol: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct CompanyContact {
    pub name: String,
    pub position: String,
    pub phone: String,
}

#[derive(Debug, PartialEq)]
pub struct ServiceArea {
    pub state_name: String,
    pub cities: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Brand {
    pub logo: String,
    pub name: String,
}

pub type Category = String;

#[derive(Debug, PartialEq)]
pub struct ProjectExperience {
    pub general: Vec<String>,
    pub commercial: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Qualification {
    name: String,
    issuer: Option<String>,
    qualifier: Option<String>,
    expiration: Option<String>
}

#[derive(Debug, PartialEq)]
pub struct Project {
    name: String,
    image: Option<ImageLink>,
    location: Option<String>,
    completed: Option<bool>
}

pub type ImageLink = String;

#[derive(Debug, PartialEq)]
pub struct CompanyProfile {
    pub info: CompanyInfo,
    pub locations: Vec<CompanyLocation>,
    pub contacts: Vec<CompanyContact>,
    pub story: String,
    pub categories: Vec<Category>,
    pub service_areas: Vec<ServiceArea>,
    pub project_experience: ProjectExperience,
    pub emails: Vec<String>,
    pub logo: Option<ImageLink>,
    pub preferred_brands: Vec<Brand>,
    pub social_media: Vec<String>,
    pub qualifications: Vec<Qualification>,
    pub media_gallery: Vec<ImageLink>,
    pub projects: Vec<Project>
}
