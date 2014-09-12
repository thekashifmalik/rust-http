use std::from_str::FromStr;
use std::fmt::{Show, Formatter, FormatError};


pub struct Header {
    pub key: String,
    pub value: String,
}

impl Show for Header {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        write!(fmt, "{}: {}", self.key, self.value)
    }
}

impl FromStr for Header {
    fn from_str(string: &str) -> Option<Header> {

        let header_vector: Vec<&str> = string.splitn(1, ':').collect();

        if header_vector.len() != 2 {
            return None;
        }

        Some(Header {
            key: header_vector[0].trim_left().to_string(),
            value: header_vector[1].trim().to_string(),
        })
    }
}

pub struct Headers(pub Vec<Header>);

impl Show for Headers {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        let Headers(ref vector) = *self;
        for header in vector.iter() {
            try!(write!(fmt, "{}\n", *header));
        }
        Ok(())
    }
}

impl FromStr for Headers {
    fn from_str(string: &str) -> Option<Headers> {

        let header_string_vector: Vec<&str> = string.trim().lines().collect();

        if header_string_vector.len() == 0 {
            return None;
        }
        let headers_vector: Vec<Header> = header_string_vector.iter().filter_map(|&x| { from_str(x) }).collect();

        Some(Headers(headers_vector))
    }
}

