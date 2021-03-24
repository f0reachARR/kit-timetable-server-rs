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
    interactors::{SubjectInteractor, UserInteractor},
    repositories::{SubjectRepository, UserRepository},
    usecases::{SubjectUsecase, UserUsecase},
};
use elasticsearch::http::transport::Transport;
use elasticsearch::Elasticsearch;
use envconfig::Envconfig;
use infrastructure::{
    frameworks::{config::AppConfig, graphql::start_graphql, UsecaseContainer},
    gateways::{SubjectGateway, UserGateway},
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();

    let config: Arc<AppConfig> = Arc::new(AppConfig::init_from_env()?);

    let transport = Transport::single_node(&config.elasticsearch_url)?;
    let es = Arc::new(Elasticsearch::new(transport));

    let subject_repository: Arc<dyn SubjectRepository> = Arc::new(SubjectGateway::new(
        es.clone(),
        config.elasticsearch_subject_index.clone(),
    ));
    let user_repository: Arc<dyn UserRepository> = Arc::new(UserGateway::new(
        config.jwt_secret.clone(),
        config.jwt_issuer.clone(),
    ));

    let subject_usecase: Arc<dyn SubjectUsecase> =
        Arc::new(SubjectInteractor::new(subject_repository.clone()));
    let user_usecase: Arc<dyn UserUsecase> = Arc::new(UserInteractor::new(user_repository.clone()));

    let context = Arc::new(UsecaseContainer {
        subject_usecase: subject_usecase.clone(),
        user_usecase: user_usecase.clone(),
    });

    start_graphql(context).await;

    Ok(())
}
