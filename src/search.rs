use serde::Serialize;
use crate::model::{Region, UniversityCategory, InstitutionCategory};

#[derive(Debug, Serialize, Default)]
pub struct SearchParams {
  pub id: Option<i32>,
  pub region: Option<Region>,
  pub university_category: Option<UniversityCategory>,
  pub institution_category: Option<InstitutionCategory>
}

impl SearchParams {
  pub fn new() -> Self {
    SearchParams {
      id: None,
      region: None,
      university_category: None,
      institution_category: None
    }
  }

  pub fn with_id(mut self, id: i32) -> Self {
    self.id = Some(id);
    self
  }

  pub fn with_region(mut self, region: Region) -> Self {
    self.region = Some(region);
    self
  }

  pub fn with_university_category(mut self, university_category: UniversityCategory) -> Self {
    self.university_category = Some(university_category);
    self
  }

  pub fn with_institution_category(mut self, institution_category: InstitutionCategory) -> Self {
    self.institution_category = Some(institution_category);
    self
  }
}