use std::net::SocketAddr;
use serde::Deserialize;

use axum::Json;
use axum::response::IntoResponse;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

use ldap3::{LdapConn, Scope, SearchEntry};
use ldap3::result::Result;

#[tokio::main]
async fn main() {

    // Endpoint
    let routes = Router::new().route("/search", get(searchbase));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

/*******************************/
/*    Search                   */
/*******************************/
#[derive(Debug, Deserialize)]
struct SearchParams {
    dn: Option<String>,
    filter: Option<String>,
    attributes: Option<Vec<String>>,
}
async fn searchbase (Json(params): Json<SearchParams>) -> impl IntoResponse {

    // Print to our server's console
    println!("->> {:<12} - Search - {params:?}", "Handler");

    // Set our LDAP config
    let ldap_host = "ldap://192.168.0.111:389";
    let ldap_user = "cn=admin,dc=paradisecoffee,dc=cafe";
    let ldap_pass = "Worldwarcraft3!@";
    // Receive our JSON parameters
    let dn = params.dn.as_deref().unwrap_or("cn=admin,dc=paradisecoffee,dc=cafe");
    let filter = params.filter.as_deref().unwrap_or("sn=*");
    let attr: Vec<String> = params.attributes.unwrap_or(Vec::new());

    // Return some HTML
    Html(format!("DN: {dn}\n\nFilter: {filter}\n\nAttributes: {attr:?}"))
}

/*******************************/
/*    Testing                  */
/*******************************/
/* Testing JSON */
// GET http://localhost:3000/search HTTP/1.1
// User-Agent: Fiddler
// Host: localhost:3000
// Content-Type: application/json; charset=utf-8

// {
//    "dn": "Test DN",
//    "filter": "Test Filter",
//    "attributes": [
//       "firstName",
//       "givenName",
//       "sn"
//    ]
// }