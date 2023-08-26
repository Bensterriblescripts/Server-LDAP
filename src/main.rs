use ldap3::{LdapConn, Scope, SearchEntry};
use ldap3::result::Result;

fn main() {

    // Configuration
    let ldap_host = "ldap://192.168.0.111:389";
    let ldap_base = "dc=paradisecoffee,dc=cafe";
    let ldap_config = [ldap_host, ldap_base];

    // Settings
    let ldap_function = "SearchSub"; // We want this to be passed into main, currently hardcoded
    let ldap_filter = "(sn=Nanson)";
    let ldap_attr = ["sn", "displayName", "givenName"];


    /*******************************/
    /*    Functions                */
    /*******************************/

    // Search
    if ldap_function == "SearchBase" {
        let results = ldapsearch_base(ldap_config, &ldap_filter, ldap_attr);
        for result in results {
            for record in result {
                for (attr_name, attr_values) in &record.attrs {
                    println!("{}: ", attr_name); // Attribute name e.g. givenName
                    for value in attr_values {
                        println!("{}", value); // Attribute value e.g. Ben Nanson
                    }
                }
            }
        }
    }
    else if ldap_function == "SearchSub" {
        let results = ldapsearch_sub(ldap_config, &ldap_filter, ldap_attr);
        for result in results {
            for record in result {
                for (attr_name, attr_values) in &record.attrs {
                    println!("{}: ", attr_name); // Attribute name e.g. givenName
                    for value in attr_values {
                        println!("{}", value); // Attribute value e.g. Ben Nanson
                    }
                }
            }
        }
    }
    else {
        panic!("Invalid call");
    }
}


/*********************************/
/*    Search                     */
/*********************************/
fn ldapsearch_base(ldap_config: [&str; 2], ldap_filter: &str, ldap_attr: [&str; 3]) -> Result<Vec<SearchEntry>> {
    let mut ldap = LdapConn::new(&ldap_config[0])?;
    let (rs, _res) = ldap.search(
        &ldap_config[1],
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
fn ldapsearch_sub(ldap_config: [&str; 2], ldap_filter: &str, ldap_attr: [&str; 3]) -> Result<Vec<SearchEntry>> {
    let mut ldap = LdapConn::new(&ldap_config[0])?;
    let (rs, _res) = ldap.search(
        &ldap_config[1],
        Scope::Subtree,
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