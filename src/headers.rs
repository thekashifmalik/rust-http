use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::collections::hash_map::Entries;
use std::from_str::FromStr;
use std::fmt::{Show, Formatter, FormatError};

use http_url::HttpUrl;


type Header = (String, String);

pub struct Headers {
    data: HashMap<String, String>,
}


impl Headers {

    fn parse_header(string: &str) -> Option<Header> {

        let header_vector: Vec<&str> = string.splitn(1, ':').collect();

        if header_vector.len() != 2 {
            return None;
        }

        Some((
            header_vector[0].trim_left().to_ascii_lower(),
            header_vector[1].trim().to_string(),
        ))
    }

    pub fn from_map(headers_map: HashMap<&str, &str>) -> Headers {
        let mut headers = Headers::new();
        for (&key, &value) in headers_map.iter() {
            headers.insert(key, value);
        }
        headers
    }

    pub fn new() -> Headers {
        Headers {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        let case_insensitive_key = key.to_ascii_lower();
        self.data.get(&case_insensitive_key)
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        let case_insensitive_key = key.to_ascii_lower();
        self.data.insert(case_insensitive_key, value.to_string())
    }

    pub fn len(&self) -> uint {
        self.data.len()
    }

    pub fn iter(&self) -> Entries<String, String> {
        self.data.iter()
    }
}


impl Show for Headers {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatError> {
        for (key, value) in self.iter() {
            try!(write!(fmt, "{}: {}\r\n", key, value));
        }
        Ok(())
    }
}

impl FromStr for Headers {
    fn from_str(string: &str) -> Option<Headers> {

        let mut headers = Headers::new();

        for line in string.trim().lines() {
            if let Some((key, value)) = Headers::parse_header(line) {
                headers.insert(key[], value[]);
            }
        }

        if headers.len() == 0 {
            return None;
        } else {
            return Some(headers)
        }

    }
}



pub fn make_default_headers(http_url: &HttpUrl) -> Headers {
    let mut headers = Headers::new();

    // Add host
    let host = match http_url.port {
        Some(p) => format!("{}:{}", http_url.host, p.to_string()),
        None => format!("{}", http_url.host),
    };
    headers.insert("HOST", host[]);

    // Add Server
    headers.insert("SERVER", "Rust/httpc/0.1");

    headers
}
