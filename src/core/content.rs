


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentType {
    Json,
    Text,
    Custom(String)
}

impl ContentType {
    pub fn get(&self) -> String {
        let msg = match self {
            ContentType::Json => "application/json",
            ContentType::Text => "text/plain",
            ContentType::Custom(s) => s,
            // _ => "text/plain",
        };
        msg.into()
    }
}