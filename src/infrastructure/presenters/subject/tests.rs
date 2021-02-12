use crate::domain::entities::{
    subject::{
        SubjectCategory, SubjectClassPlan, SubjectFlag, SubjectGoal, SubjectGoalEvaluation,
        SubjectInstructor, SubjectSchedule,
    },
    SubjectEntity,
};

use super::{dto::*, from_entity};

#[test]
pub fn test_dto() {
    let entity = SubjectEntity {
        id: 1,
        title: String::from("title"),
        categories: vec![SubjectCategory {
            faculty: Some(String::from("faculty")),
            field: Some(String::from("field")),
            program: Some(String::from("program")),
            category: Some(String::from("category")),
            semester: String::from("semester"),
            available: true,
            year: vec![1000],
            schedule: SubjectSchedule::Intensive,
        }],
        instructors: vec![
            SubjectInstructor {
                name: String::from("ins1"),
                id: Some(String::from("id1")),
            },
            SubjectInstructor {
                name: String::from("ins2"),
                id: None,
            },
        ],
        attachments: [(String::from("a1"), String::from("b1"))]
            .iter()
            .cloned()
            .collect(),
        flags: vec![SubjectFlag::Internship],
        outline: String::from("outline"),
        purpose: String::from("purpose"),
        plans: vec![
            SubjectClassPlan {
                topic: String::from("topic"),
                content: Some(String::from("content")),
            },
            SubjectClassPlan {
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
        goal: Some(SubjectGoal {
            description: String::from("desc"),
            evaluations: vec![SubjectGoalEvaluation {
                label: String::from("label"),
                description: String::from("desc2"),
            }],
        }),
    };
    let actual = from_entity(entity);
    let expected = GqlSubjectDto {
        id: 1,
        title: String::from("title"),
        categories: vec![GqlSubjectCategory {
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
        }],
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
