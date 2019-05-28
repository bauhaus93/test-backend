#[macro_use]
extern crate log;
extern crate hyper;
extern crate futures;

extern crate test_backend;

use std::sync::{ Arc, RwLock };
use hyper::{ Server, Request, Response, Body };
use hyper::service::{ Service, service_fn, service_fn_ok };
use futures::{ future, Future };

use test_backend::utility::init_logger;
use test_backend::application::{ Application, ApplicationError };

fn run_server(addr: &str) {
    let addr = match addr.parse() {
            Ok(p) => p,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };

    let app = match Application::new() {
        Ok(app) => Arc::new(RwLock::new(app)),
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let make_service = move || {
        let instance = app.clone();
        service_fn_ok(move |req: Request<Body>| {
                match instance.read() {
                    Ok(guard) => (*guard).request(req),
                    Err(_poisoned) => {
                        error!("RwLock poisoned!");
                        Response::builder()
                            .status(500)
                            .body(Body::from("Something bad happened."))
                            .unwrap()
                    }
                }
                
            }
        )
    };

    let server = Server::bind(&addr)
        .serve(make_service)
        .map_err(|e| error!("{}", e));
    hyper::rt::run(server);
}

fn main() {
    init_logger();

    run_server("127.0.0.1:12345");
    
}

