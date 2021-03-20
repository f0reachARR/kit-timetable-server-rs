use crate::domain::entities::UserEntity;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn get_from_token(&self, token: String) -> Result<UserEntity, anyhow::Error>;
}
