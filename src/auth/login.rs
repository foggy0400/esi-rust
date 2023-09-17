extern crate open;
extern crate reqwest;
extern crate urlencoding;

use urlencoding::encode;
pub fn generate_scope(scopes: &str) -> &str {
    println!("{}", scopes);
    "test"
}

pub fn launch_login_page(client_id: &str, callback_url: &str, scope: &str) -> bool {
    let root_url: &str =
        "https://login.eveonline.com/v2/oauth/authorize/?response_type=code&redirect_uri=";
    let unformatted_url = format!(
        "{}{}&client_id={}&scope={}",
        root_url, callback_url, client_id, scope
    );
    let url = encode(&unformatted_url);
    println!("{}", url);
    match open::that(&*url) {
        Ok(()) => true,
        Err(err) => {
            eprintln!("An error occurred while opening '{}': {}", &url, err);
            false
        }
    }
}
