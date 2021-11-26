Send back cookie.

Arkos can handle for the moment only HTTP/1.1, therefore, only one Set-Cookie Header can be sent back for the moment.

# Create a cookie

```ìgnore

let cookie = Cookie::new("Name".into(), "Value".into());

>> Will send back the Header "Set-Cookie: Name=Value"

```

# Verification logic