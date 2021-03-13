use crate::domain::entities::{SubjectEntity, SubjectSearchTermsEntity};

#[derive(Debug)]
pub enum SubjectSearchScheduleOption {
    None,
    Fixed {
        date: Option<u32>,
        hour: Option<u32>,
    },
    Intensive,
}

#[derive(Debug)]
pub struct SubjectSearchParameter<'a> {
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
pub struct SubjectSearchResult {
    pub count: u32,
    pub subjects: Vec<SubjectEntity>,
}

#[async_trait::async_trait]
pub trait SubjectUsecase: Sync + Send {
    async fn get_by_id<'b>(&'b self, id: u32) -> Result<SubjectEntity, anyhow::Error>;
    async fn search(
        &self,
        param: &SubjectSearchParameter<'_>,
    ) -> Result<SubjectSearchResult, anyhow::Error>;
    async fn get_terms(&self) -> Result<SubjectSearchTermsEntity, anyhow::Error>;
}
