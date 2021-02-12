use std::sync::Arc;

use crate::{
    application::{repositories::SubjectRepository, usecases::SubjectUsecase},
    domain::entities::SubjectEntity,
};

pub struct SubjectInteractor {
    subject_repository: Arc<dyn SubjectRepository>,
}

impl SubjectInteractor {
    pub fn new(subject_repository: Arc<dyn SubjectRepository>) -> SubjectInteractor {
        SubjectInteractor { subject_repository }
    }
}

#[async_trait::async_trait]
impl SubjectUsecase for SubjectInteractor {
    async fn get_by_id<'b>(&'b self, id: u32) -> Result<SubjectEntity, anyhow::Error> {
        self.subject_repository.get_by_id(id).await
    }
}
