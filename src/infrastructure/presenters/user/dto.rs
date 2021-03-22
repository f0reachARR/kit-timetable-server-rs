use async_graphql::{SimpleObject, ID};

#[derive(Debug, PartialEq, Eq, Clone, SimpleObject)]
pub struct GqlUserDto {
    pub id: ID,
    pub created_at: chrono::NaiveDateTime,
}
