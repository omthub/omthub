use serde::{Deserialize, Serialize};

pub const MOTHER_TONGUE_TABLE: &str = "mother_tongues";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct MotherTongueRecordId(pub ulid::Ulid);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MotherTongue {
  pub id:          MotherTongueRecordId,
  pub name:        String,
  pub description: String,
  pub is_vetted:   bool,
}
