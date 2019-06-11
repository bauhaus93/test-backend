#[macro_use]
extern crate log;
extern crate hyper;
extern crate futures;

extern crate test_backend;

use std::sync::{ Arc, RwLock };
use hyper::{ Server, Request, Body };
use hyper::service::{ service_fn, make_service_fn };
use hyper::server::conn::AddrStream;
use futures::{ Future };

use test_backend::utility::init_logger;
use test_backend::application::{ Application, StaticResponse };

fn main() {
    const SERVER_ADDR: &'static str = "127.0.0.1:12345";
    init_logger();

    info!("Running server on {}", SERVER_ADDR);
    run_server(SERVER_ADDR); 
    info!("Application finished");
}

fn run_server(addr: &str) {
    let addr = match addr.parse() {
            Ok(p) => p,
            Err(e) => {
                error!("Server address parsing failed: {}", e);
                return;
            }
        };

    let app = match Application::new() {
        Ok(app) => Arc::new(RwLock::new(app)),
        Err(e) => {
            error!("Application creation failed: {}", e);
            return;
        }
    };

    let make_service = make_service_fn(move |socket: &AddrStream| {
        info!("Incoming connection from {}", socket.remote_addr());
        let instance = app.clone();
        service_fn(move |req: Request<Body>| {
                match instance.read() {
                    Ok(guard) => (*guard).request(req),
                    Err(_poisoned) => {
                        error!("RwLock poisoned!");
                        StaticResponse::fallback_500()
                    }
                }
                
            }
        )
    });

    let server = Server::bind(&addr)
        .serve(make_service)
        .map_err(|e| error!("{}", e));
    hyper::rt::run(server);
}