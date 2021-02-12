use crate::domain::entities::SubjectEntity;
use std::future::Future;
use std::pin::Pin;

pub trait SubjectRepository: Sync + Send {
    fn get_by_id<'a>(
        &'a self,
        id: u32,
    ) -> Pin<Box<dyn Future<Output = Result<SubjectEntity, anyhow::Error>> + 'a>>;
}
