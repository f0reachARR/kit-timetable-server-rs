#[derive(Debug, PartialEq, Clone)]
pub struct UserEntity {
    pub id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
