use crate::application::repositories::SubjectRepository;
use elasticsearch::{Elasticsearch, GetParts};

mod dto;

use crate::domain::entities::SubjectEntity;
use crate::utils::elasticsearch::GetResponse;
use dto::SubjectDocument;
use futures::prelude::*;
use std::convert::TryFrom;
use std::pin::Pin;

mod extend;
mod extend_test;

pub struct SubjectGateway<'a>(&'a Elasticsearch, &'a str);

impl<'a> SubjectGateway<'a> {
    pub fn new(client: &'a Elasticsearch, index: &'a str) -> SubjectGateway<'a> {
        SubjectGateway(client, index)
    }
}

impl<'a> SubjectRepository for SubjectGateway<'a> {
    fn get_by_id<'b>(
        &'b self,
        id: u32,
    ) -> Pin<Box<dyn Future<Output = Result<SubjectEntity, anyhow::Error>> + 'b>> {
        async move {
            let res = self
                .0
                .get(GetParts::IndexId(self.1, id.to_string().as_str()))
                .send()
                .await?;
            let doc = res.json::<GetResponse<SubjectDocument>>().await?;

            SubjectEntity::try_from(doc)
        }
        .boxed_local()
    }
}
