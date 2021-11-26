pub trait HttpStatusCode {
    fn get_code(&self) -> u16;
    fn get_title(&self) -> String;
}

// Rust, just implement Java's enum for crying out loud

#[derive( Clone, Debug, PartialEq, Eq, Hash)]
pub enum StatusCode {
    Continue, SwitchingProtocols, Processing, EarlyHints,
    Ok, Created, Accepted, NonAuthoritativeInformations, NoContent, ResetContent, PartialContent, MultiStatus, AlreadyReported, ContentDifferent, IMUsed,
    MultipleChoices, MovedPermanently, Found, SeeOther, NotModified, UseProxy, SwitchProxy, TemporaryRedirect, PermanentRedirect,
    BadRequest, Unauthorized, PaymentRequired, Forbidden, NotFound, MethodNotAllowed, NotAcceptable, ProxyAuthenticationRequired, RequestTimeout, Conflict, Gone, LengthRequired, PreconditionFailed, PayloadTooLarge, URIToolLong, UnsupportedMediaType, RangeNotSatisfiable, ExpectationFailed, ImATeaPot, MisdirectionRequest, UnprocessableEntity, Locked, FailedDependency, TooEarly, UpgradeRequired, PreconditionRequired, TooManyRequests, RequestHeaderFieldsTooLarge, UnavailableForLegalReasons,
    InternalServerError, NotImplemented, BadGateway, ServiceUnavailable, GatewayTimeout, HTTPVersionNotSupported, VariantAlsoNegotiates, InsufficientStorage, LoopDetected, NotExtended, NetworkAuthenticationRequired,
    Custom(u16, String),
}

impl HttpStatusCode for StatusCode {
    
    fn get_code(&self) -> u16{
        match self {
            StatusCode::Continue => 100, 
            StatusCode::SwitchingProtocols => 101, 
            StatusCode::Processing => 102, 
            StatusCode::EarlyHints => 103,
            StatusCode::Ok => 200, 
            StatusCode::Created => 201, 
            StatusCode::Accepted => 202, 
            StatusCode::NonAuthoritativeInformations => 203, 
            StatusCode::NoContent => 204, 
            StatusCode::ResetContent => 205, 
            StatusCode::PartialContent => 206, 
            StatusCode::MultiStatus => 207, 
            StatusCode::AlreadyReported => 208 , 
            StatusCode::ContentDifferent => 210, 
            StatusCode::IMUsed => 226,
            StatusCode::MultipleChoices => 300, 
            StatusCode::MovedPermanently => 301, 
            StatusCode::Found => 302, 
            StatusCode::SeeOther => 303, 
            StatusCode::NotModified => 304, 
            StatusCode::UseProxy => 305, 
            StatusCode::SwitchProxy => 306, 
            StatusCode::TemporaryRedirect => 307, 
            StatusCode::PermanentRedirect => 308,
            StatusCode::BadRequest => 400, 
            StatusCode::Unauthorized => 401, 
            StatusCode::PaymentRequired => 402, 
            StatusCode::Forbidden => 403, 
            StatusCode::NotFound => 404, 
            StatusCode::MethodNotAllowed => 405, 
            StatusCode::NotAcceptable => 406, 
            StatusCode::ProxyAuthenticationRequired => 407, 
            StatusCode::RequestTimeout => 408, 
            StatusCode::Conflict => 409, 
            StatusCode::Gone => 410, 
            StatusCode::LengthRequired => 411, 
            StatusCode::PreconditionFailed => 412, 
            StatusCode::PayloadTooLarge => 413, 
            StatusCode::URIToolLong => 414, 
            StatusCode::UnsupportedMediaType => 415, 
            StatusCode::RangeNotSatisfiable => 416, 
            StatusCode::ExpectationFailed => 417, 
            StatusCode::ImATeaPot => 418, 
            StatusCode::MisdirectionRequest => 421, 
            StatusCode::UnprocessableEntity => 422, 
            StatusCode::Locked => 423, 
            StatusCode::FailedDependency => 424, 
            StatusCode::TooEarly => 425, 
            StatusCode::UpgradeRequired => 426, 
            StatusCode::PreconditionRequired => 428, 
            StatusCode::TooManyRequests => 429, 
            StatusCode::RequestHeaderFieldsTooLarge => 431, 
            StatusCode::UnavailableForLegalReasons => 451,
            StatusCode::InternalServerError => 500, 
            StatusCode::NotImplemented => 501, 
            StatusCode::BadGateway => 502, 
            StatusCode::ServiceUnavailable => 503, 
            StatusCode::GatewayTimeout => 504, 
            StatusCode::HTTPVersionNotSupported => 505, 
            StatusCode::VariantAlsoNegotiates => 506, 
            StatusCode::InsufficientStorage => 507, 
            StatusCode::LoopDetected => 508, 
            StatusCode::NotExtended => 510, 
            StatusCode::NetworkAuthenticationRequired => 511,
            StatusCode::Custom(code, _) => *code,
        }
    }

