use super::subject::*;

pub fn get_subject_entity_test_data() -> SubjectEntity {
    SubjectEntity {
        id: 1,
        title: String::from("title"),
        categories: vec![
            SubjectCategory {
                faculty: Some(String::from("faculty")),
                field: Some(String::from("field")),
                program: Some(String::from("program")),
                category: Some(String::from("category")),
                semester: String::from("semester"),
                available: true,
                year: vec![1000],
                schedule: SubjectSchedule::Fixed(vec![SubjectFixedSchedule { date: 1, hour: 1 }]),
            },
            SubjectCategory {
                faculty: Some(String::from("faculty")),
                field: Some(String::from("field")),
                program: Some(String::from("program")),
                category: Some(String::from("category")),
                semester: String::from("semester"),
                available: true,
                year: vec![1000],
                schedule: SubjectSchedule::Intensive,
            },
        ],
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
    }
}

pub fn get_subject_search_terms_test_data() -> SubjectSearchTermsEntity {
    SubjectSearchTermsEntity {
        categories: [(
            "f1".to_string(),
            [(
                "f2".to_string(),
                [("p".to_string(), vec!["c1".to_string(), "c2".to_string()])]
                    .iter()
                    .cloned()
                    .collect(),
            )]
            .iter()
            .cloned()
            .collect(),
        )]
        .iter()
        .cloned()
        .collect(),
        semesters: vec!["s1".to_string(), "s2".to_string()],
        years: vec![1, 2, 3],
    }
}
