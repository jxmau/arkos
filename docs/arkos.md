Arkos is a blocking http web-server inspired by axum, warp and Spring AOP.

# Summary

- [Disclaimer](#Disclaimer)
- [Features](#Features)
- [Log](#Log)
- [Example](#Example)



# Disclaimer

Arkos is unstable and not production-ready, nor it would in the near future as its architecture, inner functionment and end-user use could change.
<br> 
If you need to use a webserver library, please check-out axum or warp. 

# Features

* Integrated CORS Handling,
* HTTP Methods: GET POST DELETE PUT OPTIONS,
* System of Error Status Code Error.

# Log

Arkos uses the crate [log](https://docs.rs/log).

| Level | What for? | Example |
| --- | --- | --- |
| error | Anormal behaviour that required to end the program. | Failed to listen to specified sport |
| warn | Anormal behaviour that doesn't impact the normal functionment of the server. | Route doesn't have a Response set | 
| info | Normal behaviour of the server. | Start-up logs |
| debug | Show normal behaviour | Where is the request routed  |
| trace | Show inner functionment | Will display the missing required fields when checking the validity of the Request |


# Example

```ignore

pub fn main() {
    let mut server : Server = Server::new([127, 0, 0, 1], 8080).unwrap();

    let mut routes: Vec<Route> = Vec::new();
    let mut hello = Route::new("/hello", HttpMethod::GET);
    hello.set_response(Arc::new(|_req: Request| {give_response()}));

    routes.push(hello);

    server.set_routes(routes);
    server.serve();
}

pub fn give_response() -> Result<Response, StatusCode> {
        let mut response = Response::default();
        response.set_body("Hello, World!".into());
        Ok(response)
}

```
