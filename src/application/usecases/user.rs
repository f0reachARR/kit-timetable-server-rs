use crate::domain::entities::UserEntity;

#[async_trait::async_trait]
pub trait UserUsecase: Sync + Send {
    async fn get_by_token<'b>(&'b self, token: String) -> Result<UserEntity, anyhow::Error>;
}
