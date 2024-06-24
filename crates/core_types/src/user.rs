use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub const USER_TABLE: &str = "users";

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ssr", serde(from = "crate::ssr::UlidOrThing"))]
pub struct UserRecordId(pub ulid::Ulid);

#[cfg(feature = "ssr")]
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
  pub id:            UserRecordId,
  pub name:          String,
  pub email:         String,
  pub pw_hash:       String,
  pub is_active:     bool,
  #[serde(with = "iso8601")]
  pub registered_at: time::OffsetDateTime,
}

mod iso8601 {
  use serde::{self, Deserialize, Deserializer, Serializer};
  use time::{format_description::well_known::Iso8601, OffsetDateTime};

  pub fn serialize<S>(
    date: &OffsetDateTime,
    serializer: S,
  ) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let s = date.format(&Iso8601::DEFAULT).unwrap();
    serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D>(
    deserializer: D,
  ) -> Result<OffsetDateTime, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    let dt = OffsetDateTime::parse(&s, &Iso8601::DEFAULT)
      .map_err(serde::de::Error::custom)?;
    Ok(dt)
  }
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
      .finish()
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
  pub id:            UserRecordId,
  pub name:          String,
  pub email:         String,
  pub is_active:     bool,
  #[serde(with = "iso8601")]
  pub registered_at: time::OffsetDateTime,
}

#[cfg(feature = "ssr")]
impl From<User> for PublicUser {
  fn from(value: User) -> Self {
    PublicUser {
      id:            value.id,
      name:          value.name,
      email:         value.email,
      is_active:     value.is_active,
      registered_at: value.registered_at,
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

    fn id(&self) -> <Self as AuthUser>::Id { self.id.0 }
    fn session_auth_hash(&self) -> &[u8] { self.pw_hash.as_bytes() }
  }
}