    fn get_title(&self) -> String {
        let msg = match &self {
            StatusCode::Continue => "Continue", 
            StatusCode::SwitchingProtocols => "Switching Protocol", 
            StatusCode::Processing => "Processing", 
            StatusCode::EarlyHints => "Early Hints",
            StatusCode::Ok => "Ok", 
            StatusCode::Created => "Created", 
            StatusCode::Accepted => "Accepted", 
            StatusCode::NonAuthoritativeInformations => "Non Authoritative Informations", 
            StatusCode::NoContent => "No Content", 
            StatusCode::ResetContent => "Reset Content", 
            StatusCode::PartialContent => "Partial Content", 
            StatusCode::MultiStatus => "Multi Status", 
            StatusCode::AlreadyReported => "Already Reported" , 
            StatusCode::ContentDifferent => "Content Different", 
            StatusCode::IMUsed => "IM Used",
            StatusCode::MultipleChoices => "Multiple Choices", 
            StatusCode::MovedPermanently => "Moved Permanently", 
            StatusCode::Found => "Found", 
            StatusCode::SeeOther => "See Other", 
            StatusCode::NotModified => "Not Modified", 
            StatusCode::UseProxy => "Use Proxy", 
            StatusCode::SwitchProxy => "Switch Proxy", 
            StatusCode::TemporaryRedirect => "Temporary Redirect", 
            StatusCode::PermanentRedirect => "Permanent Redirect",
            StatusCode::BadRequest => "Bad Redirect", 
            StatusCode::Unauthorized => "Unauthorized", 
            StatusCode::PaymentRequired => "Payment Required", 
            StatusCode::Forbidden => "Forbidden", 
            StatusCode::NotFound => "Not Found", 
            StatusCode::MethodNotAllowed => "Method Not Allowed", 
            StatusCode::NotAcceptable => "Not Acceptable", 
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required", 
            StatusCode::RequestTimeout => "Request Timeout", 
            StatusCode::Conflict => "Conflict", 
            StatusCode::Gone => "Gone", 
            StatusCode::LengthRequired => "Length Required", 
            StatusCode::PreconditionFailed => "Precondition Failed", 
            StatusCode::PayloadTooLarge => "Payload Too Large", 
            StatusCode::URIToolLong => "URI Too Long", 
            StatusCode::UnsupportedMediaType => "Unsupported Media Type", 
            StatusCode::RangeNotSatisfiable => "Range Not Satisfiable", 
            StatusCode::ExpectationFailed => "Expectation Failed", 
            StatusCode::ImATeaPot => "I'm A Tea Pot", 
            StatusCode::MisdirectionRequest => "Misdirection Request", 
            StatusCode::UnprocessableEntity => "Unprocessable Entity", 
            StatusCode::Locked => "Locked", 
            StatusCode::FailedDependency => "Failed Dependency", 
            StatusCode::TooEarly => "Too Early", 
            StatusCode::UpgradeRequired => "Upgrade Required", 
            StatusCode::PreconditionRequired => "Precondition Required", 
            StatusCode::TooManyRequests => "Too Many Requests", 
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large", 
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            StatusCode::InternalServerError => "Internal Server Error", 
            StatusCode::NotImplemented => "Not Implemented", 
            StatusCode::BadGateway => "Bad Gateway", 
            StatusCode::ServiceUnavailable => "Service Unavailable", 
            StatusCode::GatewayTimeout => "Gateway Timeout", 
            StatusCode::HTTPVersionNotSupported => "HTTP Version Not Supported", 
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates", 
            StatusCode::InsufficientStorage => "Insufficient Storage", 
            StatusCode::LoopDetected => "Loop Detected", 
            StatusCode::NotExtended => "Not Extended", 
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
            StatusCode::Custom(_, title) => title,
        };
        msg.to_string()
    }


}

