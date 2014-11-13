extern crate url;
extern crate serialize;

use std::io::{
    BufferedStream,
    TcpStream,
    IoResult,
};
use std::io::net::ip::ToSocketAddr;

mod readers;

/// Manager of an HTTP connection
pub struct Connection {
    pub stream: BufferedStream<TcpStream>,
    pub header: Option<HttpHeaderBytes>,
}


#[deriving(Clone)]
pub struct HttpHeaderBytes {
    pub start_line: Vec<u8>,
    pub headers: Vec<u8>,
}

#[deriving(Clone)]
pub struct HttpMessageBytes {
    pub header: HttpHeaderBytes,
    pub body: Vec<u8>,
}

impl Connection {
    pub fn new<A: ToSocketAddr>(address: A) -> IoResult<Connection> {
        let tcp_stream = try!(TcpStream::connect(address));
        Ok(Connection {
            stream: BufferedStream::new(tcp_stream),
            header: None
        })
    }

    pub fn write(&mut self, payload: &[u8]) -> IoResult<()> {
        // Write flush an return
        try!(self.stream.write(payload));

        try!(self.stream.flush());
        Ok(())
    }

    pub fn read(mut self) -> IoResult<HttpMessageBytes> {
        // Make sure header has been recieved.
        match self.header {
            None => try!(self.fetch_header()),
            _ => (),
        };

        // Read body
        let body_bytes = try!(self.stream.read_to_end());

        // Construct and return Bytes struct
        Ok(HttpMessageBytes {
            header: self.header.unwrap(),
            body: body_bytes,
        })
    }

    pub fn fetch_header(&mut self) -> IoResult<()> {
        // Read status line and headers
        let start_line_bytes = try!(readers::read_to_crlf(&mut self.stream));
        let headers_bytes = try!(readers::read_to_header_end(&mut self.stream));

        // Create and save struct on self and return
        self.header = Some(HttpHeaderBytes {
            start_line: start_line_bytes,
            headers: headers_bytes,
        });
        Ok(())
    }
}

