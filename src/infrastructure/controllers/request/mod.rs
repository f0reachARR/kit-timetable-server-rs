use crate::infrastructure::frameworks::{RequestContainer, UsecaseContainer};

pub async fn create_request_container(
    container: &UsecaseContainer,
    auth_header: &Option<String>,
) -> Result<RequestContainer, anyhow::Error> {
    let mut request_container = RequestContainer::default();
    if let Some(auth) = auth_header {
        let token = auth
            .strip_prefix("Bearer ")
            .ok_or_else(|| anyhow::anyhow!("Invalid header"))?;

        let user = container
            .user_usecase
            .get_by_token(token.to_string())
            .await?;
        request_container.user = user.into();
    }

    Ok(request_container)
}
