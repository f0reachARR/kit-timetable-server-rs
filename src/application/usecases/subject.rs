use crate::domain::entities::SubjectEntity;

#[async_trait::async_trait]
pub trait SubjectUsecase: Sync + Send {
    async fn get_by_id<'b>(&'b self, id: u32) -> Result<SubjectEntity, anyhow::Error>;
}
