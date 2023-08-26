use ldap3::{LdapConn, Scope, SearchEntry};
use ldap3::result::Result;

fn main() {
    
    // LDAP Function Types:
    // 0 - SearchBase       4 - Add Person 
    // 1 - SearchSub        5 - Add Attribute
    // 2 - SearchOne        6 - Delete Person
    // 3 - Replace          7 - Delete Attribute

    // Received values
    let ldap_function: i8 = 1; // We want this to be passed in, currently hardcoded
    let ldap_location = "dc=paradisecoffee,dc=cafe";

    /*******************************/
    /*    Configuration            */
    /*******************************/
    let ldap_host = "ldap://192.168.0.111:389";
    let ldap_user = "cn=admin,dc=paradisecoffee,dc=cafe";
    let ldap_pass = "password";
    let ldap_config = [ldap_host, ldap_user, ldap_pass];

    /*******************************/
    /*    Functions                */
    /*******************************/

    // Search
    if ldap_function == 0 {
        
        // Query
        let ldap_filter = "(sn=Nanson)";
        let ldap_attr = ["sn", "displayName", "givenName"];
        let results = ldap_searchbase(ldap_config, &ldap_location, &ldap_filter, ldap_attr);

        // Result
        for result in results {
            for record in result {
                for (attr_name, attr_values) in &record.attrs {
                    print!(" Attr:{:?} Value:{:?}, ", attr_name, attr_values);
                }
            }
        }
    }
    else if ldap_function == 1 {

        // Query
        let ldap_filter = "(sn=Nanson)";
        let ldap_attr = ["sn", "displayName", "givenName"];
        let results = ldap_searchsub(ldap_config, &ldap_location, &ldap_filter, ldap_attr);

        // Result
        for result in results {
            for record in result {
                for (attr_name, attr_values) in &record.attrs {
                    print!(" Attr:{:?} Value:{:?}, ", attr_name, attr_values);
                }
            }
        }
    }

    // Replace

    else {
        panic!("Invalid call");
    }
}


/*********************************/
/*    Search                     */
/*********************************/
fn ldap_searchbase(ldap_config: [&str; 3], ldap_location: &str, ldap_filter: &str, ldap_attr: [&str; 3]) -> Result<Vec<SearchEntry>> {
    let mut ldap = LdapConn::new(&ldap_config[0])?;
    ldap.simple_bind(&ldap_config[1], &ldap_config[2])?;
    let (rs, _res) = ldap.search(
        &ldap_location,
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
fn ldap_searchsub(ldap_config: [&str; 3], ldap_location: &str, ldap_filter: &str, ldap_attr: [&str; 3]) -> Result<Vec<SearchEntry>> {
    let mut ldap = LdapConn::new(&ldap_config[0])?;
    ldap.simple_bind(&ldap_config[1], &ldap_config[2])?;
    let (rs, _res) = ldap.search(
        &ldap_location,
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