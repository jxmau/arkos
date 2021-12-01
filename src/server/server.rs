
use std::io::{Read, Write};
use std::net::TcpStream;
use std::net::TcpListener;
use std::ops::Deref;
use std::option::Option;

use std::sync::{Mutex, Arc};

use log::{debug, error, info, trace};

use tokio::task;

use crate::server::cors::CORSHandler;
use crate::server::request::Request;
use crate::server::response::Response;
use crate::core::status::{HttpStatusCode, StatusCode};
use crate::core::method::HttpMethod;


use super::checkpoint::Checkpoint;
use super::checkpoint_manager::CheckpointManager;
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
                Ok(s) => {

                    let stream = Arc::new(Mutex::new(s));
                    let routes = Arc::new(Mutex::new(self.routes.clone()));
                    let cors = Arc::new(Mutex::new(self.cors_handler.clone()));
                    let checkpoints = Arc::new(Mutex::new(self.checkpoints.clone()));
                    let _handle = task::spawn(async {

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


    // TODO: Perhaps use a Request Factory that will be able to find the HTTP param in the request, that returns an enu

    let mut response : Response = match Protocol::parse_from_raw(&b) {
        Protocol::Http11 => {
            trace!("Request received has Protocol HTTP/1.1 - Routed for Request parsing.");
            match Request::parse(&b) {
                Ok(request) => { 
                    match route_request(routes, &request, cors, checkpoints) {
                        Ok(response) => response,
                        Err(e) => Response::generate_from_status_code(e),
                    }
                },
                Err(e) => {
                    error!("Failed to parsed incoming request. ");
                    Response::generate_from_status_code(e)},
            }},
        Protocol::Error => {
            trace!("Fail to know which Transfert Protocol Request used. Returning 501 HTTP VersionNot Supported");
            Response::generate_from_status_code(StatusCode::HTTPVersionNotSupported)
        }
    };

    let response: String = response.convert();

    stream.write(response.as_bytes())?;

    Ok(())
}

#[doc(hidden)]
fn route_request(paths: Arc<Mutex<Vec<Route>>>, req: &Request, cors: Arc<Mutex<CORSHandler>>, checkpoints: Arc<Mutex<Vec<Checkpoint>>>) -> Result<Response, StatusCode> {

    {
        for check in checkpoints.lock().unwrap().deref() {
            let manager = CheckpointManager::new(check.to_owned());
            match manager.verify(req.to_owned()) {
                Some(e) => {
                    // TODO We generated the Error here, but to me moved at the lower level.
                    trace!("Request {} {} failed to pass a server checkpoint", req.method.to_string(), req.url);
                    return Ok(Response::generate_from_status_code(e))
                },
                None => continue,
            }
        }
    }

    let routes = paths.lock().unwrap().clone();
    let route = routes.iter().find(|r| req.url.eq(&r.url) && req.method.eq(&r.method));

    match route {
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
fn handle_cors(paths: Arc<Mutex<Vec<Route>>>, req: &Request, cors: Arc<Mutex<CORSHandler>>) -> Option<Response> {
    
    let cors_deref = cors.lock().unwrap().clone();
    let cors_bool = cors_deref.activated;
    if !cors_bool {
        return None
    }
    
    for p in paths.lock().unwrap().deref() {
        if req.url.eq(&p.url) && req.method.eq(&HttpMethod::OPTIONS) {
            return Some(match cors_deref.generate_response(){
                Ok(s) => return Some(s),
                Err(_) => Response::generate_from_status_code(StatusCode::InternalServerError), 
            });
        }
    }
    
    None
}


#[doc(hidden)]
fn ask_response(route: &Route, request: &Request) -> Response {


    for check in &route.checks {
        match (check)(request.to_owned()){
            Ok(_) => continue,
            Err(e) => {
                trace!("Request {} {} failed to pass a route checkpoint", request.method.to_string(), request.url);
                return Response::generate_from_status_code(e);
            },
        }
    }    


    let response = match (&route.response)(request.to_owned()) {
        Ok(r) => r,
        Err(e) => Response::generate_from_status_code(e),
    };

    debug!("Request {} {} : Returning {} {}.", request.method.to_string(), request.url, response.status.get_code(), response.status.get_title());
    response
}



#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[test]
    fn not_found_request(){
       let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
       let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(Vec::new())) ;
       let cors = Arc::new(Mutex::new(CORSHandler::inert()));
       let checkpoints = Arc::new(Mutex::new(vec![]));
       assert_eq!(Err(StatusCode::NotFound), route_request(routes, &request, cors, checkpoints));
    }

    #[test]
    fn request_found(){
        let route = Route::new("/hello", HttpMethod::GET);
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(StatusCode::Ok , route_request(routes, &request, cors, checkpoints).unwrap().status);
    }

    #[test]
    fn bad_request(){
        let mut route = Route::new("/hello", HttpMethod::GET);
        route.add_required_url_param("name");
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(Err(StatusCode::BadRequest) , route_request(routes, &request, cors, checkpoints));
    }

    #[test]
    fn active_cors(){
        let route = Route::new("/hello", HttpMethod::GET);
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(StatusCode::Ok , route_request(routes, &request, cors, checkpoints).unwrap().status);
    }


    fn check() -> Arc<dyn Fn(Request) -> Result<(), StatusCode> + Send + Sync> {
        Arc::new(|request: Request| {
            match request.param.contains_key("security") {
                true => Ok(()),
                false => Err(StatusCode::BadRequest),
            }
        })
    } 

    // With server checkpoints
    #[test]
    fn server_checkpoint_valid(){
        let route = Route::new("/hello".into(), HttpMethod::GET);

        let mut headers : HashMap<String, String>= HashMap::new();
        headers.insert("security".into(), "value".into());
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: headers, body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(StatusCode::Ok, route_request(routes, &request, cors, checkpoints).unwrap().status);
     }

     #[test]
    fn server_checkpoint_invalid(){
        let route = Route::new("/hello".into(), HttpMethod::GET);
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(StatusCode::BadRequest, route_request(routes, &request, cors, checkpoints).unwrap().status);
     }

     // With route check
     #[test]
    fn route_check_valid(){
        
        let mut route = Route::new("/hello".into(), HttpMethod::GET);
        route.add_check(check());
        let mut headers : HashMap<String, String>= HashMap::new();
        headers.insert("security".into(), "value".into());
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: headers, body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(StatusCode::Ok, route_request(routes, &request, cors, checkpoints).unwrap().status);
     }

     #[test]
    fn route_check_invalid(){
        let mut route = Route::new("/hello".into(), HttpMethod::GET);
        route.add_check(check());
        let request = Request {method: HttpMethod::GET, url: "/hello".into(), headers: HashMap::new(), cookies: HashMap::new(), param: HashMap::new(), body: String::new() };
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(StatusCode::BadRequest, route_request(routes, &request, cors, checkpoints).unwrap().status);
     }



}
