use crate::application::repositories::SubjectRepository;
use elasticsearch::{Elasticsearch, GetParts};

mod dto;

use crate::domain::entities::SubjectEntity;
use crate::utils::elasticsearch::GetResponse;
use dto::SubjectDocument;
use std::{convert::TryFrom, sync::Arc};

mod extend;
mod extend_test;

pub struct SubjectGateway<'a>(Arc<Elasticsearch>, &'a str);

impl<'a> SubjectGateway<'a> {
    pub fn new(client: Arc<Elasticsearch>, index: &'a str) -> SubjectGateway<'a> {
        SubjectGateway(client, index)
    }
}

#[async_trait::async_trait]
impl<'a> SubjectRepository for SubjectGateway<'a> {
    async fn get_by_id(&self, id: u32) -> Result<SubjectEntity, anyhow::Error> {
        let res = self
            .0
            .get(GetParts::IndexId(self.1, id.to_string().as_str()))
            .send()
            .await?;
        let doc = res.json::<GetResponse<SubjectDocument>>().await?;

        SubjectEntity::try_from(doc)
    }
}
