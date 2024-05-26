use serde::{Deserialize, Serialize};

use crate::meta::Meta;

pub const ARTIFACT_TABLE: &str = "artifacts";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct ArtifactRecordId(pub ulid::Ulid);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artifact {
  pub id:         ArtifactRecordId,
  pub object_key: String,
  pub meta:       Meta,
}
