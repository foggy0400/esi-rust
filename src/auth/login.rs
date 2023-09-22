extern crate open;
extern crate rand;

use base64::{engine::general_purpose, Engine as _};
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use urlencoding::encode;

/// Enum implementing an optionally customisable value for the SSO state string
pub enum StateString {
    Standard,
    Custom(String),
}

/// Enum implementing an optionally customisable value for the code verifier array
pub enum CodeVerifier {
    Standard,
    Custom([u8; 32]),
}

/// Default behaviour for the [StateString] enum.
impl Default for StateString {
    fn default() -> Self {
        StateString::Custom(Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
    }
}

/// Default behaviour for the [CodeVerifier] enum.
impl Default for CodeVerifier {
    fn default() -> Self {
        CodeVerifier::Custom(rand::thread_rng().gen::<[u8; 32]>())
    }
}

/// Function to generate a String containing the login URL for the Eve SSO.
///
/// # Arguments
///
/// * 'client_id' - the client ID of the application using the library
/// * 'callback_url' - the application's callback URL
/// * 'scope' - the scopes being requested by the application
/// * 'state_string' - SSO state string
/// * 'code_verifier' - Array of 32 u8 to be used in the code challenge
///
/// # Returns
///
/// * Formatted login URL
///
/// # Example
/// ```
/// # use esi_rust::auth::login::gen_url;
/// let mut bytes: [u8; 32] = [8; 32];
/// let url = gen_url("myclient", "mycallback", "scopes", "statestring", &bytes);
/// assert_eq!("https://login.eveonline.com/v2/oauth/authorize/?response_type=code&redirect_uri=mycallback&client_id=myclient&scope=scopes&code_challenge=CAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAg%3D&code_challenge_method=S256&state=statestring", url);
/// ```
pub fn gen_url(
    client_id: &str,
    callback_url: &str,
    scope: &str,
    state_str: &str,
    code_ver: &[u8; 32],
) -> String {
    // Define the root URI as per the Eve docs - will eventually make this a variable in a config
    // file
    let root_url: &str =
        "https://login.eveonline.com/v2/oauth/authorize/?response_type=code&redirect_uri=";

    // Use format! to construct the final URL
    let url = format!(
        "{}{}&client_id={}&scope={}&code_challenge={}&code_challenge_method=S256&state={}",
        root_url,
        encode(callback_url),
        encode(client_id),
        encode(scope),
        encode(&general_purpose::STANDARD.encode(code_ver)),
        encode(state_str)
    );
    println!("{}", url);
    url
}

/// Launches the Eve SSO login page to obtain user authentication.
/// Allows user-defined implementation of the state string and code verifier variables.
/// If you do not need to customise these variables, use [launch_login_page] instead.
/// Returns the state string and code verifier used in the login URL.
///
/// # Arguments
///
/// * 'client_id' - the client ID of the application using the library
/// * 'callback_url' - the application's callback URL
/// * 'scope' - the scopes being requested by the application
/// * 'state_string' - [StateString] object that contains the user-defined state string
/// * 'code_verifier' - [CodeVerifier] object that contains the user-defined code verifier
///
/// # Returns
///
/// * Option<[StateString], [CodeVerifier]> - the Option will be None if an error opening the login
/// page was encountered.
pub fn launch_login_page_custom(
    client_id: &str,
    callback_url: &str,
    scope: &str,
    state_string: StateString,
    code_verifier: CodeVerifier,
) -> Option<(StateString, CodeVerifier)> {
    // Unwrap the state string and code verifier enums
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

    let url = gen_url(client_id, callback_url, scope, &state_str, &code_ver);

    // Attempt to open in browser and match the result to return final Option
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

/// Launches the Eve SSO login page to obtain user authentication.
/// Uses default behaviour for the state string and code verifier.
/// See [StateString] and [CodeVerifier] for implementation details.
/// Use [launch_login_page_custom] to customise these variables.
///
/// # Arguments
///
/// * 'client_id' - the client ID of the application using the library
/// * 'callback_url' - the application's callback URL
/// * 'scope' - the scopes being requested by the application
///
/// # Returns
///
/// * Option<[StateString], [CodeVerifier]> - the Option will be None if an error opening the login
/// page was encountered.
pub fn launch_login_page(
    client_id: &str,
    callback_url: &str,
    scope: &str,
) -> Option<(StateString, CodeVerifier)> {
    launch_login_page_custom(
        client_id,
        callback_url,
        scope,
        StateString::Standard,
        CodeVerifier::Standard,
    )
}

#[cfg(test)]
mod tests {
    use super::{gen_url, CodeVerifier, StateString};
    use rand::distributions::{Alphanumeric, DistString};
    use rand::Rng;

    #[test]
    fn code_verifier_default() {
        match CodeVerifier::default() {
            CodeVerifier::Custom(val) => {
                assert!(val.len() == 32)
            }
            CodeVerifier::Standard => {
                panic!("CodeVerifier default method generated CodeVerifier::Standard.")
            }
        }
    }
    #[test]
    fn code_verifier_custom() {
        let code = rand::thread_rng().gen::<[u8; 32]>();
        match CodeVerifier::Custom(code) {
            CodeVerifier::Custom(val) => {
                assert!(val == code)
            }
            CodeVerifier::Standard => {
                panic!("CodeVerifier default method generated CodeVerifier::Standard.")
            }
        }
    }
    #[test]
    fn state_string_default() {
        match StateString::default() {
            StateString::Custom(val) => {
                assert!(val.len() == 16)
            }
            StateString::Standard => {
                panic!("StateString default method generated StateString::Standard.")
            }
        }
    }
    #[test]
    fn state_string_custom() {
        let state = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        match StateString::Custom(state.clone()) {
            StateString::Custom(val) => {
                assert!(val == state)
            }
            StateString::Standard => {
                panic!("StateString default method generated StateString::Standard.")
            }
        }
    }
}
