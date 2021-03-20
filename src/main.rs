#![feature(proc_macro_hygiene, decl_macro)]

mod application;
mod domain;
mod infrastructure;
mod utils;

extern crate elasticsearch;
extern crate futures;
extern crate serde_json;
extern crate warp;

use application::{
    interactors::SubjectInteractor, repositories::SubjectRepository, usecases::SubjectUsecase,
};
use elasticsearch::http::transport::Transport;
use elasticsearch::Elasticsearch;
use envconfig::Envconfig;
use infrastructure::{
    frameworks::{config::AppConfig, graphql::start_graphql, UsecaseContainer},
    gateways::SubjectGateway,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let config: AppConfig = AppConfig::init_from_env()?;
    let transport = Transport::single_node(config.elasticsearch_url.as_str())?;
    let es = Arc::new(Elasticsearch::new(transport));
    let subject_repository: Arc<dyn SubjectRepository> = Arc::new(SubjectGateway::new(
        Arc::clone(&es),
        config.elasticsearch_subject_index.as_str(),
    ));
    let subject_usecase: Arc<dyn SubjectUsecase> =
        Arc::new(SubjectInteractor::new(Arc::clone(&subject_repository)));

    let context = UsecaseContainer {
        subject_usecase: Arc::clone(&subject_usecase),
    };

    start_graphql(context).await;

    Ok(())
}
