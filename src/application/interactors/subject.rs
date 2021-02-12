use std::{pin::Pin, sync::Arc};

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

impl SubjectUsecase for SubjectInteractor {
    fn get_by_id<'b>(
        &'b self,
        id: u32,
    ) -> Pin<Box<dyn futures::Future<Output = Result<SubjectEntity, anyhow::Error>> + 'b>> {
        self.subject_repository.get_by_id(id)
    }
}
