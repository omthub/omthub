use std::fmt::Debug;

#[cfg(feature = "ssr")]
use diesel::prelude::*;

use crate::meta::Meta;

#[cfg(feature = "ssr")]
#[derive(Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  #[diesel(deserialize_as = crate::utils::UlidWrapper)]
  #[diesel(serialize_as = String)]
  pub id:        ulid::Ulid,
  pub name:      String,
  pub email:     String,
  pub pw_hash:   String,
  pub is_active: bool,
  #[diesel(serialize_as = String)]
  pub meta:      Meta,
}

#[cfg(feature = "ssr")]
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

#[derive(Clone, Debug)]
pub struct PublicUser {
  pub id:        ulid::Ulid,
  pub name:      String,
  pub email:     String,
  pub is_active: bool,
  pub meta:      Meta,
}

#[cfg(feature = "ssr")]
impl From<User> for PublicUser {
  fn from(value: User) -> Self {
    PublicUser {
      id:        value.id,
      name:      value.name,
      email:     value.email,
      is_active: value.is_active,
      meta:      value.meta,
    }
  }
}

#[derive(Clone, Debug)]
pub struct LoggedInUser(pub Option<PublicUser>);

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