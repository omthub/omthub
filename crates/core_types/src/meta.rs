use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
