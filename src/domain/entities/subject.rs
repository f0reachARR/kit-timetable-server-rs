use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct SubjectClassPlan {
    pub topic: String,
    pub content: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct SubjectInstructor {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct SubjectGoalEvaluation {
    pub label: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct SubjectGoal {
    pub description: String,
    pub evaluations: Vec<SubjectGoalEvaluation>,
}

#[derive(Debug, PartialEq)]
pub struct SubjectFixedSchedule {
    pub date: i32,
    pub hour: i32,
}

#[derive(Debug, PartialEq)]
pub enum SubjectSchedule {
    Intensive,
    Fixed(Vec<SubjectFixedSchedule>),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct SubjectCategory {
    pub faculty: Option<String>,
    pub field: Option<String>,
    pub program: Option<String>,
    pub category: Option<String>,
    pub semester: String,
    pub available: bool,
    pub year: Vec<i32>,
    pub schedule: SubjectSchedule,
}

#[derive(Debug, PartialEq)]
pub enum SubjectFlag {
    Internship,
    IGP,
    AL,
    PBL,
    PT,
    Univ3,
    Kyoto,
    Lottery,
}

#[derive(Debug, PartialEq)]
pub struct SubjectEntity {
    pub id: i32,
    pub title: String,
    pub categories: Vec<SubjectCategory>,
    pub instructors: Vec<SubjectInstructor>,
    pub attachments: HashMap<String, String>,
    pub flags: Vec<SubjectFlag>,
    pub outline: String,
    pub purpose: String,
    pub plans: Vec<SubjectClassPlan>,
    pub requirement: String,
    pub point: String,
    pub textbook: String,
    pub grading_policy: String,
    pub remark: String,
    pub research_plan: String,

    pub timetable_id: Option<i32>,
    pub course_id: Option<i32>,
    pub credits: Option<i32>,
    pub subject_type: Option<String>,
    pub code: Option<String>,
    pub class_name: Option<String>,
    pub goal: Option<SubjectGoal>,
}

#[derive(Debug, PartialEq)]
pub struct SubjectSearchTermsEntity {
    pub categories: HashMap<
        String, /* faculty */
        HashMap<String /* field */, HashMap<String /* program */, Vec<String /* category */>>>,
    >,
    pub semesters: Vec<String>,
    pub years: Vec<i32>,
}
