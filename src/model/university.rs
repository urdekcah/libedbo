use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UniversityCategory {
  HigherEducationInstitutions               = 1,  // Заклади вищої освіти
  VocationalEducationInstitutions,                // Заклади професійної (професійно-технічної) освіти
  SpecializedPreHigherEducationInstitutions = 9,  // Заклади фахової передвищої освіти
  ScientificInstitutes                      = 8,  // Наукові інститути (установи)
  PostgraduateEducationInstitutions         = 10, // Заклади післядипломної освіти
}

impl fmt::Display for UniversityCategory {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", *self as i32)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniversityBranch {
  pub university_name: String,
  pub university_id: String,
  pub region_name: String,
  pub katottgcodeu: String,
  pub katottg_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialityLicense {
  pub qualification_group_name: String,
  pub speciality_code: String,
  pub speciality_name: String,
  pub specialization_name: String,
  pub all_count: String,
  pub all_term_count: String,
  pub full_time_count: String,
  pub part_time_count: String,
  pub evening_count: String,
  pub certificate: String,
  pub certificate_expired: Option<String>,
  pub license_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionLicense {
  pub professions: String,
  pub license_count: String,
  pub accreditation: String,
  pub accreditation_expired: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Educator {
  pub qualification_group_name: String,
  pub speciality_code: String,
  pub speciality_name: String,
  pub specialization_name: String,
  pub full_time_count: String,
  pub part_time_count: String,
  pub external_count: String,
  pub evening_count: String,
  pub distance_count: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct University {
  pub university_name: String,
  pub university_id: String,
  pub university_parent_id: Option<String>,
  pub university_short_name: String,
  pub university_name_en: String,
  pub is_from_crimea: String,
  pub registration_year: String,
  pub university_type_name: String,
  pub university_financing_type_name: String,
  pub university_governance_type_name: String,
  pub post_index_u: String,
  pub katottgcodeu: String,
  pub katottg_name_u: String,
  pub region_name_u: String,
  pub university_address_u: String,
  pub university_phone: String,
  pub university_email: String,
  pub university_site: String,
  pub university_director_post: String,
  pub university_director_fio: String,
  pub close_date: Option<String>,
  pub branches: Vec<UniversityBranch>,
  pub facultets: Vec<String>,
  pub speciality_licenses: Vec<SpecialityLicense>,
  pub profession_licenses: Vec<ProfessionLicense>,
  pub educators: Vec<Educator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniversityBrief {
  pub university_name: String,
  pub university_id: String,
  pub university_parent_id: Option<String>,
  pub university_short_name: String,
  pub university_name_en: String,
  pub is_from_crimea: String,
  pub registration_year: String,
  pub university_type_name: String,
  pub university_financing_type_name: String,
  pub university_governance_type_name: String,
  pub post_index_u: String,
  pub katottgcodeu: String,
  pub katottg_name_u: String,
  pub region_name_u: String,
  pub university_address_u: String,
  pub university_phone: String,
  pub university_email: String,
  pub university_site: String,
  pub university_director_post: String,
  pub university_director_fio: String,
  pub close_date: Option<String>,
  pub primitki: String
}