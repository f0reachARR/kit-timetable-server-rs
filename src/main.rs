#![feature(proc_macro_hygiene, decl_macro)]

mod application;
mod domain;
mod infrastructure;
mod utils;

extern crate elasticsearch;
extern crate futures;
extern crate serde_json;
extern crate warp;

use std::sync::Arc;

use application::{
    interactors::SubjectInteractor, repositories::SubjectRepository, usecases::SubjectUsecase,
};
use elasticsearch::http::transport::Transport;
use elasticsearch::Elasticsearch;
use infrastructure::{
    frameworks::{graphql::start_graphql, UsecaseContainer},
    gateways::SubjectGateway,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let transport = Transport::single_node("http://10.0.0.2:9200")?;
    let es = Arc::new(Elasticsearch::new(transport));
    let subject_repository: Arc<dyn SubjectRepository> = Arc::new(SubjectGateway::new(
        Arc::clone(&es),
        "kittimetable_subjects",
    ));
    let subject_usecase: Arc<dyn SubjectUsecase> =
        Arc::new(SubjectInteractor::new(Arc::clone(&subject_repository)));

    let context = UsecaseContainer {
        subject_usecase: Arc::clone(&subject_usecase),
    };

    start_graphql(context).await;

    Ok(())
}
