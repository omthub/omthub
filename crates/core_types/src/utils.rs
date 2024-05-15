use diesel::{
  backend::Backend,
  deserialize::{FromSql, FromSqlRow},
  sql_types::Text,
};
use serde::Deserialize;

#[derive(Deserialize, FromSqlRow)]
pub struct UlidWrapper(String);

impl TryFrom<UlidWrapper> for ulid::Ulid {
  type Error = ulid::DecodeError;

  fn try_from(value: UlidWrapper) -> Result<Self, Self::Error> {
    ulid::Ulid::from_string(&value.0)
  }
}

impl<DB> FromSql<Text, DB> for UlidWrapper
where
  DB: Backend,
  String: FromSql<Text, DB>,
{
  fn from_sql(
    bytes: <DB as Backend>::RawValue<'_>,
  ) -> diesel::deserialize::Result<Self> {
    Ok(UlidWrapper(String::from_sql(bytes)?))
  }
}
