use crate::error::ServiceError;
use async_trait::async_trait;
use futures::TryStreamExt;
use model::plant::Plant;
use mongodb::{bson::Document, Collection};

#[async_trait]
pub trait PlantService {
    async fn get_all(&self) -> Result<Vec<Plant>, ServiceError>;
}

#[derive(Debug, Clone)]
pub struct PlantServiceImpl {
    pub collection: Collection<Plant>,
}

#[async_trait]
impl PlantService for PlantServiceImpl {
    async fn get_all(&self) -> Result<Vec<Plant>, ServiceError> {
        let res = self.collection.find(Document::new()).await?;
        res.try_collect().await.map_err(Into::into)
    }
}
