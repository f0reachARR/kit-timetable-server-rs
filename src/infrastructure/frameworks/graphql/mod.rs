use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Request, Schema,
};
use async_graphql_warp::{graphql, Response as GqlResponse};
use std::convert::Infallible;
use warp::{http::Response, Filter};

mod resolvers;
use resolvers::*;

use super::container::UsecaseContainer;

type SchemaType = Schema<QueryRoot, MutationRoot, EmptySubscription>;
pub async fn start_graphql(usecases: UsecaseContainer) {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(usecases)
    .finish();

    let gql_post =
        graphql(schema).and_then(|(schema, request): (SchemaType, Request)| async move {
            // Execute query
            let resp = schema.execute(request).await;

            // Return result
            Ok::<_, Infallible>(GqlResponse::from(resp))
        });
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let filter = graphql_playground.or(gql_post);
    warp::serve(filter).run(([0, 0, 0, 0], 8000)).await;
}
