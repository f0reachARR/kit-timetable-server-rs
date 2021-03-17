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

    async fn subject_search(
        &self,
        ctx: &Context<'_>,
        from: u32,
        count: u32,
        query: controllers::subject::GqlSubjectSearchInput,
    ) -> async_graphql::Result<presenters::subject::GqlSubjectSearchResult> {
        let container = ctx.data::<UsecaseContainer>()?;
        Ok(controllers::subject::search(container, from, count, query).await?)
    }

    async fn subject_search_terms(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<presenters::subject::GqlSubjectSearchTerms> {
        let container = ctx.data::<UsecaseContainer>()?;
        Ok(controllers::subject::search_terms(container).await?)
    }
}
