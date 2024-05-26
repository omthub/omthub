use serde::{Deserialize, Serialize};

use crate::meta::Meta;

pub const MOTHER_TONGUE_TABLE: &str = "mother_tongue";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct MotherTongueRecordId(pub ulid::Ulid);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MotherTongue {
  pub id:          MotherTongueRecordId,
  pub name:        String,
  pub description: String,
  pub is_vetted:   bool,
  pub meta:        Meta,
}