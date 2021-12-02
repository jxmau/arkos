use std::{sync::{Mutex, Arc}, ops::Deref};

use log::{trace, debug};

use crate::{server::{route::Route, cors::CORSHandler, checkpoint::Checkpoint, response::Response}, core::{status::{StatusCode, HttpStatusCode}, method::HttpMethod}, wrapper::{request_factory::parse_http1x, checkpoint_manager::CheckpointManager}};

// What is needed
// The global checkpoint
// The route
// 

pub fn handle_http1_request(p_subversion: &u8, paths: Arc<Mutex<Vec<Route>>>, req: &str, cors: Arc<Mutex<CORSHandler>>, checkpoints: Arc<Mutex<Vec<Checkpoint>>>) -> Result<Response, StatusCode> {
    
    // * Handler:
    // 0. Check if protocol sub-version is taken in charge.
    // No need as the if not taken in charge, a StatusCode should've been sent back when parsing the Protocol.


    // 1. Parse the Request
    let request = match parse_http1x(req, p_subversion) {
        Ok(req) => req,
        Err(e) => {
            debug!("Failed to parse Request with Protocol HTTP/1.{} - Returning {} {}", p_subversion, e.get_code(), e.get_title());
            return Err(e)},
    };

    
    // 2. Pass global Checkpoint
    {
        for check in checkpoints.lock().unwrap().deref() {
            let manager = CheckpointManager::new(check.to_owned());
            match manager.verify(request.to_owned()) {
                Some(e) => {
                    debug!("Request {} {} failed to pass a server checkpoint - Returning {} {}", request.method.to_string(), request.url, e.get_code(), e.get_title());
                    return Err(e)
                },
                None => continue,
            }
        }
    }
    trace!("Request {} {} has passed the Server Checkpoints.", request.method.to_string(), request.url);
    
    // 3. Find route of Request
    // 4. Pass the CORS Handler if necessary
    let routes = paths.lock().unwrap();
    let route = routes.iter().find(|r| request.url.eq(&r.url) && request.method.eq(&r.method));


    let route_found = match route {
        Some(r) => r,
        None => {
            let cors = cors.lock().unwrap();
            if cors.activated && request.method.eq(&HttpMethod::OPTIONS){
                trace!("No Route found for OPTIONS Request, but CORS Handler is activated. "); 
                for route in routes.iter() {
                    if route.url.eq(&request.url) {
                        match cors.generate_response() {
                            Ok(r) => {
                                debug!("Request {} {} has been rerouted to the CORS Handler.", request.method.to_string(), request.url);
                                return Ok(r);
                            }
                            Err(_) => {
                                debug!("An issue has occurent when generating CORS Handler for Request {} {}", request.method.to_string(), request.url);
                                return Err(StatusCode::InternalServerError);
                            }
                        }
                    }
                }
            }
            debug!("Server hasn't found a Route for Request {} {} - Returning 404 Not Found", request.method.to_string(), request.url);
            return Err(StatusCode::NotFound)
        }
    };

    trace!("Server has found a Route for Request {} {}", request.method.to_string(), request.url);

    // 5. Verify the Request is valid
    match route_found.is_request_valid(&request) {
        true => trace!("Request {} {} has been deemed valid for and by the Route", request.method.to_string(), request.url),
        false => {
            trace!("Request {} {} has been deemed valid for and by the Route", request.method.to_string(), request.url);
            return Err(StatusCode::BadRequest)
        }
    };

    // 6. Pass Route Checks
    
    for check in &route_found.checks {
        match (check)(request.to_owned()){
            Ok(_) => continue,
            Err(e) => {
                debug!("Request {} {} failed to pass a route checkpoint - Returning {} {}", request.method.to_string(), request.url, e.get_code(), e.get_title());
                return Err(e);
            },
        }
    }    
    
    trace!("Request {} {} has passed the Route Checks.", request.method.to_string(), request.url);


    // 7. Ask for the Response.
    let response = match (&route_found.response)(request.to_owned()) {
        Ok(r) => r,
        Err(e) => {
            debug!("An error has been returned when calling the Response function of the Route for Request {} {} - Returning {} {} ", request.method.to_string(), request.url, e.get_code(), e.get_title());
            return Err(e);
        },
    };

    debug!("Request {} {} : Returning {} {}.", request.method.to_string(), request.url, response.status.get_code(), response.status.get_title());
    Ok(response)

}


#[cfg(test)]
mod test {

    use crate::server::request::Request;

    use super::*;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    
    #[test]
    fn not_found_request(){
        
        let request = "GET /hello HTTP/1.1".to_string();

        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(Vec::new())) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));

        assert_eq!(Err(StatusCode::NotFound), handle_http1_request(&1, routes, &request, cors, checkpoints));
    }

    #[test]
    fn request_found(){
        let route = Route::new("/hello", HttpMethod::GET);
        let request = "GET /hello HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(StatusCode::Ok , handle_http1_request(&1, routes, &request, cors, checkpoints).unwrap().status);
    }
    
    #[test]
    fn required_param_invalid(){
        let mut route = Route::new("/hello", HttpMethod::GET);
        route.add_required_url_param("name");
        let request = "GET /hello HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(Err(StatusCode::BadRequest) , handle_http1_request(&1, routes, &request, cors, checkpoints));
    }
    
    #[test]
    fn required_param_valid(){
        let mut route = Route::new("/hello", HttpMethod::GET);
        route.add_required_url_param("name");
        let request = "GET /hello?name=Bernard HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(StatusCode::Ok, handle_http1_request(&1, routes, &request, cors, checkpoints).unwrap().status);
    }
    
    #[test]
    fn active_cors(){
        let route = Route::new("/hello", HttpMethod::GET);
        let request = "OPTIONS /hello HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::default()));
        let checkpoints = Arc::new(Mutex::new(vec![]));
        assert_eq!(StatusCode::Ok , handle_http1_request(&1, routes, &request, cors, checkpoints).unwrap().status);
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
        let request = "GET /hello?security=Bernard HTTP/1.1".to_string();
        
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(StatusCode::Ok, handle_http1_request(&1, routes, &request, cors, checkpoints).unwrap().status);
     }

     #[test]
    fn server_checkpoint_invalid(){
        let route = Route::new("/hello".into(), HttpMethod::GET);
        let request = "GET /hello HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(Err(StatusCode::BadRequest), handle_http1_request(&1, routes, &request, cors, checkpoints));
     }

     // With route check
     #[test]
    fn route_check_valid(){
        
        let mut route = Route::new("/hello".into(), HttpMethod::GET);
        route.add_check(check());
        let mut headers : HashMap<String, String>= HashMap::new();
        headers.insert("security".into(), "value".into());
        let request = "GET /hello?security=Bernard HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(StatusCode::Ok, handle_http1_request(&1, routes, &request, cors, checkpoints).unwrap().status);
     }

     #[test]
    fn route_check_invalid(){
        let mut route = Route::new("/hello".into(), HttpMethod::GET);
        route.add_check(check());
        let request = "GET /hello HTTP/1.1".to_string();
        let routes : Arc<Mutex<Vec<Route>>> =Arc::new(Mutex::new(vec![route])) ;
        let cors = Arc::new(Mutex::new(CORSHandler::inert()));
        let check = Checkpoint::new(vec!["/hello".into()], check());
        let checkpoints = Arc::new(Mutex::new(vec![check]));
        assert_eq!(Err(StatusCode::BadRequest), handle_http1_request(&1, routes, &request, cors, checkpoints));
     }

}