use crate::{
    application::{repositories::UserRepository, usecases::UserUsecase},
    domain::entities::UserEntity,
};
use std::sync::Arc;

pub struct UserInteractor {
    user_repository: Arc<dyn UserRepository>,
}

impl UserInteractor {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl UserUsecase for UserInteractor {
    async fn get_by_token<'b>(&'b self, token: String) -> Result<UserEntity, anyhow::Error> {
        if token.len() == 0 {
            return Err(anyhow::anyhow!("Token is not provided"));
        }

        Ok(self.user_repository.get_from_token(token).await?)
    }
}
