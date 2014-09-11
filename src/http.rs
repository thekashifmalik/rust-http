
extern crate url;


use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::{BufferedStream, TcpStream};
use std::from_str::FromStr;

use self::url::Url;
use std::io::{IoResult, IoError};
use std::io::Stream;


mod methods {

    #[deriving(Show)]
    pub enum Method {
        GET,
        HEAD,
        POST,
        PUT,
        DELETE,
        CONNECT,
        OPTIONS,
        TRACE,
    }

    // impl Show for Method {
    //     pub fn to_string(self) -> String {
    //         match self {
    //             GET => "GdET".to_string(),
    //             HEAD => "HEAD".to_string(),
    //             POST => "POST".to_string(),
    //             PUT => "PUT".to_string(),
    //             DELETE => "DELETE".to_string(),
    //             CONNECT => "CONNECT".to_string(),
    //             OPTIONS => "OPTIONS".to_string(),
    //             TRACE => "TRACE".to_string(),
    //         }
    //     }
    // }

}

static HTTP_VERSION: &'static str = "1.1";
static CLRF: &'static str = "\r\n";
static DEFAULT_HTTP_PORT: u16 = 80;


enum StartLine {
    RequestLine,
    StatusLine,
}

#[deriving(Show)]
struct Response {
    pub status_code: uint,
    pub text: String
}

impl Response {
    fn new(mut stream: BufferedStream<TcpStream>) -> Result<Response, ()> {
        let first_line = stream.read_line().ok().expect("error reading from stream!");
        let status_line: StatusLine = from_str(first_line.as_slice()).expect("error parsing status line!");

        println!("{}", status_line);

        let header_bytes = read_to_headers(&mut stream).unwrap();

        let header_string = match String::from_utf8(header_bytes) {
            Ok(x) => x,
            Err(_) => return Err(()),
        };

        let text_bytes = stream.read_to_end().unwrap();

        Ok(Response {
            status_code: status_line.status_code,
            text: String::from_utf8(text_bytes).unwrap(),
        })

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

impl Address {
    fn new(url: &Url) -> Result<Address, ()> {
        let host = match url.domain() {
            Some(x) => (x),
            None => return Err(()),
        };

        let port = match url.port() {
            Some(x) => x,
            None => if url.scheme.as_slice() == "https" { 443 } else { 80 },
        };

        Ok(Address{
            host: host.to_string(),
            port: port,
        })
    }
}

#[deriving(Show)]
struct Request {
//     address: Address,
//     request_line: RequestLine,
    method: String,
    path: String,
}

impl Request {
    fn new(method: methods::Method, url: &Url) -> Result<Request, ()> {
        let path = match url.path() {
            Some(x) => format!("/{}", x.connect("/")),
            None => return Err(()),
        };

        // let request_line = RequestLine;

        Ok(Request {
            method: method.to_string(),
            path: path.to_string(),
        })
    }


    fn to_bytes(&self) -> Vec<u8> {
        let string = format!(
            "{} {} HTTP/1.0\n\n",
            self.method,
            self.path,
        );
        return string.into_bytes()
    }

    fn send(&self, stream: &mut BufferedStream<TcpStream>) {
        stream.write(self.to_bytes().as_slice());
        stream.flush();
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

    if read_upto.as_slice() == b"\r\n" {
        return Ok(read_upto);
    }

    let next_char = try!(read_to_crlf(stream));

    let read_upto = read_upto.append(next_char.as_slice());

    if next_char.as_slice() == b"\r\n" {
        return Ok(read_upto);

    } else {
        return Ok(read_upto.append(try!(read_to_headers(stream)).as_slice()));
    }
}

pub fn get(url_string: &str) -> Result<Response, ()> {

    let url = match Url::parse(url_string) {
        Ok(x) => x,
        Err(_) => return Err(()),
    };

    let address = Address::new(&url).unwrap();
    let request = Request::new(methods::GET, &url).unwrap();
    println!("{}", address);
    println!("{}", request);

    let mut stream = BufferedStream::new(
        TcpStream::connect(address.host.as_slice(), address.port).unwrap()
    );

    request.send(&mut stream);


    let resp = Response::new(stream);
    // println!("{}", resp);
    return resp;
}
