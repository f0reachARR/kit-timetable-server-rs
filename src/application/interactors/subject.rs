use std::pin::Pin;

use crate::{
    application::{repositories::SubjectRepository, usecases::SubjectUsecase},
    domain::entities::SubjectEntity,
};

pub struct SubjectInteractor<'a, T1>
where
    T1: SubjectRepository,
{
    subject_repository: &'a T1,
}

impl<'a, T1: Send + Sync + SubjectRepository> SubjectInteractor<'a, T1> {
    pub fn new<'b>(subject_repository: &'b T1) -> SubjectInteractor<'b, T1> {
        SubjectInteractor::<'b, T1> { subject_repository }
    }
}

impl<'a, T1: Send + Sync + SubjectRepository> SubjectUsecase for SubjectInteractor<'a, T1> {
    fn get_by_id<'b>(
        &'b self,
        id: u32,
    ) -> Pin<Box<dyn futures::Future<Output = Result<SubjectEntity, anyhow::Error>> + 'b>> {
        self.subject_repository.get_by_id(id)
    }
}
