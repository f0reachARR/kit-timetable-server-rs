use async_graphql::{Context, Object};

use crate::infrastructure::{controllers, frameworks::UsecaseContainer, presenters};

#[derive(Default)]
pub struct SubjectQueryRoot;

#[Object]
impl SubjectQueryRoot {
    async fn subject(
        &self,
        ctx: &Context<'_>,
        id: u32,
    ) -> async_graphql::Result<presenters::subject::GqlSubjectDto> {
        let container = ctx.data::<UsecaseContainer>()?;
        Ok(controllers::subject::get_by_id(container, id).await?)
    }
}
