use crate::infrastructure::{frameworks::UsecaseContainer, presenters::subject};

pub async fn get_by_id(
    container: &UsecaseContainer,
    id: u32,
) -> async_graphql::Result<subject::GqlSubjectDto> {
    let entity = container.subject_usecase.get_by_id(id).await?;
    let dto = subject::from_entity(entity);
    Ok(dto)
}
