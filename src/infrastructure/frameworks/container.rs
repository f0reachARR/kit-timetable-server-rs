use std::sync::Arc;

use crate::{
    application::usecases::{SubjectUsecase, UserUsecase},
    domain::entities::UserEntity,
};

pub struct UsecaseContainer {
    pub subject_usecase: Arc<dyn SubjectUsecase>,
    pub user_usecase: Arc<dyn UserUsecase>,
}

#[derive(Debug, Default)]
pub struct RequestContainer {
    pub user: Option<UserEntity>,
}
