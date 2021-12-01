
#[derive(Debug, PartialEq, Eq)]
pub enum Protocol{
    Http11, // HTTP/1.1
    Error,
}   

impl Protocol {

    pub fn parse_from_raw(raw: &str) -> Self {
        let row = raw.split("\r\n").next().unwrap();
        let end = row.split(' ').last();
        Self::from_str(end.unwrap())
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Protocol::Http11,
            _ => Self::Error,
        }
    }


}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        let s = match self {
            Self::Http11 => "HTTP/1.1",
            _ => "Error"
        };
        s.into()
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
        assert_eq!(Protocol::Http11, Protocol::parse_from_raw("GET /hello HTTP/1.1\r\n Header : Blablabla"));
    }

}