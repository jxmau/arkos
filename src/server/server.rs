



use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::TcpListener;
use std::option::Option;

use log::{debug, error, info, warn};

use crate::server::cors::CORSHandler;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::core::status::{HttpStatusCode, StatusCode};
use crate::core::method::HttpMethod;


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
}

impl Server

 {

    /// Will return an empty Server with an inert (deactivated) CORSHandler.
    pub fn new(address: [usize; 4], port: u32 ) -> Option<Server> {
        Some(Server {address, port, routes: Vec::new(), cors_handler: CORSHandler::inert() })
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

    /// Start up the server.
    pub fn serve(&self){

        env_logger::init();

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
                Ok(mut s) => {
                    match handle_request(&mut s, &self.routes, &self.cors_handler) {
                        Ok(_s) => _s,
                        Err(_) => continue,
                    };
                },
                Err(_) =>
                continue,
            }
            
        }
    }
    
}

#[doc(hidden)]
fn handle_request(stream: &mut TcpStream, routes: &Vec<Route>, cors: &CORSHandler) -> std::io::Result<()>{
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    let b = String::from_utf8_lossy(&buffer[..bytes_read]);

    let mut response : Response = match Request::parse(&b) {
        Ok(request) => match route_request(&routes, &request, &cors) {
            Ok(response) => response,
            Err(e) => Response::generate_from_status_code(e),
        },
        Err(_) => {
            error!("Failed to parsed incoming request.");
            Response::generate_from_status_code(StatusCode::InternalServerError)},
    };


    let response: String = response.convert();

    stream.write(response.as_bytes())?;

    Ok(())
}

#[doc(hidden)]
fn route_request(paths: &Vec<Route>, req: &Request, cors: &CORSHandler) -> Result<Response, StatusCode> {
    let mut path = None;

    for p in paths {
        if req.url.eq(&p.url) && req.method.eq(&p.method){
            path = Some(p);
            break;
        }
    }

    match path {
        Some(p) => match p.is_request_valid(&req) {
                true => Ok(ask_response(&p, &req)),
                false => {
                    debug!("Request {} {} was deemed invalid : Returning 400 Bad Request.", req.method.to_string(), req.url);
                    Err(StatusCode::BadRequest)},
            },
        None => match handle_cors(paths, req, cors) {
            Some(s) => {
                debug!("Request {} {} has been rerouted for CORS handling :  Returning {} {}.", req.method.to_string(), req.url, s.status.get_code(), s.status.get_title());
                Ok(s)},
            None => {
                debug!("No route found for Request {} {} : Returning 404 Not Found.", req.method.to_string(), req.url );
                Err(StatusCode::NotFound)},
        } 
    }

}

#[doc(hidden)]
fn ask_response(route: &Route, request: &Request) -> Response {
    let response = match &route.response {
        Some(r) => match (r)(request.to_owned()) {
            Ok(s) => s, 
            Err(e) => Response::generate_from_status_code(e),
        },
        None => {
            warn!("Route found for Request {} {} , but no Response has been set.", request.method.to_string(), request.url);
            Response::default()},
    };
    debug!("Request {} {} : Returning {} {}.", request.method.to_string(), request.url, response.status.get_code(), response.status.get_title());
    response
}

#[doc(hidden)]
fn handle_cors(paths: &Vec<Route>, req: &Request, cors: &CORSHandler) -> Option<Response> {
    
    if !cors.activated {
        return None
    }

    for p in paths {
        if req.url.eq(&p.url) && req.method.eq(&HttpMethod::OPTIONS) {
            return Some(match cors.generate_response(){
                Ok(s) => return Some(s),
                Err(_) => Response::generate_from_status_code(StatusCode::InternalServerError), 
            });
        }
    }

    None
}

    


#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn not_found_request(){
       let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
       assert_eq!(Err(StatusCode::NotFound), route_request(&Vec::new(), &request, &CORSHandler::inert()));
    }

    #[test]
    fn request_found(){
        let route = Route::new("/hello", HttpMethod::GET);
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        assert_eq!(StatusCode::Ok , route_request(&vec![route], &request, &CORSHandler::inert()).unwrap().status);
    }

    #[test]
    fn bad_request(){
        let mut route = Route::new("/hello", HttpMethod::GET);
        route.add_required_url_param("name");
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        assert_eq!(Err(StatusCode::BadRequest) , route_request(&vec![route], &request, &CORSHandler::inert()));
    }

    #[test]
    fn active_cors(){
        let route = Route::new("/hello", HttpMethod::GET);
        let cors = CORSHandler::default();
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        assert_eq!(StatusCode::Ok , route_request(&vec![route], &request, &cors).unwrap().status);
    }

}