use crate::infrastructure::{frameworks::UsecaseContainer, presenters::subject};

mod dto;
pub use dto::*;

pub async fn get_by_id(
    container: &UsecaseContainer,
    id: u32,
) -> async_graphql::Result<subject::GqlSubjectDto> {
    let entity = container.subject_usecase.get_by_id(id).await?;
    let dto = subject::from_entity(entity);
    Ok(dto)
}

pub async fn search(
    container: &UsecaseContainer,
    from: u32,
    count: u32,
    query: dto::GqlSubjectSearchInput,
) -> async_graphql::Result<subject::GqlSubjectSearchResult> {
    let param = query
        .into_usecase_param(from, count)
        .ok_or_else(|| async_graphql::Error::new("Invalid param"))?;
    let result = container.subject_usecase.search(&param).await?;
    let dto = subject::convert_search_result(result);
    Ok(dto)
}

pub async fn search_terms(
    container: &UsecaseContainer,
) -> async_graphql::Result<subject::GqlSubjectSearchTerms> {
    let entity = container.subject_usecase.get_terms().await?;
    let dto = subject::GqlSubjectSearchTerms::from_entity(entity);
    Ok(dto)
}
