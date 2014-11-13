#![feature(macro_rules, struct_variant, slicing_syntax, phase)]

extern crate serialize;
extern crate time;

use std::from_str::from_str;
use std::str::from_utf8;

pub use headers::{ Header, Headers };
use transport::Connection;
use self::serialize::json::Json;
use self::http_url::HttpUrl;

pub mod methods;
pub mod status_codes;
mod headers;
mod transport;
mod macros;
mod http_url;
mod parser;



const HTTP_VERSION: &'static [u8] = b"HTTP/1.0";

const DEFAULT_HTTP_PORT: u16 = 80;
const DEFAULT_HTTPS_PORT: u16 = 443;
const CRLF: &'static [u8] = b"\r\n";
const STATUS_LINE_SEPERATOR: &'static [u8] = b" ";


#[deriving(Show)]
pub struct Response {
    pub url: String,
    pub status_code: status_codes::StatusCode,
    pub text: String,
    pub headers: Headers,
    // pub elapsed: f64,
}

impl Response {

    fn from_bytes(http_url: &HttpUrl, msg: parser::HttpMessage) -> Option<Response> {
        let status_code = optional!(status_codes::StatusCode::from_bytes(msg.header.start_line.0[]));

        Some(Response {
            url: http_url.url.to_string(),
            status_code: status_code,
            text: String::from_utf8(msg.body).unwrap(),
            headers: msg.header.headers,
        })


    }

    pub fn json(&self) -> Option<Json> {
        let raw_json = "{\"e\":2.71,\"pi\":3.14}";
        from_str(raw_json)
    }
}


fn make_default_headers(http_url: &HttpUrl) -> Headers {
    let port = match http_url.port {
        Some(p) => p.to_string(),
        None => "".to_string(),
    };

    let host = format!("{}{}", http_url.host, port);

    Headers::from_vector(
        vec![
            Header {key: "HOST".to_string(), value: host},
        ]
    )

}


#[deriving(Show)]
struct Request {
    method: methods::Method,
    path: String,
    headers: Headers
}


impl Request {
    fn new(method: methods::Method, http_url: &HttpUrl) -> Request {
        let headers = make_default_headers(http_url);
        Request {
            headers: headers,
            method: method,
            path: http_url.path[].to_string(),
        }
    }


    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push_all(self.method.to_bytes());
        buffer.push_all(STATUS_LINE_SEPERATOR);
        buffer.push_all(self.path.clone().into_bytes()[]);
        buffer.push_all(STATUS_LINE_SEPERATOR);
        buffer.push_all(HTTP_VERSION);
        buffer.push_all(CRLF);
        buffer.push_all(self.headers.to_string().into_bytes()[]);
        buffer.push_all(CRLF);

        // println!("Sent:\n{}", buffer.len());
        return buffer
    }

}


fn make_request(method: methods::Method, address: &str) -> Result<Response, ()>{

    let http_url = try!(HttpUrl::from_str(address));
    let request = Request::new(method, &http_url);

    let port = match http_url.port {
        Some(x) => x,
        None => if http_url.scheme.as_slice() == "https" { DEFAULT_HTTPS_PORT } else { DEFAULT_HTTP_PORT },
    };

    let mut client = try!(Connection::new((http_url.host[], port)).map_err(|_| {()}));

    // Send data
    let payload = request.to_bytes();
    let start = time::precise_time_s();
    try!(client.write(payload[]).map_err(|_| {()}));

    // Read data
    let msg_bytes = optional_try!(client.read().ok());
    let elapsed = time::precise_time_s() - start;

    // Parse
    let http_msg = optional_try!(parser::parse_response(msg_bytes));

    // Create response
    let response = match Response::from_bytes(&http_url, http_msg) {
        Some(a) => Ok(a),
        None => Err(()),
    };

    println!("Took: {}s", elapsed);
    response
}



pub fn get(address: &str) -> Result<Response, ()> {
    make_request(methods::GET, address)
}

pub fn post(address: &str) -> Result<Response, ()> {
    make_request(methods::POST, address)
}
