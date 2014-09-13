extern crate url;
extern crate serialize;


use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::{BufferedStream, TcpStream};
use std::from_str::{from_str, FromStr};
use std::fmt::{Show, Formatter, FormatError};
use std::from_str::;

use std::io::{IoResult, IoError};
use std::io::Stream;

use self::serialize::json::Json;


use self::headers::{Header, Headers};
use self::url::Url;


mod methods;
mod headers;



static HTTP_VERSION: &'static str = "1.1";
static CLRF: &'static str = "\r\n";
static DEFAULT_HTTP_PORT: u16 = 80;
static DEFAULT_HTTPS_PORT: u16 = 443;


enum StartLine {
    RequestLine,
    StatusLine,
}

#[deriving(Show)]
struct Response {
    pub status_code: uint,
    pub text: String,
    pub headers: Headers,
}

impl Response {
    fn new(mut stream: BufferedStream<TcpStream>) -> Result<Response, ()> {
        let first_line = stream.read_line().ok().expect("error reading from stream!");
        let status_line: StatusLine = from_str(first_line.as_slice()).expect("error parsing status line!");

        // println!("{}", status_line);

        let header_bytes = read_to_headers(&mut stream).unwrap();

        let header_string = match String::from_utf8(header_bytes) {
            Ok(x) => x,
            Err(_) => return Err(()),
        };

        let headers: Headers = match from_str(header_string.as_slice()) {
            Some(x) => x,
            None => return Err(()),
        };

        // println!("{}", headers.vector[0]);
        // println!("{}", headers.vector[1]);
        // println!("{}", headers.vector[2]);

        let text_bytes = stream.read_to_end().unwrap();

        Ok(Response {
            status_code: status_line.status_code,
            text: String::from_utf8(text_bytes).unwrap(),
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
    status_code: uint,
    reason_phrase: String,
}

impl RequestLine {
    fn to_string(&self) -> String {
        format!(
            "{} {} HTTP/{}{}",
            self.method.to_string(),
            self.path,
            HTTP_VERSION,
            CLRF,
        )
        // return string.into_bytes()

    }
}

impl FromStr for StatusLine {
    fn from_str(string: &str) -> Option<StatusLine> {
        let status_vector: Vec<&str> = string.trim().splitn(2, ' ').collect();

        let http_version = status_vector[0].slice_from(5).to_string();
        let status_code: uint = from_str(status_vector[1]).unwrap();
        let reason_phrase = status_vector[2].to_string();

        Some(StatusLine{
            http_version: http_version,
            status_code: status_code,
            reason_phrase: reason_phrase,
        })
    }
}

#[deriving(Show)]
struct Address {
    host: String,
    port: u16,
}


#[deriving(Show)]
struct HttpUrl {
    scheme: String,
    domain: String,
    path: String,
    port: Option<u16>,
}

impl HttpUrl {
    fn from_url(url: &Url) -> Result<HttpUrl, ()> {
        // Check scheme
        if url.scheme.as_slice() != "http" && url.scheme.as_slice() != "https" {
            return Err(());
        }

        // Check domain
        let domain = match url.domain() {
            Some(x) => (x),
            None => return Err(()),
        };

        // Create path
        let path = match url.path() {
            Some(path_vector) => format!("/{}", path_vector.connect("/")),
            None => "/".to_string(),
        };


        return Ok(HttpUrl{
            scheme: url.scheme.clone(),
            domain: domain.to_string(),
            path: path,
            port: url.port(),
        })
    }
}

impl Address {
    fn new(http_url: &HttpUrl) -> Address {

        let port = match http_url.port {
            Some(x) => x,
            None => if http_url.scheme.as_slice() == "https" { DEFAULT_HTTPS_PORT } else { DEFAULT_HTTP_PORT },
        };

        Address {
            host: http_url.domain.to_string(),
            port: port,
        }
    }
}




#[deriving(Show)]
struct Request {
//     address: Address,
//     request_line: RequestLine,
    method: methods::Method,
    path: String,
    headers: Headers
}


fn make_default_headers(http_url: &HttpUrl) -> Headers {
    let port = match http_url.port {
        Some(p) => p.to_string(),
        None => "".to_string(),
    };

    let host = format!("{}{}", http_url.domain, port);

    Headers::from_vector(
        vec![
            Header {key: "HOST".to_string(), value: host},
        ]
    )

}

impl Request {
    fn new(method: methods::Method, path: String, headers: Headers) -> Result<Request, ()> {
        // let request_line = RequestLine;

        Ok(Request {
            headers: headers,
            method: method,
            path: path,
        })
    }


    fn to_bytes(&self) -> Vec<u8> {
        let string = format!(
            "{} {} HTTP/1.0\n{}\n",
            self.method,
            self.path,
            self.headers,
        );
        println!("Sent:\n{}EOF", string);
        return string.into_bytes()
    }

    fn send(&self, stream: &mut BufferedStream<TcpStream>) {
        stream.write(self.to_bytes().as_slice()).ok().expect("Error writing to socket!");
        stream.flush().ok().expect("Error writing to socket!");
    }
}


fn read_to_crlf(stream: &mut BufferedStream<TcpStream>) -> IoResult<Vec<u8>> {
    let mut read_upto = try!(stream.read_until('\r' as u8));

    let next_char = try!(stream.read_byte());
    read_upto.push(next_char);

    if next_char  == '\n' as u8 {
        return Ok(read_upto);

    } else {
        return Ok(read_upto.append(try!(read_to_crlf(stream)).as_slice()));
    }
}


fn read_to_headers(stream: &mut BufferedStream<TcpStream>) -> IoResult<Vec<u8>> {
    let read_upto = try!(read_to_crlf(stream));

    if read_upto.as_slice() == CLRF.as_bytes() {
        return Ok(read_upto);
    }

    let next_char = try!(read_to_crlf(stream));

    let read_upto = read_upto.append(next_char.as_slice());

    if next_char.as_slice() == CLRF.as_bytes() {
        return Ok(read_upto);

    } else {
        return Ok(read_upto.append(try!(read_to_headers(stream)).as_slice()));
    }
}


// enum HttpVersion {
//     HTTP1.1
// }


// pub fn http(method: methods::Method, url: HttpUrl, headers: Headers, payload:, version: HttpVersion) -> ret {
//     // add code here
// }


pub fn get(url_string: &str) -> Result<Response, ()> {

    let url = try!(Url::parse(url_string).map_err(|_| {()}));
    let http_url = try!(HttpUrl::from_url(&url));
    // println!("{}", http_url);

    let headers = make_default_headers(&http_url);
    // println!("{}", headers);

    let address = Address::new(&http_url);

    let request = Request::new(methods::GET, http_url.path, headers).ok().expect("error making request!");

    // println!("{}", address);
    // println!("{}", request);

    let mut stream = BufferedStream::new(
        try!(TcpStream::connect(address.host.as_slice(), address.port).map_err(|_| {()}))
    );

    request.send(&mut stream);


    let resp = Response::new(stream);
    return resp;
}
