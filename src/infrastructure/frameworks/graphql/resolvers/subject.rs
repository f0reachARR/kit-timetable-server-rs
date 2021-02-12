use async_graphql::{Context, Object, Result};

use crate::infrastructure::{frameworks::UsecaseContainer, presenters::subject};

#[derive(Default)]
pub struct SubjectQueryRoot;

#[Object]
impl SubjectQueryRoot {
    async fn subject(&self, ctx: &Context<'_>, id: u32) -> Result<subject::GqlSubjectDto> {
        let usecases = ctx.data::<UsecaseContainer>()?;
        let entity = usecases.subject_usecase.get_by_id(id).await?;
        let dto = subject::from_entity(entity);
        Ok(dto)
    }
}
