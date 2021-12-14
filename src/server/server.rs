
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::TcpListener;

use std::option::Option;

use std::sync::{Mutex, Arc};

use log::{error, info, trace};

// use tokio::task;


use crate::handler::http1::handle_http1_request;
use crate::server::cors::CORSHandler;


use crate::core::status::{ StatusCode};
use crate::wrapper::response_factory::ResponseFactory;



use super::checkpoint::Checkpoint;

use super::protocol::Protocol;

use super::route::Route;

pub struct Server{
    #[doc(hidden)]
    address: [usize; 4],
    #[doc(hidden)]
    port: u32,
    #[doc(hidden)]
    routes: Vec<Route>,
    #[doc(hidden)]
    cors_handler: CORSHandler,
    #[doc(hidden)]
    checkpoints: Vec<Checkpoint>,
}




impl Server

 {

    /// Will return an empty Server with an inert (deactivated) CORSHandler.
    pub fn new(address: [usize; 4], port: u32 ) -> Option<Server> {
        Some(Server {address, port, routes: Vec::new(), cors_handler: CORSHandler::inert(), checkpoints: Vec::new()})
    }

    /// Will set the routes as Arkos doesn't use a Router kind of struct.
    /// Note regarding CORS Request: Arkos server will reroute request to the CORSHandler only if it cannot found an OPTIONS request with the url. Add an OPTIONS only if you want to override the CORS Handler.
    pub fn set_routes(&mut self, routes: Vec<Route>) {
        self.routes = routes;
    }

    /// Will replace the CORSHandler. 
    pub fn set_cors_handler(&mut self, cors: CORSHandler) {
        self.cors_handler = cors;
    }

    /// Add a checkpoint to be executed after the Request has been parsed.
    pub fn add_checkpoint(&mut self, checkpoint: Checkpoint) {
        self.checkpoints.push(checkpoint);
    }


    /// Start up the server.
    pub fn serve(&self){



        info!("{} route(s) found.", &self.routes.len());
        if self.cors_handler.activated {
            info!("CORS Handler is activated.")
        } else {
            info!("CORS Handler is deactivated.")
        }


        let address = format!("{:?}.{:?}.{:?}.{:?}:{:?}", &self.address.get(0).unwrap(), &self.address.get(1).unwrap(), &self.address.get(2).unwrap(), &self.address.get(3).unwrap(), &self.port);

        let listener = match TcpListener::bind(&address){
            Ok(s) => {
                info!("Server has been successfully launched on port {}.", self.port);
                s
            },
            Err(_) => {
                error!("Failed to use port {}.", self.port);
                std::process::exit(1);
            }
        };


        for stream in listener.incoming(){

            match stream {
                Ok(s) => {

                    let stream = Arc::new(Mutex::new(s));
                    let routes = Arc::new(Mutex::new(self.routes.clone()));
                    let cors = Arc::new(Mutex::new(self.cors_handler.clone()));
                    let checkpoints = Arc::new(Mutex::new(self.checkpoints.clone()));

                    let _handle = async_std::task::spawn(async {

                        match handle_request(stream, routes, cors, checkpoints) {
                            Ok(_s) => trace!("Succesful handling of request."),
                            Err(_) => trace!("Failed to handle request."),
                        };
                    });

                },
                Err(_) =>
                continue,
            }
            
        }
    }
    
}

#[doc(hidden)]
fn handle_request(stream: Arc<Mutex<TcpStream>>, routes: Arc<Mutex<Vec<Route>>>, cors: Arc<Mutex<CORSHandler>>, checkpoints: Arc<Mutex<Vec<Checkpoint>>>) -> std::io::Result<()>{
    
    let mut stream = stream.lock().unwrap();
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    let b = String::from_utf8_lossy(&buffer[..bytes_read]);




    let mut response_factory  : ResponseFactory = match Protocol::parse_from_raw(&b) {
        Ok(protocol_parsed) => match protocol_parsed {
            Protocol::Http1(v) => {
                trace!("Request received has Protocol HTTP/1.{} - Routed for Request handling", v);
                match handle_http1_request(&v, routes, &b, cors, checkpoints) {
                    Ok(r) => r,
                    Err(e) => ResponseFactory::for_status_code(Protocol::Http1(v), e),
                }
            },
            _ => {            
            trace!("Fail to know which Transfert Protocol Request used. Returning 505 HTTP Version Not Supported");
            ResponseFactory::for_status_code(Protocol::Http1(0), StatusCode::HTTPVersionNotSupported)
        }},
        _ => {
            trace!("Fail to know which Transfert Protocol Request used. Returning 505 HTTP Version Not Supported");
            ResponseFactory::for_status_code(Protocol::Http1(0), StatusCode::HTTPVersionNotSupported)
        }
    };


    let response: String = response_factory.consume();

    stream.write(response.as_bytes())?;

    Ok(())
}
