use tide::Request;
use serde_json::json;

mod util;
mod dbs;
mod gql;

mod users;
mod articles;

use crate::util::{constant::CFG, common::Tpl};
use crate::articles::routes::articles_index;
use crate::users::routes::users_index;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app = tide::with_state(State(gql::build_schema().await));

    //environment variables defined in .env file
    app.at("/").get(index);
    app.at("users").get(users_index);
    app.at("articles").get(articles_index);

    // app.at(ENV.get("GRAPHQL_PATH").unwrap()).post(async_graphql_tide::endpoint(schema));
    app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(gql::graphql);
    app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(gql::graphiql);

    app.listen(format!("{}:{}", CFG.get("ADDRESS").unwrap(), CFG.get("PORT").unwrap())).await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State(
    pub  async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
);

async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data = json!({"app_name": "blog-rs", "author": "zzy"});

    index.render(&data).await
}
