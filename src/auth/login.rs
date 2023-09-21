extern crate open;
extern crate rand;

use base64::{engine::general_purpose, Engine as _};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

use urlencoding::encode;

pub enum StateString {
    Standard,
    Custom(String),
}

pub enum CodeVerifier {
    Standard,
    Custom([u8; 32]),
}

impl Default for StateString {
    fn default() -> Self {
        StateString::Custom(Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
    }
}

impl Default for CodeVerifier {
    fn default() -> Self {
        CodeVerifier::Custom(rand::thread_rng().gen::<[u8; 32]>())
    }
}

/// Launches the Eve SSO login page to obtain user authentication.
///
/// # Arguments
///
/// * 'client_id' - the client ID of the application using the library
/// * 'callback_url' - the application's callback URL
/// * 'scope' - the scopes being requested by the application
pub fn launch_login_page(
    client_id: &str,
    callback_url: &str,
    scope: &str,
    state_string: StateString,
    code_verifier: CodeVerifier,
) -> Option<(StateString, CodeVerifier)> {
    let root_url: &str =
        "https://login.eveonline.com/v2/oauth/authorize/?response_type=code&redirect_uri=";
    let state_str = match state_string {
        StateString::Standard => {
            let def = StateString::default();
            match def {
                StateString::Custom(val) => val,
                StateString::Standard => {
                    panic!("Default behaviour for auth/login/StateString broken")
                }
            }
        }
        StateString::Custom(val) => val,
    };
    let code_ver = match code_verifier {
        CodeVerifier::Standard => {
            let def = CodeVerifier::default();
            match def {
                CodeVerifier::Custom(val) => val,
                CodeVerifier::Standard => {
                    panic!("Default behaviour for auth/login/CodeVerifier broken")
                }
            }
        }
        CodeVerifier::Custom(val) => val,
    };
    let url = format!(
        "{}{}&client_id={}&scope={}&code_challenge={}&code_challenge_method=S256&state={}",
        root_url,
        encode(callback_url),
        encode(client_id),
        encode(scope),
        encode(&general_purpose::STANDARD.encode(&code_ver)),
        encode(&state_str)
    );
    match open::that(&*url) {
        Ok(()) => Some((
            StateString::Custom(state_str),
            CodeVerifier::Custom(code_ver),
        )),
        Err(err) => {
            eprintln!("An error occurred while opening '{}': {}", &url, err);
            None
        }
    }
}

pub fn launch_login_page_default(
    client_id: &str,
    callback_url: &str,
    scope: &str,
) -> Option<(StateString, CodeVerifier)> {
    launch_login_page(
        client_id,
        callback_url,
        scope,
        StateString::Standard,
        CodeVerifier::Standard,
    )
}
