use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Region {
  RepublicOfCrimea     = 1,  // Автономна Республіка Крим
  VinnytsiaOblast      = 5,  // Вінницька область
  VolynOblast          = 7,  // Волинська область
  DnipropetrovskOblast = 12, // Дніпропетровська область
  DonetskOblast        = 14, // Донецька область
  ZhytomyrOblast       = 18, // Житомирська область
  ZakarpattiaOblast    = 21, // Закарпатська область
  ZaporizhzhiaOblast   = 23, // Запорізька область
  IvanoFrankivskOblast = 26, // Івано-Франківська область
  KyivOblast           = 32, // Київська область
  KirovohradOblast     = 35, // Кіровоградська область
  LuhanskOblast        = 44, // Луганська область
  LvivOblast           = 46, // Львівська область
  MykolaivOblast       = 48, // Миколаївська область
  OdesaOblast          = 51, // Одеська область
  PoltavaOblast        = 53, // Полтавська область
  RivneOblast          = 56, // Рівненська область
  SumyOblast           = 59, // Сумська область
  TernopilOblast       = 61, // Тернопільська область
  KharkivOblast        = 63, // Харківська область
  KhersonOblast        = 65, // Херсонська область
  KhmelnytskyiOblast   = 68, // Хмельницька область
  CherkasyOblast       = 71, // Черкаська область
  ChernivtsiOblast     = 73, // Чернівецька область
  ChernihivOblast      = 74, // Чернігівська область
  KyivCity             = 80, // м. Київ
  SevastopolCity       = 85  // м. Севастополь
}

impl fmt::Display for Region {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", *self as i32)
  }
}