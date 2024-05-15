use std::fmt::Debug;

#[cfg(feature = "ssr")]
use diesel::prelude::*;

use crate::meta::Meta;

#[derive(Clone)]
#[cfg_attr(feature = "ssr",
  derive(Queryable, Selectable, Insertable),
  diesel(table_name = crate::schema::users),
  diesel(check_for_backend(diesel::pg::Pg)),
)]
pub struct User {
  #[cfg_attr(feature = "ssr", diesel(deserialize_as = crate::utils::UlidWrapper), diesel(serialize_as = String))]
  pub id:        ulid::Ulid,
  pub name:      String,
  pub email:     String,
  pub pw_hash:   String,
  pub is_active: bool,
  #[cfg_attr(feature = "ssr", diesel(serialize_as = String))]
  pub meta:      Meta,
}

impl Debug for User {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("User")
      .field("id", &self.id)
      .field("name", &self.name)
      .field("email", &"[redacted]")
      .field("pw_hash", &"[redacted]")
      .field("is_active", &self.is_active)
      .field("meta", &self.meta)
      .finish()
  }
}

#[cfg(feature = "auth")]
mod auth {
  use axum_login::AuthUser;

  use super::User;

  impl AuthUser for User {
    type Id = ulid::Ulid;

    fn id(&self) -> <Self as AuthUser>::Id { self.id }
    fn session_auth_hash(&self) -> &[u8] { self.pw_hash.as_bytes() }
  }
}
