// GraphQL schema and resolvers

pub mod schema;
pub mod query;
pub mod mutation;



use async_graphql::{Schema, EmptySubscription, http::{GraphiQLSource, playground_source}};
use axum::{response::{Html, IntoResponse}, extract::State, http::StatusCode, Json};
use schema::{QueryRoot, MutationRoot};
use async_graphql::http::GraphQLPlaygroundConfig;

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn build_schema() -> ApiSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .finish()
}

pub async fn graphql_playground() -> impl IntoResponse {
    // Html(playground_source("/graphql"))
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub async fn graphql_handler(
    State(schema): State<ApiSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> Result<async_graphql_axum::GraphQLResponse, StatusCode> {
    Ok(schema.execute(req.into_inner()).await.into())
}

