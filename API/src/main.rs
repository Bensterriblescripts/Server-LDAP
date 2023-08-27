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
    let routes = Router::new().route("/search", get(search));
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
// API
async fn search (Json(params): Json<SearchParams>) -> impl IntoResponse {

    println!("->> {:<12} - Search - {params:?}", "Handler");

    let ldap_dn = params.dn.as_deref().unwrap_or("cn=admin,dc=paradisecoffee,dc=cafe");
    let ldap_filter = params.filter.as_deref().unwrap_or("sn=*");
    let ldap_attr: Vec<String> = params.attributes.unwrap_or(Vec::new());

    let results = ldap_search(&ldap_dn, &ldap_filter, ldap_attr);
    for result in results {
        for record in result {
            for (attr_name, attr_values) in &record.attrs {
                print!(" Attr:{:?} Value:{:?}, ", attr_name, attr_values);
            }
        }
    }

    // Return some HTML
    // Html(format!("DN: {ldap_dn}\n\nFilter: {ldap_filter}\n\nAttributes: {ldap_attr:?}"))
}
//LDAP
async fn ldap_search (ldap_dn: &str, ldap_filter: &str, ldap_attr: Vec<String>) -> Result<Vec<SearchEntry>> {
    let ldap_host = "ldap://192.168.0.111:389";
    let ldap_user = "cn=admin,dc=paradisecoffee,dc=cafe";
    let ldap_pass = "Worldwarcraft3!@";
    let ldap_config = [ldap_host, ldap_user, ldap_pass];

    let mut ldap = LdapConn::new(&ldap_config[0])?;
    let bind = ldap.simple_bind(&ldap_config[1], &ldap_config[2])?.success()?;
    let (rs, _res) = ldap.search(
        &ldap_dn,
        Scope::Base,
        &ldap_filter,
        &ldap_attr
    )?.success()?;
    let search_entries: Vec<SearchEntry> = rs
        .into_iter()
        .map(|entry| SearchEntry::construct(entry))
        .collect();
    
    ldap.unbind()?;
    Ok(search_entries)
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