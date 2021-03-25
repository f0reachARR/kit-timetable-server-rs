use crate::infrastructure::{
    frameworks::{RequestContainer, UsecaseContainer},
    presenters::user::GqlUserDto,
};

pub async fn get_self_user(
    _container: &UsecaseContainer,
    request: &RequestContainer,
) -> async_graphql::Result<GqlUserDto> {
    match &request.user {
        Some(user) => Ok(GqlUserDto::from_entity(user.clone())),
        None => Err(async_graphql::Error::from("Not logged in")),
    }
}
