use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

pub struct CallbackListener {
    hostname: String,
    port: u16,
}

async fn handle_code(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("handling!");
    Ok(Response::new("handled".into()))
}

impl CallbackListener {
    pub fn new() -> CallbackListener {
        CallbackListener {
            hostname: String::from("localhost"),
            port: 3000,
        }
    }
    pub async fn start_server(self) {
        // We'll bind to 127.0.0.1:3000
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        // A `Service` is needed for every connection, so this
        // creates one from our `hello_world` function.
        let make_svc = make_service_fn(|_conn| async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(handle_code))
        });

        let server = Server::bind(&addr).serve(make_svc);

        // Run this server for... forever!
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}
