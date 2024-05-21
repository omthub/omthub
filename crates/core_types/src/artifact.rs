#[cfg(feature = "ssr")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::meta::Meta;

#[derive(Clone, Serialize, Deserialize)]
#[cfg(feature = "ssr")]
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::artifacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Artifact {
  #[diesel(deserialize_as = crate::utils::UlidWrapper)]
  #[diesel(serialize_as = String)]
  pub id:         ulid::Ulid,
  pub object_key: String,
  #[diesel(serialize_as = String)]
  pub meta:       Meta,
}
