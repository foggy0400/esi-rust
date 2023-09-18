extern crate open;
extern crate rand;

use base64::{engine::general_purpose, Engine as _};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

use urlencoding::encode;
/// Placeholder function to generate a space-separated list of scopes.
/// May be broken out into separate folder.
pub fn generate_scope(scopes: &str) -> &str {
    println!("{}", scopes);
    "test"
}

/// Launches the Eve SSO login page to obtain user authentication.
///
/// # Arguments
///
/// * 'client_id' - the client ID of the application using the library
/// * 'callback_url' - the application's callback URL
/// * 'scope' - the scopes being requested by the application
pub fn launch_login_page(client_id: &str, callback_url: &str, scope: &str) -> bool {
    let root_url: &str =
        "https://login.eveonline.com/v2/oauth/authorize/?response_type=code&redirect_uri=";
    let state_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let code_verifier = rand::thread_rng().gen::<[u8; 32]>();
    let url = format!(
        "{}{}&client_id={}&scope={}&code_challenge={}&code_challenge_method=S256&state={}",
        root_url,
        encode(callback_url),
        encode(client_id),
        encode(scope),
        encode(&general_purpose::STANDARD.encode(&code_verifier)),
        encode(&state_string)
    );
    match open::that(&*url) {
        Ok(()) => true,
        Err(err) => {
            eprintln!("An error occurred while opening '{}': {}", &url, err);
            false
        }
    }
}
