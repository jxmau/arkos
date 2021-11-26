Arkos uses a Route struct to define the path.

# Create a simple GET route

This will create a simple GET route that will return a 200 Ok Response.

Note: As a Response is not specified, it will raise a warning in the log.

```ignore

let hello = Route::default("/hello", HTTPMethod::GET); 

```

# Set a Response to our Hello route

This will return a Response with an 200 Ok. 

Note: Calling the default constructor associated function will specified a Content-Type of application/json.

```ignore

let mut hello = Route::default("/hello", HTTPMethod::GET); 
hello.set_response(Arc::new(|_req: Request| {
    let mut response = Reponse::default();
    response.set_body("Hello!".into());
    Ok(response)
} ));

```

# Return an Error

The Response field asked for a function returning a Result<Response, StatusCode>. The reason of that, is to throw an Error if needed. <br>
When calling the Response, Arkos will generated a Response from this StatusCode. <br>
Now let's imagine that we want to filter who has access to this Response, we will check with a if statement, if not allowed, we return a 403 Forbiden Access.

```ignore

let mut hello = Route::default("/hello", HTTPMethod::GET); 
hello.set_response(Arc::new(|req: Request| {

    if !is_allowed(&req) {
        return Err(StatusCode::ForbidenAccess);
    }

    let mut response = Reponse::default();
    response.set_body("Hello!".into());
    Ok(response)
} ));

```

# Extract a url param

Now, we will see how to extract a url parameter and use it. <br> This also works for headers and cookies.

```ignore

let mut hello = Route::default("/hello", HTTPMethod::GET); 
hello.set_response(Arc::new(|req: Request| {

    let body = match request.param.get("name") {
    Some(name) => format!("Hello, {}!", name),
    None => "Hello, World!".into(),
    }

    let mut response = Reponse::default();
    response.set_body("Hello!");
    Ok(response)
} ));

```

# Required a param

Now we want our "name" url param to be required, so we declare our param as required. <br>
When the server will verify the validity of the request, it will check that it has every required fields (url param, cookie or headers). If it doesn't the server will return a 400 Bad Request response.
    
```ignore

let mut hello = Route::default("/hello", HTTPMethod::GET); 
hello.add_required_url_param("name");
// The response declaration.

```