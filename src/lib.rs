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

/// Validates that an Option contains a value and returns it, or an error with the field name.
///
/// This helper function is used internally to ensure required search parameters are provided.
///
/// # Arguments
///
/// * `option` - The Option to check
/// * `field` - Name of the field being checked, used in error message
///
/// # Returns
///
/// * `Ok(T)` - The value contained in the Option
/// * `Err(Error)` - An error indicating which field was missing
///
/// # Examples
///
/// ```rust
/// let value: Option<i32> = Some(42);
/// let result = assert_some(value, "example_field")?;
/// assert_eq!(result, 42);
/// ```
fn assert_some<T>(option: Option<T>, field: &str) -> Result<T, Error> {
  option.ok_or_else(|| Error::OtherError(format!("{} cannot be None", field)))
}

/// Makes an asynchronous HTTP GET request to the EDBO API and deserializes the response.
///
/// # Arguments
///
/// * `url` - The complete URL to request, including query parameters
///
/// # Returns
///
/// * `Ok(T)` - Successfully deserialized response
/// * `Err(Error)` - Request or deserialization error
///
/// # Type Parameters
///
/// * `T` - The type to deserialize the response into, must implement DeserializeOwned
///
/// # Notes
///
/// This function will return an error if:
/// - The HTTP request fails
/// - The response status is not successful (2xx)
/// - The response cannot be deserialized into type T
async fn make_request<T: DeserializeOwned>(url: String) -> Result<T, Error> {
  let response = Client::new().get(&url).send().await?;
  if response.status().is_success() {
    Ok(response.json().await?)
  } else {
    Err(Error::ApiError(response.status().as_u16()))
  }
}

/// Makes a blocking HTTP GET request to the EDBO API and deserializes the response.
///
/// This is the blocking version of `make_request`.
///
/// # Arguments
///
/// * `url` - The complete URL to request, including query parameters
///
/// # Returns
///
/// * `Ok(T)` - Successfully deserialized response
/// * `Err(Error)` - Request or deserialization error
///
/// # Type Parameters
///
/// * `T` - The type to deserialize the response into, must implement DeserializeOwned
fn make_request_blocking<T: DeserializeOwned>(url: String) -> Result<T, Error> {
  let response = blocking::Client::new().get(&url).send()?;
  if response.status().is_success() {
    Ok(response.json()?)
  } else {
    Err(Error::ApiError(response.status().as_u16()))
  }
}

/// Asynchronously searches for universities based on provided parameters.
///
/// # Arguments
///
/// * `param` - Search parameters including region and university category
///
/// # Returns
///
/// * `Ok(Vec<UniversityBrief>)` - List of universities matching the search criteria
/// * `Err(Error)` - Search request failed
///
/// # Examples
///
/// ```rust
/// use libedbo::{SearchParams, Region, UniversityCategory};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let params = SearchParams::new()
///         .with_region(Region::KyivCity)
///         .with_university_category(UniversityCategory::HigherEducationInstitutions);
///
///     let universities = libedbo::search_universities_async(params).await?;
///     Ok(())
/// }
/// ```
pub async fn search_universities_async(param: SearchParams) -> Result<Vec<UniversityBrief>, Error> {
  let ut = assert_some(param.university_category, "university_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{UNIVERSITIES_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request(url).await
}

/// Searches for universities based on provided parameters (blocking version).
///
/// This is the blocking version of `search_universities_async`.
///
/// # Arguments
///
/// * `param` - Search parameters including region and university category
///
/// # Returns
///
/// * `Ok(Vec<UniversityBrief>)` - List of universities matching the search criteria
/// * `Err(Error)` - Search request failed
///
/// # Examples
///
/// ```rust
/// use libedbo::{SearchParams, Region, UniversityCategory};
///
/// let params = SearchParams::new()
///     .with_region(Region::LvivOblast)
///     .with_university_category(UniversityCategory::HigherEducationInstitutions);
///
/// let universities = libedbo::search_universities(params)?;
/// ```
pub fn search_universities(param: SearchParams) -> Result<Vec<UniversityBrief>, Error> {
  let ut = assert_some(param.university_category, "university_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{UNIVERSITIES_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request_blocking(url)
}

/// Asynchronously retrieves detailed information about a specific university.
///
/// # Arguments
///
/// * `param` - Search parameters containing the university ID
///
/// # Returns
///
/// * `Ok(University)` - Detailed university information
/// * `Err(Error)` - Request failed or invalid ID
///
/// # Examples
///
/// ```rust
/// use libedbo::SearchParams;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let params = SearchParams::new().with_id(1234);
///     let university = libedbo::search_university_async(params).await?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The ID is missing or less than 1
/// - The API request fails
/// - The university is not found
pub async fn search_university_async(param: SearchParams) -> Result<University, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("University ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{UNIVERSITY_ENDPOINT}?id={id}&exp=json");
  make_request(url).await
}

/// Retrieves detailed information about a specific university (blocking version).
///
/// This is the blocking version of `search_university_async`.
///
/// # Arguments
///
/// * `param` - Search parameters containing the university ID
///
/// # Returns
///
/// * `Ok(University)` - Detailed university information
/// * `Err(Error)` - Request failed or invalid ID
///
/// # Examples
///
/// ```rust
/// use libedbo::SearchParams;
///
/// let params = SearchParams::new().with_id(1234);
/// let university = libedbo::search_university(params)?;
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The ID is missing or less than 1
/// - The API request fails
/// - The university is not found
pub fn search_university(param: SearchParams) -> Result<University, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("University ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{UNIVERSITY_ENDPOINT}?id={id}&exp=json");
  make_request_blocking(url)
}

