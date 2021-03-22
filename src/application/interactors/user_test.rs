use crate::{
    application::{
        interactors::UserInteractor, repositories::MockUserRepository, usecases::UserUsecase,
    },
    domain::entities::user_td,
};
use mockall::predicate;
use std::sync::Arc;

#[tokio::test]
async fn get_by_token_test() {
    let mut mock_repo = MockUserRepository::new();

    mock_repo
        .expect_get_from_token()
        .times(1)
        .with(predicate::eq("token".to_string()))
        .returning(|_| Ok(user_td::get_user_entity_test_data()));

    let usecase: Arc<dyn UserUsecase> = Arc::new(UserInteractor::new(Arc::new(mock_repo)));

    let result = usecase.get_by_token("token".to_string()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), user_td::get_user_entity_test_data());
}
