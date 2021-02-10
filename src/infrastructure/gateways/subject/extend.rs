use super::dto;
use crate::domain::entities::subject::{
    SubjectCategory, SubjectClassPlan, SubjectFixedSchedule, SubjectFlag, SubjectGoal,
    SubjectGoalEvaluation, SubjectInstructor, SubjectSchedule,
};
use crate::domain::entities::SubjectEntity;
use crate::utils::elasticsearch::GetResponse;
use anyhow::anyhow;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;

impl FromStr for SubjectFlag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "internship" => Ok(Self::Internship),
            "igp" => Ok(Self::IGP),
            "al" => Ok(Self::AL),
            "pbl" => Ok(Self::PBL),
            "pt" => Ok(Self::PT),
            "univ3" => Ok(Self::Univ3),
            "kyoto" => Ok(Self::Kyoto),
            "lottery" => Ok(Self::Lottery),
            _ => Err(anyhow!("SubjectFlag does not match")),
        }
    }
}

impl TryFrom<dto::SubjectScheduleDoc> for SubjectSchedule {
    type Error = anyhow::Error;

    fn try_from(value: dto::SubjectScheduleDoc) -> Result<Self, Self::Error> {
        match value.schedule_type.as_str() {
            "intensive" => Ok(Self::Intensive),
            "unknown" => Ok(Self::Unknown),
            "fixed" => {
                if let Some(arr) = value.days {
                    let collected = arr
                        .into_iter()
                        .map(|item| -> Result<SubjectFixedSchedule, Self::Error> {
                            Ok(SubjectFixedSchedule {
                                date: item.date,
                                hour: item.hour,
                            })
                        })
                        .collect::<Result<Vec<_>, Self::Error>>()?;
                    Ok(Self::Fixed(collected))
                } else {
                    Err(anyhow!("Fixed schedule is not valid"))
                }
            }
            _ => Err(anyhow!("Schedule is not valid")),
        }
    }
}

impl TryFrom<dto::SubjectCategoryDoc> for SubjectCategory {
    type Error = anyhow::Error;

    fn try_from(value: dto::SubjectCategoryDoc) -> Result<Self, Self::Error> {
        Ok(SubjectCategory {
            faculty: value.faculty,
            field: value.field,
            program: value.program,
            category: value.category,
            semester: value.semester,
            available: value.available,
            year: value.year,
            schedule: SubjectSchedule::try_from(value.schedule)?,
        })
    }
}

impl From<dto::SubjectInstructorDoc> for SubjectInstructor {
    fn from(value: dto::SubjectInstructorDoc) -> Self {
        SubjectInstructor {
            name: value.name,
            id: value.id,
        }
    }
}

impl From<dto::SubjectClassPlanDoc> for SubjectClassPlan {
    fn from(value: dto::SubjectClassPlanDoc) -> Self {
        SubjectClassPlan {
            topic: value.topic,
            content: value.content,
        }
    }
}

impl From<dto::SubjectGoalEvaluationDoc> for SubjectGoalEvaluation {
    fn from(value: dto::SubjectGoalEvaluationDoc) -> Self {
        SubjectGoalEvaluation {
            label: value.label,
            description: value.description,
        }
    }
}

impl From<dto::SubjectGoalDoc> for SubjectGoal {
    fn from(value: dto::SubjectGoalDoc) -> SubjectGoal {
        SubjectGoal {
            description: value.description,
            evaluations: value
                .evaluations
                .into_iter()
                .map(|n| SubjectGoalEvaluation::from(n))
                .collect(),
        }
    }
}

impl TryFrom<GetResponse<dto::SubjectDocument>> for SubjectEntity {
    type Error = anyhow::Error;

    fn try_from(res: GetResponse<dto::SubjectDocument>) -> Result<Self, Self::Error> {
        Ok(SubjectEntity {
            id: res._id.parse()?,
            title: res._source.title,
            categories: res
                ._source
                .categories
                .into_iter()
                .map(|item| SubjectCategory::try_from(item))
                .collect::<Result<Vec<_>, _>>()?,
            instructors: res
                ._source
                .instructors
                .into_iter()
                .map(|item| SubjectInstructor::from(item))
                .collect(),
            attachments: res._source.attachments.map_or(Default::default(), |n| {
                HashMap::from_iter(n.into_iter().map(|v| (v.key, v.name)))
            }),
            flags: res
                ._source
                .flags
                .into_iter()
                .map(|v| SubjectFlag::from_str(v.as_str()))
                .collect::<Result<Vec<_>, _>>()?,
            outline: res._source.outline,
            purpose: res._source.purpose,
            plans: res
                ._source
                .plans
                .into_iter()
                .map(|item| SubjectClassPlan::from(item))
                .collect(),
            requirement: res._source.requirement,
            point: res._source.point,
            textbook: res._source.textbook,
            grading_policy: res._source.grading_policy,
            remark: res._source.remark,
            research_plan: res._source.research_plan,
            timetable_id: res._source.timetable_id,
            course_id: res._source.course_id,
            credits: res._source.credits,
            subject_type: res._source.subject_type,
            code: res._source.code,
            class_name: res._source.class_name,
            goal: res._source.goal.map(|n| SubjectGoal::from(n)),
        })
    }
}