/// Asynchronously searches for secondary education institutions based on provided parameters.
///
/// # Arguments
///
/// * `param` - Search parameters including region and institution category
///
/// # Returns
///
/// * `Ok(Vec<Institution>)` - List of institutions matching the search criteria
/// * `Err(Error)` - Search request failed
///
/// # Examples
///
/// ```rust
/// use libedbo::{SearchParams, Region, InstitutionCategory};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let params = SearchParams::new()
///         .with_region(Region::KyivCity)
///         .with_institution_category(InstitutionCategory::GeneralSecondaryEducationInstitutions);
///
///     let schools = libedbo::search_institutions_async(params).await?;
///     Ok(())
/// }
/// ```
pub async fn search_institutions_async(param: SearchParams) -> Result<Vec<Institution>, Error> {
  let ut = assert_some(param.institution_category, "institution_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{INSTITUTIONS_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request(url).await
}

/// Searches for secondary education institutions based on provided parameters (blocking version).
///
/// This is the blocking version of `search_institutions_async`.
///
/// # Arguments
///
/// * `param` - Search parameters including region and institution category
///
/// # Returns
///
/// * `Ok(Vec<Institution>)` - List of institutions matching the search criteria
/// * `Err(Error)` - Search request failed
///
/// # Examples
///
/// ```rust
/// use libedbo::{SearchParams, Region, InstitutionCategory};
///
/// let params = SearchParams::new()
///     .with_region(Region::LvivOblast)
///     .with_institution_category(InstitutionCategory::GeneralSecondaryEducationInstitutions);
///
/// let schools = libedbo::search_institutions(params)?;
/// ```
pub fn search_institutions(param: SearchParams) -> Result<Vec<Institution>, Error> {
  let ut = assert_some(param.institution_category, "institution_category")?;
  let lc = assert_some(param.region, "region")?;
  let url = format!("{BASE_URL}{INSTITUTIONS_ENDPOINT}?ut={ut}&lc={lc}&exp=json");
  make_request_blocking(url)
}

/// Asynchronously retrieves detailed information about a specific school.
///
/// # Arguments
///
/// * `param` - Search parameters containing the school ID
///
/// # Returns
///
/// * `Ok(Institution)` - Detailed school information
/// * `Err(Error)` - Request failed or invalid ID
///
/// # Examples
///
/// ```rust
/// use libedbo::SearchParams;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let params = SearchParams::new().with_id(1234);
///     let school = libedbo::search_school_async(params).await?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The ID is missing or less than 1
/// - The API request fails
/// - The school is not found
pub async fn search_school_async(param: SearchParams) -> Result<Institution, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("School ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{SCHOOL_ENDPOINT}?id={id}&exp=json");
  make_request(url).await
}

/// Retrieves detailed information about a specific school (blocking version).
///
/// This is the blocking version of `search_school_async`.
///
/// # Arguments
///
/// * `param` - Search parameters containing the school ID
///
/// # Returns
///
/// * `Ok(Institution)` - Detailed school information
/// * `Err(Error)` - Request failed or invalid ID
///
/// # Examples
///
/// ```rust
/// use libedbo::SearchParams;
///
/// let params = SearchParams::new().with_id(1234);
/// let school = libedbo::search_school(params)?;
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The ID is missing or less than 1
/// - The API request fails
/// - The school is not found
pub fn search_school(param: SearchParams) -> Result<Institution, Error> {
  let id = assert_some(param.id, "id")?;
  if id < 1 {
    return Err(Error::OtherError("School ID must be positive".to_string()));
  }
  let url = format!("{BASE_URL}{SCHOOL_ENDPOINT}?id={id}&exp=json");
  make_request_blocking(url)
}