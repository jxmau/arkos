# Library End User Changes

## v0.1.1

IMPORTANT: Arkos Server will now accept only HTTP/1.0, 1.1. A HTTP/1.0 response with a 505 VersionNotSupported StatusCode.

* Checkpoint Struct:
    * This struct is to be used by the Server struct. It shelters two lists: path on which the struct must be executed, and exempt, which whitelist some path. It also shelters the Function to be executed.

* Server Struct: 
    * Add the checkpoint attribute field
    * Checkpoint(s) will be executed before searching for a corresponding route.

* Route Struct:
    * The struct now has a checks struct, similar to the Server's checkpoints structs, but it's only the function to be executed before executing the Response function.

* Http Method:
    * HEAD is now implemented,
    * HEAD request will be rerouted to find a GET Request on the same path, if none have been declared on said path.

* StatusCode Enum:
    * Multiple StatusCode have now a field:
        * MovedPermanently (301), TemporaryRedirect (307), PermanentRedirect (308): The field is a String that must contain the value of the header Location.
        * SwitchingProtocol (101), UpgradeRequired(427): The field is a String that must contain the value of the Upgrade header.
        * TooManyRequests(429), ServiceUnavailable(503): The field is a String that must contain the value of the RetryAfter header.