use std::net::SocketAddr;
use serde::Deserialize;
use axum::response::IntoResponse;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use axum::extract::Query;

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/search", get(searchbase));

    // Start listening server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    dn: Option<String>,
    filter: Option<String>,
    attributes: Option<Vec<String>>,
    
}
async fn searchbase (Query(params): Query<SearchParams>) -> impl IntoResponse {
    println!("->> {:<12} - Search - {params:?}", "Handler");

    let dn = params.dn.as_deref().unwrap_or("cn=admin,dc=paradisecoffee,dc=cafe");
    let filter = params.filter.as_deref().unwrap_or("sn=*");

    Html(format!("DN: {dn}, Filter: {filter}"))

}
