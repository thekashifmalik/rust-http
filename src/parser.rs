
use std::from_str::{ from_str, FromStr };
use std::str::from_utf8;
use super::headers::Headers;
use super::transport::{HttpHeaderBytes};
use std::slice::ImmutableSlice;

#[deriving(Show)]
enum StartLine {

    RequestLine {
        method: Vec<u8>,
        path: Vec<u8>,
        http_version: Vec<u8>,
    },

    RawStatusLine {
        status_code: Vec<u8>,
        reason_phrase: Vec<u8>,
        http_version: Vec<u8>,
    },
}

#[deriving(Show)]
struct HttpHeader {
    start_line: StartLine,
    headers: Headers,
}

struct HttpMessage {
    header: HttpHeader,
    body: Vec<u8>,
}


pub fn parse_response_header(header: HttpHeaderBytes) -> Option<HttpHeader> {
    let is_space_byte = | byte: &u8 | { if *byte == ' ' as u8 { true } else { false } };
    let status_vector: Vec<&[u8]> = header.start_line.as_slice().splitn(2, is_space_byte).collect();

    if status_vector.len() != 3 {
        return None;
    }

    let status_line = RawStatusLine {
        status_code: Vec::from_slice(status_vector[1]),
        reason_phrase: Vec::from_slice(status_vector[2]),
        http_version: Vec::from_slice(status_vector[0]),
    };

    let headers: Headers = optional!(from_str(optional!(from_utf8(header.headers.as_slice()))));

    Some(HttpHeader {
        start_line: status_line,
        headers: headers,
    })
}
