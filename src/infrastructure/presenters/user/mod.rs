use crate::domain::entities::UserEntity;

mod dto;

pub use dto::GqlUserDto;

#[cfg(test)]
mod test;

impl GqlUserDto {
    pub fn from_entity(entity: UserEntity) -> Self {
        Self {
            id: entity.id.into(),
            created_at: entity.created_at.naive_utc(),
        }
    }
}
