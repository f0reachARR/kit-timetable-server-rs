use async_graphql::{Enum, InputObject, MaybeUndefined};

use crate::application::usecases;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
pub enum GqlSubjectSearchSchedule {
    Fixed,
    Intensive,
}

#[derive(Debug, PartialEq, InputObject)]
pub struct GqlSubjectSearchInput {
    pub title: Option<String>,
    pub available_only: Option<bool>,
    pub schedule: Option<GqlSubjectSearchSchedule>,
    pub schedule_date: MaybeUndefined<u32>,
    pub schedule_hour: MaybeUndefined<u32>,
    pub semester: Option<String>,
    pub year: Option<u32>,
    pub category: Option<String>,
    pub faculty: Option<String>,
    pub program: Option<String>,
    pub field: Option<String>,
}

impl GqlSubjectSearchInput {
    pub fn into_usecase_param<'a>(
        &'a self,
        from: u32,
        count: u32,
    ) -> Option<usecases::SubjectSearchParameter<'a>> {
        let schedule = match self.schedule {
            None => usecases::SubjectSearchScheduleOption::None,
            Some(kind) => match kind {
                GqlSubjectSearchSchedule::Intensive => {
                    usecases::SubjectSearchScheduleOption::Intensive
                }
                GqlSubjectSearchSchedule::Fixed => usecases::SubjectSearchScheduleOption::Fixed {
                    date: match self.schedule_date {
                        MaybeUndefined::Null => None,
                        MaybeUndefined::Undefined => return None, // treat as error
                        MaybeUndefined::Value(v) => Some(v),
                    },
                    hour: match self.schedule_hour {
                        MaybeUndefined::Null => None,
                        MaybeUndefined::Undefined => return None, // treat as error
                        MaybeUndefined::Value(v) => Some(v),
                    },
                },
            },
        };

        Some(usecases::SubjectSearchParameter {
            from,
            count,
            title: self.title.as_ref().map(|s| s.as_str()),
            available_only: self.available_only.unwrap_or(false),
            schedule,
            semester: self.semester.as_ref().map(|s| s.as_str()),
            year: self.year,
            category: self.category.as_ref().map(|s| s.as_str()),
            faculty: self.faculty.as_ref().map(|s| s.as_str()),
            program: self.program.as_ref().map(|s| s.as_str()),
            field: self.field.as_ref().map(|s| s.as_str()),
        })
    }
}
