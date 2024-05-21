use std::sync::Arc;

use eyre::{Context, Result};
use object_store::{aws::AmazonS3Builder, ObjectStore};

pub struct FetchedArtifact {
  db_object: core_types::Artifact,
  blob:      bytes::Bytes,
}

fn get_object_store() -> Result<Arc<dyn ObjectStore>> {
  let r2 = AmazonS3Builder::new()
    .with_url(
      std::env::var("R2_URL").wrap_err("failed to get `R2_URL` env var")?,
    )
    .with_access_key_id(
      std::env::var("R2_ACCESS_KEY")
        .wrap_err("failed to get `R2_ACCESS_KEY` env var")?,
    )
    .with_secret_access_key(
      std::env::var("R2_SECRET_ACCESS_KEY")
        .wrap_err("failed to get `R2_SECRET_ACCESS_KEY` env var")?,
    )
    .build()
    .wrap_err("failed to build object store")?;

  Ok(Arc::new(r2))
}

pub async fn fetch_object(
  db_object: &core_types::Artifact,
) -> Result<Option<FetchedArtifact>> {
  let object_store = get_object_store()?;

  let path = object_store::path::Path::from(db_object.object_key.clone());
  let blob = object_store.get(&path).await;

  match blob {
    Ok(blob) => Ok(Some(FetchedArtifact {
      db_object: db_object.clone(),
      blob:      blob
        .bytes()
        .await
        .wrap_err("failed to fetch all bytes of blob")?,
    })),
    Err(e) => match e {
      object_store::Error::NotFound { .. } => Ok(None),
      _ => Err(e.into()),
    },
  }
}
