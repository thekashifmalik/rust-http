
use headers::{ make_default_headers, Headers};
use serialize::json::Json;

use HTTP_VERSION;
use CRLF;
use STATUS_LINE_SEPERATOR;
use methods;
use status_codes;
use parser;
use http_url::HttpUrl;



#[deriving(Show)]
pub struct Request {
    method: methods::Method,
    path: String,
    headers: Headers
}


impl Request {
    pub fn new(method: methods::Method, http_url: &HttpUrl, headers: Option<&Headers>) -> Request {
        let mut headers_to_send = make_default_headers(http_url);

        if let Some(_headers) = headers {
            for (key, value) in _headers.iter() {
                headers_to_send.insert(key[], value[]);
            }
        }

        Request {
            headers: headers_to_send,
            method: method,
            path: http_url.path[].to_string(),
        }
    }


    pub fn to_bytes(&self) -> Vec<u8> {
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



#[deriving(Show)]
pub struct Response {
    pub url: String,
    pub status_code: status_codes::StatusCode,
    pub text: String,
    pub headers: Headers,
    // pub elapsed: f64,
}

impl Response {

    pub fn from_bytes(http_url: &HttpUrl, msg: parser::HttpMessage) -> Option<Response> {
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
