use crate::domain::entities::SubjectEntity;
use futures::Future;
use std::pin::Pin;

pub trait SubjectUsecase {
    fn get_by_id<'b>(
        &'b self,
        id: u32,
    ) -> Pin<Box<dyn Future<Output = Result<SubjectEntity, anyhow::Error>> + 'b>>;
}
