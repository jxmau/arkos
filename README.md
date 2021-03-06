# Arkos (v0.1.1 - UNSTABLE )


## What is Arkos

Arkos is a HTTP Server in development inspired by other Web Server libs as axum.rs and warp.rs, also by Spring.  
<br> 

Arkos is experimental, therefore is not stable enough to be used in production. If you want to use a web server library, please check-out [warp](https://github.com/seanmonstar/warp) or [axum](https://github.com/tokio-rs/axum) 

<br>

If you want look around, please check the [startup guide](##) <br>
To know more about changes from the End User POV, please check the [RELEASE.md](/RELEASE.md) <br>
To know more about changes in library inner function , please check the [CHANGES.md](/CHANGES.md) <br>

## Technical Information

### Protocol Version

| Version | Taken in Charge | Note |
| --- | --- | --- |
| HTTP/0.9 | Yes | By taken is charge, it means that it will always send back a 505 StatusCode over a HTTP/1.0 Response. <br> We've got no project to handle this version. |
| HTTP/1.0| Yes | Taken in charge, but not fully compliant to RFC 1945 |
| HTTP/1.1 | Yes | Absolutely not Code Compliant |
| HTTP/2 | No |  |
| HTTP/3 | No |  |

* Version not listed are not taken in charge, obviously.

### Codebase map

```
- src/
    |- core/ : This (pub) module is used by the End User, but is not directly used by server functionment
    |  |- method : HttpMethod 
    |  |- status : StatusCode 
    |  |- content : ContentType
    |  |- cookie : Cookie Struct used by the End User
    |
    |- handler/ : This (priv) module is used by the server Struct - It is used to handle request.
        |- http1 : handle HTTP/1.x requests
    |
    |- server/ : This (pub) module is the server and server direct functionment
    |   |- server : Http Server struct
    |   |- route : Route struct
    |   |- resonse : Response struct, the one that is sent by a request
    |   |- request : Request struct, the one that is parsed upon TcpStream.incoming()
    |   |- cors : CORSHandler struct, Used by the server when redirecting Options request
    |   |- checkpoint : Struct used a field by the Server. Allows the end user to define global filters. Similar to service layer.
    |   |- protocol : Enum used to know which Protocol the request uses and which Protocol to use to respond.
    |
    |- wrapper/ : This (priv) module is used to handle public structs and enums in the server and handlers work flow.
    |   |- checkpoint_manager : Wrapper Struct use to consume Checkpoint structs.
    |   |- cookie_factory : Function to generate Cookie Header
    |   |- request_factory : Create a Request from the raw string receive on the stream
    |   |- response_factory : Factory Struct that generates Response as String to be sent back.
    |
```

## Guide : How to use Arkos

### Set-up a simple Get Request

In this section, we will set up an Arkos server with a GET route to /hello: <br>

1. We start by creating a new server, binding to it the address of 127.0.0.1 on the port 8080 (localhost:8080).
2. As Server use a Vec to store its route, we creating a new one.
3. We create a Route struct with the path /hello and with method GET. 
4. We then set our response by adding the function `give_response()` that we will write later on.
5. We push our Route in our Vec and give it to our server.
6. We can now start it.

```rust
pub fn main() {
    let mut server : Server = Server::new([127, 0, 0, 1], 8080).unwrap();

    let mut routes: Vec<Route> = Vec::new();
    let mut hello = Route::new("/hello", HttpMethod::GET);
    hello.set_response(Arc::new(|_req: Request| {give_response()}));

    routes.push(hello);

    server.set_routes(routes);
    server.serve();
}
```
Now, that it is done, we will create the `give_response()' function: <br>

1. We start by creating a Response struct, by calling `default()`, it will give us a Response with a 200 Status Code, and a ContentType of application/json.
2. We then set the body of the response.
3. And finaly, we return the response.
```rust
    pub fn give_response() -> Result<Response, StatusCode> {
        let mut response = Response::default();
        response.set_body("Hello, World!".into());
        Ok(response)
    }

```

### Use the Request in your response logic.

Now, we will use the HttpRequest is our response logic: we will use a url param without requiring it. To do so, we will reuse our code from before.

We will change our response instruction of the hello route, and use the Request param.

```rust
    hello.set_response(Arc::new(|req: Request| {give_response(req)}));
```

Now, let's change our response function to take the url param 'name':
1. We change our function header by adding the Request param `request`.
2. We then write a simple match statement: If there's a name variable in the path, we will write salute the person, if not, we will salute the world. 
```rust
    pub fn give_response(request: Request) -> Result<Response, StatusCode> {
        let mut response = Response::default();

        let body = match request.param.get("name") {
        Some(name) => format!("Hello, {}!", name),
            None => "Hello, World!".into(),
        }
        response.set_body(body);
        Ok(response)
    }
```

#### What if you want the name param to be required?

If you want to do, that you could add logic to handle its absence in the None arm of our previous match arm, or, you can declare 'name' as a required url param at the route level.

```rust
    hello.add_required_url_param("name");
```

After parsing the request, the Server will compare the Http Request to our route: in short, if a required param is missing, Arkos will respond with a 400 BAD REQUEST without calling your give_response(). <br>
You can also do the sake for a required cookie or a required header. 


### Why using a Result<Response, StatusCode>?

We saw previously how Arkos could generate a response for you if a required param, cookie or header is missing. But you could do the same without declaring a required param:

```rust
    pub fn give_response(request: Request) -> Result<Response, StatusCode> {
        let mut response = Response::default();

        let body = match request.headers.get("name") {
            Some(name) => "Hello, {}!", name,
            None => return Err(StatusCode::BadRequest),
        }
        response.set_body(body.into());
        Ok(response)
    }
```
 What will happen is that, when calling the function, Arkos will take the StatusCode and generate a Reponse from it. It allows you to return any StatusCode error in a Result without worrying to create a whole Response for it.

### Set-up a CORSHandler

One of the feature of Arkos is its integrated CORS Handler. We won't talk about what is CORS, but why it is an advantage. If you know what we are talking about, jump the next two paragraphs. <br>


When a HTTP Request is made from the Front End to a different address that the Front-End server, the browser will, before sending the request, an OPTIONS request on the same path and ask for CORS headers to know if the FE dev has the right to call this request. 

<br>
In short, if you have a GET /hello, the browser will send an OPTIONS /hello, read the headers, and if it's ok, will send the GET request. 

To add a CORSHandler to our server:
1. We will start by calling the new() function. It gives us an empty, activated handler.
2. We start by specifying an origin allowed. We will use the port 4200 for our Front-End server.
3. We will allow four methods to be called by this server: GET POST DELETE PUT
4. We will allow the header "Content-Type"
5. And we will allow that those informations to be cached for a day.
5. We give our handler to our server.

```rust
    let mut cors = CORSHandler::new();
    cors.set_origins(vec!["localhost:4200".into()]);
    cors.set_methods_allowed(vec!(HttpMethod::GET, HttpMethod::POST, HttpMethod::PUT, HttpMethod::DELETE));
    cors.set_headers_allowed(vec!["Content-Type".into()]);
    cors.set_max_age(86400u32);
    server.set_cors_handler(cors);
```


