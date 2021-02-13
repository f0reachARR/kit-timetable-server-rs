use crate::{domain::entities::SubjectEntity, utils::phantom::Phantom};

#[derive(Debug)]
struct SubjectSearchScheduleDate;
#[derive(Debug)]
struct SubjectSearchScheduleHour;

#[derive(Debug)]
pub enum SubjectSearchScheduleOption {
    None,
    FixedWithoutCond,
    Fixed(
        Phantom<SubjectSearchScheduleDate>,
        Phantom<SubjectSearchScheduleHour>,
    ),
    Intensive,
}

#[derive(Debug)]
pub struct SubjectSearchInput {
    pub from: u32,
    pub count: u32,
    pub title: Option<String>,
    pub available_only: bool,
    pub schedule: SubjectSearchScheduleOption,
    pub semester: Option<String>,
    pub year: Option<u32>,
    pub category: Option<String>,
    pub faculty: Option<String>,
    pub program: Option<String>,
    pub field: Option<String>,
}

#[derive(Debug)]
pub struct SubjectSearchOutput {
    pub total_count: u32,
    pub items: Vec<SubjectEntity>,
}

#[async_trait::async_trait]
pub trait SubjectRepository: Sync + Send {
    async fn get_by_id(&self, id: u32) -> Result<SubjectEntity, anyhow::Error>;
    async fn search(&self, input: SubjectSearchInput)
        -> Result<SubjectSearchOutput, anyhow::Error>;
}
