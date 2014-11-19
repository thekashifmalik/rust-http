
use std::from_str::from_str;
use std::str::from_utf8;
use super::headers::Headers;
use transactions::HttpMessageBytes;


#[deriving(Show)]
pub struct StartLine(pub Vec<u8>, pub Vec<u8>, pub Vec<u8>);

#[deriving(Show)]
pub struct HttpHeader {
    pub start_line: StartLine,
    pub headers: Headers,
}

#[deriving(Show)]
pub struct HttpMessage {
    pub header: HttpHeader,
    pub body: Vec<u8>,
}


pub fn parse_header(message_bytes: &HttpMessageBytes) -> Option<HttpHeader> {
    let is_space_byte = | byte: &u8 | { if *byte == ' ' as u8 { true } else { false } };
    let status_vector: Vec<&[u8]> = message_bytes.get_start_line_bytes().splitn(2, is_space_byte).collect();

    if status_vector.len() != 3 {
        return None;
    }

    let status_line = StartLine(
        status_vector[1].to_vec(),
        status_vector[2].to_vec(),
        status_vector[0].to_vec(),
    );

    let headers: Headers = optional!(from_str(optional!(from_utf8(message_bytes.get_headers_bytes()))));

    Some(HttpHeader {
        start_line: status_line,
        headers: headers,
    })
}

pub fn parse_response(message_bytes: HttpMessageBytes) -> Option<HttpMessage> {
    Some(HttpMessage {
        header: optional!(parse_header(&message_bytes)),
        body: message_bytes.get_body_bytes().to_vec(),
    })
}
