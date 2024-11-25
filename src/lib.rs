use reqwest::{blocking, Client};
use serde::de::DeserializeOwned;

mod model;
mod search;
pub mod error;
pub use model::*;
pub use search::*;
use error::Error;

const BASE_URL: &str = "https://registry.edbo.gov.ua";
const UNIVERSITIES_ENDPOINT: &str = "/api/universities";
const UNIVERSITY_ENDPOINT: &str = "/api/university";
const INSTITUTIONS_ENDPOINT: &str = "/api/institutions";
const SCHOOL_ENDPOINT: &str = "/api/school";

fn assert_some<T>(option: Option<T>, field: &str) -> Result<T, Error> {
  option.ok_or_else(|| Error::OtherError(format!("{} cannot be None", field)))
}

async fn make_request<T: DeserializeOwned>(url: String) -> Result<T, Error> {
  let response = Client::new().get(&url).send().await?;
  if response.status().is_success() {
    Ok(response.json().await?)
  } else {
    Err(Error::ApiError(response.status().as_u16()))
  }
}

fn make_request_blocking<T: DeserializeOwned>(url: String) -> Result<T, Error> {
  let response = blocking::Client::new().get(&url).send()?;
  if response.status().is_success() {
    Ok(response.json()?)
  } else {
    Err(Error::ApiError(response.status().as_u16()))
  }
}

pub async fn search_universities_async(param: SearchParams) -> Result<Vec<UniversityBrief>, Error> {
  let ut = assert_some(param.university_category, "university_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{UNIVERSITIES_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request(url).await
}

pub fn search_universities(param: SearchParams) -> Result<Vec<UniversityBrief>, Error> {
  let ut = assert_some(param.university_category, "university_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{UNIVERSITIES_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request_blocking(url)
}

pub async fn search_university_async(param: SearchParams) -> Result<University, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("University ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{UNIVERSITY_ENDPOINT}?id={id}&exp=json");
  make_request(url).await
}

pub fn search_university(param: SearchParams) -> Result<University, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("University ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{UNIVERSITY_ENDPOINT}?id={id}&exp=json");
  make_request_blocking(url)
}

pub async fn search_institutions_async(param: SearchParams) -> Result<Vec<Institution>, Error> {
  let ut = assert_some(param.institution_category, "institution_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{INSTITUTIONS_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request(url).await
}

pub fn search_institutions(param: SearchParams) -> Result<Vec<Institution>, Error> {
  let ut = assert_some(param.institution_category, "institution_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{INSTITUTIONS_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request_blocking(url)
}

pub async fn search_school_async(param: SearchParams) -> Result<Institution, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("School ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{SCHOOL_ENDPOINT}?id={id}&exp=json");
  make_request(url).await
}

pub fn search_school(param: SearchParams) -> Result<Institution, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("School ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{SCHOOL_ENDPOINT}?id={id}&exp=json");
  make_request_blocking(url)
}