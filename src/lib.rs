#![feature(macro_rules, struct_variant)]

extern crate serialize;
extern crate time;

use std::from_str::{ from_str, FromStr };
use std::str::from_utf8;

pub use headers::{ Header, Headers };
use transport::{ HttpMessageBytes, Transaction };
use self::serialize::json::Json;
use self::http_url::HttpUrl;

pub mod methods;
pub mod status_codes;
mod headers;
mod transport;
mod macros;
mod http_url;
mod parser;



static HTTP_VERSION: &'static str = "1.0";
static DEFAULT_HTTP_PORT: u16 = 80;
static DEFAULT_HTTPS_PORT: u16 = 443;



#[deriving(Show)]
pub struct Response {
    pub url: String,
    pub status_code: status_codes::StatusCode,
    pub text: String,
    pub headers: Headers,
    // pub elapsed: f64,
}

impl Response {

    fn from_bytes(http_url: &HttpUrl, msg: HttpMessageBytes) -> Option<Response> {
        optional!(parser::parse_response_header(msg.header.clone()));

        // StatusLine {
        //     http_version: String,
        //     status_code: status_codes::StatusCode,
        //     status_code_val: uint,
        //     reason_phrase: String,
        // }

        let status_line: StatusLine = from_str(from_utf8(msg.header.start_line.as_slice()).expect("error parsing status line!")).expect("error parsing status line!");

        let header_string = match String::from_utf8(msg.header.headers) {
            Ok(x) => x,
            Err(_) => return None,
        };

        let headers: Headers = match from_str(header_string.as_slice()) {
            Some(x) => x,
            None => return None,
        };

        Some(Response {
            url: http_url.url.clone(),
            status_code: status_line.status_code,
            text: String::from_utf8(msg.body).unwrap(),
            headers: headers,
        })


    }

    pub fn json(&self) -> Option<Json> {
        let raw_json = "{\"e\":2.71,\"pi\":3.14}";
        from_str(raw_json)
    }
}

#[deriving(Show)]
struct RequestLine {
    method: methods::Method,
    path: String,
}


#[deriving(Show)]
struct StatusLine {
    http_version: String,
    status_code: status_codes::StatusCode,
    status_code_val: uint,
    reason_phrase: String,
}

impl FromStr for StatusLine {
    fn from_str(string: &str) -> Option<StatusLine> {
        let status_vector: Vec<&str> = string.trim().splitn(2, ' ').collect();

        let http_version = status_vector[0].slice_from(5).to_string();
        let status_code_val: uint = from_str(status_vector[1]).unwrap();
        let reason_phrase = status_vector[2].to_string();

        Some(StatusLine{
            http_version: http_version,
            status_code: status_codes::OK,
            status_code_val: status_code_val,
            reason_phrase: reason_phrase,
        })
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
            path: http_url.path.as_slice().to_string(),
        }
    }


    fn to_bytes(&self) -> Vec<u8> {
        let string = format!(
            "{} {} HTTP/{}\n{}\n",
            self.method,
            self.path,
            HTTP_VERSION,
            self.headers,
        );
        // println!("Sent:\n{}", string);
        return string.into_bytes()
    }

}


fn make_request(method: methods::Method, http_url: HttpUrl) -> Result<Response, ()>{
    let request = Request::new(method, &http_url);

    let port = match http_url.port {
        Some(x) => x,
        None => if http_url.scheme.as_slice() == "https" { DEFAULT_HTTPS_PORT } else { DEFAULT_HTTP_PORT },
    };

    let mut client = try!(Transaction::new(http_url.host.as_slice(), port).map_err(|_| {()}));

    // Send data
    let payload = request.to_bytes();
    let start = time::precise_time_s();
    client.write(payload.as_slice());

    // Read data
    let msg_bytes = client.read().ok().expect("dammit");
    let elapsed = time::precise_time_s() - start;
    let response = match Response::from_bytes(&http_url, msg_bytes) {
        Some(a) => Ok(a),
        None => Err(()),
    };

    println!("Took: {}s", elapsed);
    response
}



pub fn get(url_string: &str) -> Result<Response, ()> {
    let http_url = try!(HttpUrl::from_str(url_string));
    make_request(methods::GET, http_url)
}

pub fn post(url_string: &str) -> Result<Response, ()> {
    let http_url = try!(HttpUrl::from_str(url_string));
    make_request(methods::POST, http_url)
}
