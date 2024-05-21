#[cfg(feature = "ssr")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::meta::Meta;

#[derive(Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Queryable, Selectable, Insertable))]
#[cfg_attr(feature = "ssr", diesel(table_name = crate::schema::artifacts))]
#[cfg_attr(feature = "ssr", diesel(check_for_backend(diesel::pg::Pg)))]
pub struct Artifact {
  #[cfg_attr(feature = "ssr", diesel(deserialize_as = crate::utils::UlidWrapper))]
  #[cfg_attr(feature = "ssr", diesel(serialize_as = String))]
  pub id:         ulid::Ulid,
  pub object_key: String,
  #[cfg_attr(feature = "ssr", diesel(serialize_as = String))]
  pub meta:       Meta,
}
