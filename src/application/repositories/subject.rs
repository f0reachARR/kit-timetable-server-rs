use crate::{domain::entities::SubjectEntity, utils::phantom::Phantom};

#[derive(Debug)]
pub struct SubjectSearchScheduleDate;
#[derive(Debug)]
pub struct SubjectSearchScheduleHour;

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
pub struct SubjectSearchInput<'a> {
    pub from: u32,
    pub count: u32,
    pub title: Option<&'a str>,
    pub available_only: bool,
    pub schedule: SubjectSearchScheduleOption,
    pub semester: Option<&'a str>,
    pub year: Option<u32>,
    pub category: Option<&'a str>,
    pub faculty: Option<&'a str>,
    pub program: Option<&'a str>,
    pub field: Option<&'a str>,
}

#[derive(Debug)]
pub struct SubjectSearchOutput {
    pub total_count: u32,
    pub items: Vec<SubjectEntity>,
}

#[async_trait::async_trait]
pub trait SubjectRepository: Sync + Send {
    async fn get_by_id(&self, id: u32) -> Result<SubjectEntity, anyhow::Error>;
    async fn search(
        &self,
        input: &SubjectSearchInput<'_>,
    ) -> Result<SubjectSearchOutput, anyhow::Error>;
}
