use super::dto;
use crate::utils::elasticsearch::GetResponse;
use crate::{
    domain::entities::subject::{
        SubjectCategory, SubjectClassPlan, SubjectFixedSchedule, SubjectFlag, SubjectGoal,
        SubjectGoalEvaluation, SubjectInstructor, SubjectSchedule,
    },
    utils::elasticsearch::SearchHitItem,
};
use crate::{
    domain::entities::{SubjectEntity, SubjectSearchTermsEntity},
    utils::elasticsearch::AggregationResponse,
};
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
        if let Some(_source) = res._source {
            Ok(SubjectEntity {
                id: res._id.parse()?,
                title: _source.title,
                categories: _source
                    .categories
                    .into_iter()
                    .map(|item| SubjectCategory::try_from(item))
                    .collect::<Result<Vec<_>, _>>()?,
                instructors: _source
                    .instructors
                    .into_iter()
                    .map(|item| SubjectInstructor::from(item))
                    .collect(),
                attachments: _source.attachments.map_or(Default::default(), |n| {
                    HashMap::from_iter(n.into_iter().map(|v| (v.key, v.name)))
                }),
                flags: _source
                    .flags
                    .into_iter()
                    .map(|v| SubjectFlag::from_str(v.as_str()))
                    .collect::<Result<Vec<_>, _>>()?,
                outline: _source.outline.unwrap_or(Default::default()),
                purpose: _source.purpose.unwrap_or(Default::default()),
                plans: _source
                    .plans
                    .into_iter()
                    .map(|item| SubjectClassPlan::from(item))
                    .collect(),
                requirement: _source.requirement.unwrap_or(Default::default()),
                point: _source.point.unwrap_or(Default::default()),
                textbook: _source.textbook.unwrap_or(Default::default()),
                grading_policy: _source.grading_policy.unwrap_or(Default::default()),
                remark: _source.remark.unwrap_or(Default::default()),
                research_plan: _source.research_plan.unwrap_or(Default::default()),
                timetable_id: _source.timetable_id,
                course_id: _source.course_id,
                credits: _source.credits,
                subject_type: _source.subject_type,
                code: _source.code,
                class_name: _source.class_name,
                goal: _source.goal.map(|n| SubjectGoal::from(n)),
            })
        } else {
            Err(anyhow!("Source not found"))
        }
    }
}

impl TryFrom<SearchHitItem<dto::SubjectDocument>> for SubjectEntity {
    type Error = anyhow::Error;

    fn try_from(res: SearchHitItem<dto::SubjectDocument>) -> Result<Self, Self::Error> {
        SubjectEntity::try_from(GetResponse::<dto::SubjectDocument> {
            found: true,
            _id: res._id,
            _source: res._source,
        })
    }
}

impl From<AggregationResponse<dto::SubjectSearchTermsAgg>> for SubjectSearchTermsEntity {
    fn from(res: AggregationResponse<dto::SubjectSearchTermsAgg>) -> Self {
        SubjectSearchTermsEntity {
            categories: res
                .aggregations
                .categories
                .buckets
                .into_iter()
                .map(|faculty| {
                    (
                        faculty.key,
                        faculty
                            .sub_aggs
                            .fields
                            .buckets
                            .into_iter()
                            .map(|field| {
                                (
                                    field.key,
                                    field
                                        .sub_aggs
                                        .programs
                                        .buckets
                                        .into_iter()
                                        .map(|program| {
                                            (
                                                program.key,
                                                program
                                                    .sub_aggs
                                                    .categories
                                                    .buckets
                                                    .into_iter()
                                                    .map(|category| category.key)
                                                    .collect(),
                                            )
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            semesters: res
                .aggregations
                .semesters
                .buckets
                .into_iter()
                .map(|b| b.key)
                .collect(),
            years: res
                .aggregations
                .years
                .buckets
                .into_iter()
                .map(|b| b.key)
                .collect(),
        }
    }
}