impl StatusCode {

    pub fn from_str(val: &str) -> StatusCode {
        match val {
            "100" => StatusCode::Ok, 
            "101" => StatusCode::SwitchingProtocols, 
            "102" => StatusCode::Processing , 
            "103" => StatusCode::EarlyHints,
            "200" => StatusCode::Ok , 
            "201" => StatusCode::Created  , 
            "202" => StatusCode::Accepted  , 
            "203" => StatusCode::NonAuthoritativeInformations  , 
            "204" => StatusCode::NoContent  , 
            "205" => StatusCode::ResetContent  , 
            "206" => StatusCode::PartialContent  , 
            "207" => StatusCode::MultiStatus  , 
            "208" => StatusCode::AlreadyReported  , 
            "210" => StatusCode::ContentDifferent  , 
            "226" => StatusCode::IMUsed,
            "300" => StatusCode::MultipleChoices  , 
            "301" => StatusCode::MovedPermanently , 
            "302" => StatusCode::Found  , 
            "303" => StatusCode::SeeOther  , 
            "304" => StatusCode::NotModified  , 
            "305" => StatusCode::UseProxy  , 
            "306" => StatusCode::SwitchProxy  , 
            "307" => StatusCode::TemporaryRedirect  , 
            "308" => StatusCode::PermanentRedirect,
            "400" => StatusCode::BadRequest  , 
            "401" => StatusCode::Unauthorized  , 
            "402" => StatusCode::PaymentRequired  , 
            "403" => StatusCode::Forbidden  , 
            "404" => StatusCode::NotFound  , 
            "405" => StatusCode::MethodNotAllowed  , 
            "406" => StatusCode::NotAcceptable  , 
            "407" => StatusCode::ProxyAuthenticationRequired  , 
            "408" => StatusCode::RequestTimeout  , 
            "409" => StatusCode::Conflict  , 
            "410" => StatusCode::Gone  , 
            "411" => StatusCode::LengthRequired  , 
            "412" => StatusCode::PreconditionFailed  , 
            "413" => StatusCode::PayloadTooLarge  , 
            "414" => StatusCode::URIToolLong  , 
            "415" => StatusCode::UnsupportedMediaType  , 
            "416" => StatusCode::RangeNotSatisfiable  , 
            "417" => StatusCode::ExpectationFailed  , 
            "418" => StatusCode::ImATeaPot  , 
            "421" => StatusCode::MisdirectionRequest  , 
            "422" => StatusCode::UnprocessableEntity  , 
            "423" => StatusCode::Locked  , 
            "424" => StatusCode::FailedDependency  , 
            "425" => StatusCode::TooEarly  , 
            "426" => StatusCode::UpgradeRequired  , 
            "428" => StatusCode::PreconditionRequired  , 
            "429" => StatusCode::TooManyRequests  , 
            "431" => StatusCode::RequestHeaderFieldsTooLarge  , 
            "451" => StatusCode::UnavailableForLegalReasons,
            "501" => StatusCode::InternalServerError  , 
            "502" => StatusCode::NotImplemented  , 
            "503" => StatusCode::BadGateway  , 
            "504" => StatusCode::ServiceUnavailable  , 
            "505" => StatusCode::GatewayTimeout  , 
            "506" => StatusCode::HTTPVersionNotSupported  , 
            "507" => StatusCode::VariantAlsoNegotiates  , 
            "508" => StatusCode::InsufficientStorage  , 
            "509" => StatusCode::LoopDetected  , 
            "510" => StatusCode::NotExtended  , 
            "511" => StatusCode::NetworkAuthenticationRequired  ,
            _ => StatusCode::InternalServerError,
        }
    }
}