use crate::domain::entities::SubjectEntity;

#[async_trait::async_trait]
pub trait SubjectRepository: Sync + Send {
    async fn get_by_id(&self, id: u32) -> Result<SubjectEntity, anyhow::Error>;
}
