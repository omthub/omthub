use std::collections::HashMap;
#[cfg(feature = "ssr")]
use std::error::Error;

#[cfg(feature = "ssr")]
use diesel::{
  backend::Backend,
  deserialize::{FromSql, FromSqlRow},
  sql_types::Text,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromSqlRow))]
pub struct Meta(HashMap<String, serde_json::Value>);

impl Meta {
  pub fn new() -> Meta {
    let mut map = HashMap::new();
    map.insert(
      "created_at".to_string(),
      serde_json::Value::Number(
        time::OffsetDateTime::now_utc().unix_timestamp().into(),
      ),
    );

    Meta(map)
  }
}

impl Default for Meta {
  fn default() -> Self { Self::new() }
}

#[cfg(feature = "ssr")]
impl<DB> FromSql<Text, DB> for Meta
where
  DB: Backend,
  String: FromSql<Text, DB>,
{
  fn from_sql(
    bytes: <DB as Backend>::RawValue<'_>,
  ) -> diesel::deserialize::Result<Self> {
    let string = String::from_sql(bytes)?;
    let value: serde_json::Value = serde_json::from_str(&string)?;

    let map = match value {
      serde_json::Value::Object(m) => m.into_iter().collect::<HashMap<_, _>>(),
      _ => Err(Box::<dyn Error + Send + Sync>::from(String::from(
        "wrong JSON type",
      )))?,
    };

    Ok(Meta(map))
  }
}

#[cfg(feature = "ssr")]
impl From<Meta> for String {
  fn from(value: Meta) -> Self {
    use eyre::Context;
    serde_json::to_string(&value)
      .wrap_err("failed to serialize meta object")
      .unwrap()
  }
}
