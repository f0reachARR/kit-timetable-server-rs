use super::SubjectInteractor;
use crate::{
    application::{
        repositories::{subject_mock, SubjectRepository},
        usecases::{SubjectSearchParameter, SubjectSearchScheduleOption, SubjectUsecase},
    },
    domain::entities::subject_td,
};
use std::sync::Arc;

#[tokio::test]
async fn test_get_by_id() {
    let mock_repo: Arc<dyn SubjectRepository> = Arc::new(subject_mock::SubjectRepositoryMock);
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(mock_repo));

    let result = usecase.get_by_id(1).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), subject_td::get_subject_entity_test_data());

    let result = usecase.get_by_id(1).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_search() {
    let mock_repo: Arc<dyn SubjectRepository> = Arc::new(subject_mock::SubjectRepositoryMock);
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(mock_repo));

    let result = usecase
        .search(&SubjectSearchParameter {
            from: 0,
            count: 5,
            title: None,
            available_only: true,
            schedule: SubjectSearchScheduleOption::None,
            semester: None,
            year: None,
            category: None,
            faculty: None,
            program: None,
            field: None,
        })
        .await;
    assert!(result.is_ok());
    let unwrapped_result = result.unwrap();
    assert_eq!(unwrapped_result.count, 4);
    assert_eq!(unwrapped_result.subjects.len(), 4);
}
