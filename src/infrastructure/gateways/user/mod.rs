use crate::{application::repositories::UserRepository, domain::entities::UserEntity};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub struct UserGateway {
    token_secret: String,
    token_issuer: String,
}

impl UserGateway {
    pub fn new(token_secret: String, token_issuer: String) -> UserGateway {
        Self {
            token_secret,
            token_issuer,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TokenClaim {
    pub sub: String,
}

#[async_trait::async_trait]
impl UserRepository for UserGateway {
    async fn get_from_token(&self, token: String) -> Result<UserEntity, anyhow::Error> {
        let decoded = jsonwebtoken::decode::<TokenClaim>(
            token.as_str(),
            &DecodingKey::from_secret(self.token_secret.as_bytes()),
            &Validation {
                iss: self.token_issuer.clone().into(),
                ..Default::default()
            },
        )?;

        Ok(UserEntity {
            id: decoded.claims.sub,
            created_at: chrono::Utc::now(), // TODO
        })
    }
}
