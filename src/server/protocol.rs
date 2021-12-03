use log::trace;
use std::str::FromStr;
use crate::core::status::StatusCode;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol{
    Http1(u8), // HTTP/1.1
    Error,
}   

impl Protocol {

    pub fn parse_from_raw(raw: &str) -> Result<Protocol, StatusCode> {
        let row = raw.split("\r\n").next().unwrap();
        let end = row.split(' ').last();
        Self::parse_from_str(end.unwrap())
    }

    pub fn parse_from_str(s: &str) -> Result<Self, StatusCode> {
        
        let mut version_iterator = s.split('.');                                   // We strip the subversion of the version protocol by spliting at '.': HTTP/1.1
        let version = version_iterator.next().ok_or(StatusCode::BadRequest)?;      // We take the first item as our version: HTTP/1
        let subversion = version_iterator.next().ok_or(StatusCode::BadRequest)?;   // We take the second item as our sub-verion: 1

        let v = match  u8::from_str(subversion) {
            Ok(v) => v,
            Err(_) => {
                trace!("The server failed to parse the sub-version - HTTP param received: {}", s);
                return Err(StatusCode::BadRequest);
            }
        };

        let protocol = match version {
            "HTTP/1" => Protocol::Http1(v),
            _ => Self::Error,
        };

        match protocol.is_taken_in_charge() {
            true => Ok(protocol),
            false => Err(StatusCode::HTTPVersionNotSupported),
        }
    }

    pub fn is_taken_in_charge(&self) -> bool {
        match &self {
            Self::Http1(v) => v <= &1,
            _ => false,
        }
    }

    pub fn since(&self, supported_from: u8) -> bool {
        match &self {
            Self::Http1(v) => v >= &supported_from,
            _ => false,
        }
    }
    
    pub fn get_version(&self) -> &u8 {
        match self {
            Self::Http1(v) => v,
            _ => &0,
        }
    }

}





impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            Self::Http1(v) => format!("HTTP/1.{}", v),
            _ => "Error".into(),
        }
    }
}

// impl FromStr for Protocol {
//     type Err = Infallible;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let r = match s {
//             "HTTP/1.1" => Protocol::Http11,
//             _ => return Err(_),
//         };
//         Ok(r)
//     }

// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parse_http11() {
        assert_eq!(Protocol::Http1(1), Protocol::parse_from_raw("GET /hello HTTP/1.1\r\n Header : Blablabla").unwrap());
    }

}