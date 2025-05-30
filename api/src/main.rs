use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::GraphQL;
use dotenv::dotenv;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use sea_orm::Database;
use seaography::{async_graphql, lazy_static};
use std::env;

lazy_static::lazy_static! {
    static ref URL: String = env::var("URL").unwrap_or("[::]:8000".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into()) ;
    static ref DATABASE_URL: String = env::var("DATABASE_URL").expect ("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option < usize > = env::var("DEPTH_LIMIT").map_or (None, |data| Some (data.parse().expect("DEPTH_LIMIT is not a number")));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT").map_or(None, |data| { Some (data.parse ().expect ("COMPLEXITY_LIMIT is not a number")) });
}

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(&ENDPOINT)))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();
    let database = Database::connect(&*DATABASE_URL)
        .await
        .expect("Fail to initialize database connection");
    let schema = api::query_root::schema(database, *DEPTH_LIMIT, *COMPLEXITY_LIMIT).unwrap();
    let app = Route::new().at(
        &*ENDPOINT,
        get(graphql_playground).post(GraphQL::new(schema)),
    );
    println!("Visit GraphQL Playground at http://{}", *URL);
    Server::new(TcpListener::bind(&*URL))
        .run(app)
        .await
        .expect("Fail to start web server");
}
