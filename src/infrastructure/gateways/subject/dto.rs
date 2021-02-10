use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectClassPlanDoc {
    pub topic: String,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectInstructorDoc {
    pub name: String,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectGoalEvaluationDoc {
    pub label: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectGoalDoc {
    pub description: String,
    pub evaluations: Vec<SubjectGoalEvaluationDoc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectFixedScheduleDoc {
    pub date: i32,
    pub hour: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectScheduleDoc {
    #[serde(rename = "type")]
    pub schedule_type: String,
    pub days: Option<Vec<SubjectFixedScheduleDoc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectCategoryDoc {
    pub faculty: Option<String>,
    pub field: Option<String>,
    pub program: Option<String>,
    pub category: Option<String>,
    pub semester: String,
    pub available: bool,
    pub year: Vec<i32>,
    pub schedule: SubjectScheduleDoc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectAttachmentDoc {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectDocument {
    pub title: String,
    pub categories: Vec<SubjectCategoryDoc>,
    pub instructors: Vec<SubjectInstructorDoc>,
    pub attachments: Option<Vec<SubjectAttachmentDoc>>,
    pub flags: Vec<String>,
    pub outline: Option<String>,
    pub purpose: Option<String>,
    pub plans: Vec<SubjectClassPlanDoc>,
    pub requirement: Option<String>,
    pub point: Option<String>,
    pub textbook: Option<String>,
    #[serde(rename = "gradingPolicy")]
    pub grading_policy: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "researchPlan")]
    pub research_plan: Option<String>,

    #[serde(rename = "timetableId")]
    pub timetable_id: Option<i32>,
    #[serde(rename = "courseId")]
    pub course_id: Option<i32>,
    pub credits: Option<i32>,
    #[serde(rename = "type")]
    pub subject_type: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "className")]
    pub class_name: Option<String>,
    pub goal: Option<SubjectGoalDoc>,
}
