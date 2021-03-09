use std::sync::Arc;

use crate::{
    application::{
        repositories::{
            SubjectRepository, SubjectSearchInput, SubjectSearchScheduleDate,
            SubjectSearchScheduleHour, SubjectSearchScheduleOption,
        },
        usecases,
        usecases::{SubjectSearchParameter, SubjectSearchResult, SubjectUsecase},
    },
    domain::entities::SubjectEntity,
    utils::phantom::Phantom,
};

pub struct SubjectInteractor {
    subject_repository: Arc<dyn SubjectRepository>,
}

impl SubjectInteractor {
    pub fn new(subject_repository: Arc<dyn SubjectRepository>) -> SubjectInteractor {
        SubjectInteractor { subject_repository }
    }
}

#[async_trait::async_trait]
impl SubjectUsecase for SubjectInteractor {
    async fn get_by_id<'b>(&'b self, id: u32) -> Result<SubjectEntity, anyhow::Error> {
        self.subject_repository.get_by_id(id).await
    }

    async fn search(
        &self,
        param: &SubjectSearchParameter<'_>,
    ) -> Result<SubjectSearchResult, anyhow::Error> {
        let input = SubjectSearchInput {
            from: param.from,
            count: param.count,
            title: param.title,
            available_only: param.available_only,
            schedule: match &param.schedule {
                usecases::SubjectSearchScheduleOption::None => SubjectSearchScheduleOption::None,
                usecases::SubjectSearchScheduleOption::FixedWithoutCond => {
                    SubjectSearchScheduleOption::FixedWithoutCond
                }
                usecases::SubjectSearchScheduleOption::Fixed { date, hour } => {
                    SubjectSearchScheduleOption::Fixed(
                        Phantom::<SubjectSearchScheduleDate>::from(date.clone()),
                        Phantom::<SubjectSearchScheduleHour>::from(hour.clone()),
                    )
                }
                usecases::SubjectSearchScheduleOption::Intensive => {
                    SubjectSearchScheduleOption::Intensive
                }
            },
            semester: param.semester,
            year: param.year,
            category: param.category,
            faculty: param.faculty,
            program: param.program,
            field: param.field,
        };
        let result = self.subject_repository.search(input).await?;

        Ok(SubjectSearchResult {
            count: result.total_count,
            subjects: result.items,
        })
    }
}
