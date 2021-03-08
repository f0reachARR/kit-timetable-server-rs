use crate::domain::entities::subject_td;

use super::SubjectRepository;

pub struct SubjectRepositoryMock;

#[async_trait::async_trait]
impl SubjectRepository for SubjectRepositoryMock {
    async fn get_by_id(
        &self,
        id: u32,
    ) -> Result<crate::domain::entities::SubjectEntity, anyhow::Error> {
        match id {
            1 => Ok(subject_td::get_subject_entity_test_data()),
            _ => Err(anyhow::anyhow!("Some error")),
        }
    }

    async fn search(
        &self,
        input: &super::SubjectSearchInput<'_>,
    ) -> Result<super::SubjectSearchOutput, anyhow::Error> {
        match input.from {
            0 => Ok(super::SubjectSearchOutput {
                total_count: 4,
                items: vec![
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                    subject_td::get_subject_entity_test_data(),
                ],
            }),
            5 => Ok(super::SubjectSearchOutput {
                total_count: 0,
                items: vec![],
            }),
            _ => Err(anyhow::anyhow!("Some error")),
        }
    }
}
