mod dto;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::{
    application::usecases,
    domain::entities::{
        subject::{
            SubjectCategory, SubjectClassPlan, SubjectFlag, SubjectGoal, SubjectGoalEvaluation,
            SubjectInstructor, SubjectSchedule,
        },
        SubjectEntity,
    },
};
pub use dto::{GqlSubjectDto, GqlSubjectSearchResult};

fn convert_attachments(value: HashMap<String, String>) -> Vec<dto::GqlSubjectAttachment> {
    value
        .into_iter()
        .map(|item| dto::GqlSubjectAttachment {
            key: item.0,
            name: item.1,
        })
        .collect()
}

fn convert_flag(flag: SubjectFlag) -> dto::GqlSubjectFlag {
    match flag {
        SubjectFlag::Internship => dto::GqlSubjectFlag::Internship,
        SubjectFlag::IGP => dto::GqlSubjectFlag::IGP,
        SubjectFlag::AL => dto::GqlSubjectFlag::AL,
        SubjectFlag::PBL => dto::GqlSubjectFlag::PBL,
        SubjectFlag::PT => dto::GqlSubjectFlag::PT,
        SubjectFlag::Univ3 => dto::GqlSubjectFlag::Univ3,
        SubjectFlag::Kyoto => dto::GqlSubjectFlag::Kyoto,
        SubjectFlag::Lottery => dto::GqlSubjectFlag::Lottery,
    }
}

fn convert_schedule(value: SubjectSchedule) -> dto::GqlSubjectSchedule {
    match value {
        SubjectSchedule::Intensive => dto::GqlSubjectSchedule {
            schedule_type: dto::GqlSubjectScheduleType::Intensive,
            days: Default::default(),
        },
        SubjectSchedule::Unknown => dto::GqlSubjectSchedule {
            schedule_type: dto::GqlSubjectScheduleType::Unknown,
            days: Default::default(),
        },
        SubjectSchedule::Fixed(days) => dto::GqlSubjectSchedule {
            schedule_type: dto::GqlSubjectScheduleType::Fixed,
            days: days
                .into_iter()
                .map(|d| dto::GqlSubjectFixedSchedule {
                    date: d.date,
                    hour: d.hour,
                })
                .collect(),
        },
    }
}

fn convert_category(value: SubjectCategory) -> dto::GqlSubjectCategory {
    dto::GqlSubjectCategory {
        faculty: value.faculty,
        field: value.field,
        program: value.program,
        category: value.category,
        semester: value.semester,
        available: value.available,
        year: value.year,
        schedule: convert_schedule(value.schedule),
    }
}

fn convert_instructor(value: SubjectInstructor) -> dto::GqlSubjectInstructor {
    dto::GqlSubjectInstructor {
        name: value.name,
        id: value.id,
    }
}

fn convert_plan(value: SubjectClassPlan) -> dto::GqlSubjectClassPlan {
    dto::GqlSubjectClassPlan {
        topic: value.topic,
        content: value.content,
    }
}

fn convert_evaluation(value: SubjectGoalEvaluation) -> dto::GqlSubjectGoalEvaluation {
    dto::GqlSubjectGoalEvaluation {
        label: value.label,
        description: value.description,
    }
}

fn convert_goal(value: SubjectGoal) -> dto::GqlSubjectGoal {
    dto::GqlSubjectGoal {
        description: value.description,
        evaluations: value
            .evaluations
            .into_iter()
            .map(convert_evaluation)
            .collect(),
    }
}

pub fn from_entity(value: SubjectEntity) -> dto::GqlSubjectDto {
    dto::GqlSubjectDto {
        id: value.id,
        title: value.title,
        categories: value.categories.into_iter().map(convert_category).collect(),
        instructors: value
            .instructors
            .into_iter()
            .map(convert_instructor)
            .collect(),
        attachments: convert_attachments(value.attachments),
        flags: value.flags.into_iter().map(convert_flag).collect(),
        outline: value.outline,
        purpose: value.purpose,
        plans: value.plans.into_iter().map(convert_plan).collect(),
        requirement: value.requirement,
        point: value.point,
        textbook: value.textbook,
        grading_policy: value.grading_policy,
        remark: value.remark,
        research_plan: value.research_plan,
        timetable_id: value.timetable_id,
        course_id: value.course_id,
        credits: value.credits,
        subject_type: value.subject_type,
        code: value.code,
        class_name: value.class_name,
        goal: value.goal.map(convert_goal),
    }
}

pub fn convert_search_result(input: usecases::SubjectSearchResult) -> GqlSubjectSearchResult {
    GqlSubjectSearchResult {
        count: input.count,
        items: input.subjects.into_iter().map(from_entity).collect(),
    }
}
