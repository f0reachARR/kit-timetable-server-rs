use super::SubjectInteractor;
use crate::{
    application::{
        repositories,
        repositories::subject::MockSubjectRepository,
        usecases::{SubjectSearchParameter, SubjectSearchScheduleOption, SubjectUsecase},
    },
    domain::entities::subject_td,
};
use mockall::predicate;
use std::sync::Arc;

#[tokio::test]
async fn test_get_by_id_ok() {
    let mut mock_repo = MockSubjectRepository::new();
    mock_repo
        .expect_get_by_id()
        .with(predicate::eq(1))
        .times(1)
        .returning(|_| Ok(subject_td::get_subject_entity_test_data()));
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(Arc::new(mock_repo)));

    let result = usecase.get_by_id(1).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), subject_td::get_subject_entity_test_data());
}

#[tokio::test]
async fn test_get_by_id_err() {
    let mut mock_repo = MockSubjectRepository::new();
    mock_repo
        .expect_get_by_id()
        .with(predicate::eq(2))
        .times(1)
        .returning(|_| Err(anyhow::anyhow!("Error")));
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(Arc::new(mock_repo)));

    let result = usecase.get_by_id(2).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_search_1() {
    let mut mock_repo = MockSubjectRepository::new();
    mock_repo
        .expect_search()
        .withf(|input: &repositories::SubjectSearchInput<'_>| {
            input.from == 0
                && input.count == 5
                && input.available_only == true
                && input.schedule == repositories::SubjectSearchScheduleOption::None
                && input.title.is_none()
                && input.semester.is_none()
                && input.faculty.is_none()
                && input.field.is_none()
                && input.category.is_none()
                && input.program.is_none()
                && input.year.is_none()
        })
        .times(1)
        .returning(|_| {
            Ok(repositories::SubjectSearchOutput {
                total_count: 4,
                items: vec![
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                ],
            })
        });
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(Arc::new(mock_repo)));

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

#[tokio::test]
async fn test_search_2() {
    let mut mock_repo = MockSubjectRepository::new();
    mock_repo
        .expect_search()
        .withf(|input: &repositories::SubjectSearchInput<'_>| {
            input.from == 0
                && input.count == 5
                && input.available_only == true
                && input.schedule
                    == repositories::SubjectSearchScheduleOption::Fixed {
                        date: Some(2),
                        hour: Some(3),
                    }
                && input.title.unwrap() == "title"
                && input.semester.unwrap() == "semester"
                && input.faculty.unwrap() == "faculty"
                && input.field.unwrap() == "field"
                && input.category.unwrap() == "category"
                && input.program.unwrap() == "program"
                && input.year.unwrap() == 6
        })
        .times(1)
        .returning(|_| {
            Ok(repositories::SubjectSearchOutput {
                total_count: 4,
                items: vec![
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                ],
            })
        });
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(Arc::new(mock_repo)));

    assert!(usecase
        .search(&SubjectSearchParameter {
            from: 0,
            count: 5,
            title: Some("title"),
            available_only: true,
            schedule: SubjectSearchScheduleOption::Fixed {
                date: Some(2),
                hour: Some(3),
            },
            semester: Some("semester"),
            year: Some(6),
            category: Some("category"),
            faculty: Some("faculty"),
            program: Some("program"),
            field: Some("field"),
        })
        .await
        .is_ok());
}

#[tokio::test]
async fn test_search_count_over() {
    let mock_repo = MockSubjectRepository::new();
    let usecase: Arc<dyn SubjectUsecase> = Arc::new(SubjectInteractor::new(Arc::new(mock_repo)));

    let result = usecase
        .search(&SubjectSearchParameter {
            from: 0,
            count: 500,
            title: Some("title"),
            available_only: true,
            schedule: SubjectSearchScheduleOption::Fixed {
                date: Some(2),
                hour: Some(3),
            },
            semester: Some("semester"),
            year: Some(6),
            category: Some("category"),
            faculty: Some("faculty"),
            program: Some("program"),
            field: Some("field"),
        })
        .await;
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Too many items in a request"
    );
}
