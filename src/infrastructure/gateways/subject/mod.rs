use crate::{
    application::repositories::{subject, SubjectRepository},
    utils::elasticsearch::SearchResponse,
};
use elasticsearch::{Elasticsearch, GetParts, SearchParts};

mod dto;

use crate::domain::entities::SubjectEntity;
use crate::utils::elasticsearch::GetResponse;
use dto::SubjectDocument;
use serde_json::{json, Value};
use std::{convert::TryFrom, sync::Arc};

mod extend;
#[cfg(test)]
mod extend_test;

pub struct SubjectGateway<'a>(Arc<Elasticsearch>, &'a str);

impl<'a> SubjectGateway<'a> {
    pub fn new(client: Arc<Elasticsearch>, index: &'a str) -> SubjectGateway<'a> {
        SubjectGateway(client, index)
    }
}

macro_rules! push_terms {
    ($arr: expr, $key: expr, $value: expr) => {
        $arr.push(json!({
            "terms": {
                $key: [$value]
            }
        }))
    };
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

    async fn search<'b>(
        &self,
        input: subject::SubjectSearchInput<'b>,
    ) -> Result<subject::SubjectSearchOutput, anyhow::Error> {
        let mut must = Vec::<Value>::new();

        if input.available_only {
            must.push(json!({
                "term": {
                    "categories.available": {
                        "value": true,
                    }
                }
            }));
        }

        match &input.schedule {
            subject::SubjectSearchScheduleOption::None => {}
            subject::SubjectSearchScheduleOption::Intensive => {
                push_terms!(must, "categories.schedule.type", "intensive");
            }
            subject::SubjectSearchScheduleOption::Fixed { date, hour } => {
                push_terms!(must, "categories.schedule.type", "fixed");
                if let Some(date) = date {
                    push_terms!(must, "categories.schedule.days.date", date);
                }
                if let Some(hour) = hour {
                    push_terms!(must, "categories.schedule.days.hour", hour);
                }
            }
        }

        if let Some(title) = input.title {
            must.push(json!({
                "simple_query_string": {
                    "default_operator": "and",
                    "query": title,
                    "fields": ["title"]
                }
            }));
        }

        if let Some(semester) = input.semester {
            push_terms!(must, "categories.semester", semester);
        }

        if let Some(category) = input.category {
            push_terms!(must, "categories.category", category);
        }

        if let Some(faculty) = input.faculty {
            push_terms!(must, "categories.faculty", faculty);
        }

        if let Some(program) = input.program {
            push_terms!(must, "categories.program", program);
        }

        if let Some(field) = input.field {
            push_terms!(must, "categories.field", field);
        }

        let res = self
            .0
            .search(SearchParts::Index(&[self.1]))
            .body(json!({
                "query": {
                    "bool": {
                        "must": must,
                    }
                },
                "from": input.from,
                "size": input.count,
            }))
            .send()
            .await?;
        let result = res.json::<SearchResponse<dto::SubjectDocument>>().await?;

        Ok(subject::SubjectSearchOutput {
            total_count: result.hits.total.value,
            items: result
                .hits
                .hits
                .into_iter()
                .map(|item| SubjectEntity::try_from(item))
                .collect::<Result<Vec<SubjectEntity>, anyhow::Error>>()?,
        })
    }
}
