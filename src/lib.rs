#![feature(macro_rules, struct_variant, slicing_syntax, phase, if_let, tuple_indexing)]

extern crate serialize;
extern crate time;

use std::from_str::from_str;

pub use headers::Headers;
use transactions::Transaction;
use http_url::HttpUrl;
use messages::{ Request, Response };
use connections::Connection;

pub mod methods;
pub mod status_codes;
mod headers;
mod readers;
mod connections;
mod transactions;
mod macros;
mod http_url;
mod parser;
mod messages;


const HTTP_VERSION: &'static [u8] = b"HTTP/1.0";
const CRLF: &'static [u8] = b"\r\n";
const STATUS_LINE_SEPERATOR: &'static [u8] = b" ";

const DEFAULT_HTTP_PORT: u16 = 80;
const DEFAULT_HTTPS_PORT: u16 = 443;



fn request(method: methods::Method, address: &str, headers: Option<&Headers>) -> Result<Response, ()>{
    // Parse URL
    let http_url = try!(HttpUrl::from_str(address));
    let request = Request::new(method, &http_url, headers);

    // Extract or infer port
    let port = match http_url.port {
        Some(x) => x,
        None => if http_url.scheme.as_slice() == "https" { DEFAULT_HTTPS_PORT } else { DEFAULT_HTTP_PORT },
    };
    let connection = try!(Connection::new((http_url.host[], port)).map_err(|_| {()}));
    let mut session = try!(Transaction::new(connection).map_err(|_| {()}));

    // session::make_request(method, headers,)

    // Send data
    let payload = request.to_bytes();
    let start = time::precise_time_s();
    try!(session.write(payload[]).map_err(|_| {()}));

    // Read data
    let msg_bytes = optional_try!(session.read().ok());
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



pub fn get(address: &str, headers: Option<&Headers>) -> Result<Response, ()> {
    request(methods::GET, address, headers)
}

pub fn post(address: &str, headers: Option<&Headers>) -> Result<Response, ()> {
    request(methods::POST, address, headers)
}
