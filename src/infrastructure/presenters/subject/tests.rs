use crate::application::usecases;
use crate::domain::entities::subject_td;

use super::{convert_search_result, dto::*, from_entity};

#[test]
pub fn test_dto() {
    let entity = subject_td::get_subject_entity_test_data();
    let actual = from_entity(entity);
    let expected = GqlSubjectDto {
        id: 1,
        title: String::from("title"),
        categories: vec![
            GqlSubjectCategory {
                faculty: Some(String::from("faculty")),
                field: Some(String::from("field")),
                program: Some(String::from("program")),
                category: Some(String::from("category")),
                semester: String::from("semester"),
                available: true,
                year: vec![1000],
                schedule: GqlSubjectSchedule {
                    schedule_type: GqlSubjectScheduleType::Fixed,
                    days: vec![GqlSubjectFixedSchedule { date: 1, hour: 1 }],
                },
            },
            GqlSubjectCategory {
                faculty: Some(String::from("faculty")),
                field: Some(String::from("field")),
                program: Some(String::from("program")),
                category: Some(String::from("category")),
                semester: String::from("semester"),
                available: true,
                year: vec![1000],
                schedule: GqlSubjectSchedule {
                    schedule_type: GqlSubjectScheduleType::Intensive,
                    days: vec![],
                },
            },
        ],
        instructors: vec![
            GqlSubjectInstructor {
                name: String::from("ins1"),
                id: Some(String::from("id1")),
            },
            GqlSubjectInstructor {
                name: String::from("ins2"),
                id: None,
            },
        ],
        attachments: vec![GqlSubjectAttachment {
            key: String::from("a1"),
            name: String::from("b1"),
        }],
        flags: vec![GqlSubjectFlag::Internship],
        outline: String::from("outline"),
        purpose: String::from("purpose"),
        plans: vec![
            GqlSubjectClassPlan {
                topic: String::from("topic"),
                content: Some(String::from("content")),
            },
            GqlSubjectClassPlan {
                topic: String::from("topic"),
                content: None,
            },
        ],
        requirement: String::from("requirement"),
        point: String::from("point"),
        textbook: String::from("textbook"),
        grading_policy: String::from("grading_policy"),
        remark: String::from("remark"),
        research_plan: String::from("research_plan"),
        timetable_id: Some(2),
        course_id: Some(3),
        credits: Some(4),
        subject_type: Some(String::from("subject_type")),
        code: Some(String::from("code")),
        class_name: Some(String::from("class_name")),
        goal: Some(GqlSubjectGoal {
            description: String::from("desc"),
            evaluations: vec![GqlSubjectGoalEvaluation {
                label: String::from("label"),
                description: String::from("desc2"),
            }],
        }),
    };

    assert_eq!(actual, expected);
}

#[test]
fn test_search_result() {
    let actual = convert_search_result(usecases::SubjectSearchResult {
        count: 123,
        subjects: vec![],
    });
    let expected = GqlSubjectSearchResult {
        count: 123,
        items: vec![],
    };

    assert_eq!(actual, expected);
}
