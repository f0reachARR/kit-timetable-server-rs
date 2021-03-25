use std::sync::Arc;

use async_graphql::{Context, Object};

use crate::infrastructure::{
    controllers,
    frameworks::{RequestContainer, UsecaseContainer},
    presenters::user::GqlUserDto,
};

#[derive(Debug, Default)]
pub struct UserQueryRoot;

#[Object]
impl UserQueryRoot {
    async fn get_self_user(&self, ctx: &Context<'_>) -> async_graphql::Result<GqlUserDto> {
        let container = ctx.data::<Arc<UsecaseContainer>>()?;
        let request = ctx.data::<RequestContainer>()?;
        Ok(controllers::user::get_self_user(container, request).await?)
    }
}
