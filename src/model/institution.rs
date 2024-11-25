use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstitutionCategory {
  GeneralSecondaryEducationInstitutions = 3, // Заклади загальної середньої освіти
}

impl fmt::Display for InstitutionCategory {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", *self as i32)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
  pub institution_name: String,
  pub institution_id: String,
  pub is_checked: String,
  pub short_name: String,
  pub state_name: String,
  pub institution_type_name: String,
  pub university_financing_type_name: String,
  pub koatuu_id: String,
  pub region_name: String,
  pub koatuu_name: String,
  pub address: String,
  pub parent_institution_id: Option<String>,
  pub governance_name: String,
  pub phone: String,
  pub fax: String,
  pub email: String,
  pub website: String,
  pub boss: String,
  pub support_name: String,
  pub is_village: String,
  pub is_mountain: String,
  pub is_internat: String,
  pub approved_count: Option<String>,
}