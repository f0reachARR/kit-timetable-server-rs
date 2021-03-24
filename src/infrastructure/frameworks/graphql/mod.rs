use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Request, Response as GqlResponse, Schema, ServerError,
};
use async_graphql_warp::{graphql, Response as GqlWarpResponse};
use std::{ops::Deref, sync::Arc};
use warp::{http::Response, Filter};

mod resolvers;
use resolvers::*;

use crate::infrastructure::controllers;

use super::container::UsecaseContainer;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub async fn start_graphql(usecases: Arc<UsecaseContainer>) {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(usecases.clone())
    .finish();

    let gql_post = warp::path::end()
        .and(warp::post())
        .and(warp::header::optional::<String>("authorization"))
        .and(graphql(schema))
        .and_then(
            move |auth_header: Option<String>, (schema, request): (SchemaType, Request)| {
                let usecases = usecases.clone();
                async move {
                    let request_container_result = controllers::request::create_request_container(
                        usecases.deref(),
                        &auth_header,
                    )
                    .await;
                    let response = match request_container_result {
                        Ok(request_container) => {
                            schema.execute(request.data(request_container)).await
                        }
                        Err(err) => GqlResponse::from_errors(vec![ServerError::new(format!(
                            "Authorization header is not valid {}",
                            err.to_string()
                        ))]),
                    };

                    Ok::<_, warp::Rejection>(GqlWarpResponse::from(response))
                }
            },
        );
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let filter = graphql_playground.or(gql_post);
    warp::serve(filter).run(([0, 0, 0, 0], 8000)).await;
}
