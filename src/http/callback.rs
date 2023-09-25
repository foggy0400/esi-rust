use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode, Uri};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use url::form_urlencoded;

use crate::auth::sso_login::StateString;

type SharedString = Arc<Mutex<String>>;

pub struct CallbackListener {
    addr: SocketAddr,
    uri: String,
    state: StateString,
}
async fn handle_code(
    req: Request<Body>,
    code_arc: Arc<Mutex<String>>,
    uri: &str,
    state: &StateString,
) -> Result<Response<Body>, hyper::Error> {
    let code_data = code_arc.lock().unwrap();
    match (req.method(), req.uri().path()) {
        (&Method::GET, uri) => {
            println! {"{}", uri};
            println! {"{:?}", state};
            Ok(Response::new("Callback received".into()))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

impl CallbackListener {
    pub fn new(host: [u8; 4], port: u16, uri: &str, state: StateString) -> CallbackListener {
        CallbackListener {
            addr: SocketAddr::from((host, port)),
            uri: uri.to_string(),
            state: state,
        }
    }
    pub async fn listen(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync + '_>> {
        let code_str = Arc::new(Mutex::new(String::new()));
        let uri = self.uri.clone();
        let state = self.state.clone();
        let make_srv = make_service_fn(move |_conn| {
            let code_str = code_str.clone();
            let uri = uri.clone();
            let state = state.clone();
            let srv = service_fn(move |req| {
                let code_str = code_str.clone();
                let uri = uri.clone();
                let state = state.clone();
                async move { handle_code(req, code_str, &uri, &state).await }
            });
            async move { Ok::<_, Infallible>(srv) }
        });
        let srv = Server::bind(&self.addr).serve(make_srv);
        if let Err(e) = srv.await {
            eprintln!("server error: {}", e);
        }
        Ok(String::from("placeholder!"))
    }
}
