extern crate url;
extern crate serialize;

use std::io::IoResult;
use std::io::net::ip::ToSocketAddr;

use readers;
use connections::Connection;


/// Manager of cookie persistence, connection-pooling, and configuration
pub struct Transaction {
    pub connection: Connection,
    pub header: Option<HttpHeaderBytes>,
}

pub struct HttpHeaderBytes {
    pub start_line: Vec<u8>,
    pub headers: Vec<u8>,
}

pub struct HttpMessageBytes {
    pub header: HttpHeaderBytes,
    pub body: Vec<u8>,
}

impl Transaction {
    pub fn new(connection: Connection) -> IoResult<Transaction> {
        Ok(Transaction {
            connection: connection,
            header: None
        })
    }

    pub fn write(&mut self, payload: &[u8]) -> IoResult<()> {
        // Write, flush and return
        try!(self.connection.stream.write(payload));

        try!(self.connection.stream.flush());
        Ok(())
    }

    pub fn read(mut self) -> IoResult<HttpMessageBytes> {
        // Make sure header has been recieved
        match self.header {
            None => try!(self.fetch_header()),
            _ => (),
        };

        // Read body
        let body_bytes = try!(self.connection.stream.read_to_end());

        // Construct and return Bytes struct
        Ok(HttpMessageBytes {
            header: self.header.unwrap(),
            body: body_bytes,
        })
    }

    pub fn fetch_header(&mut self) -> IoResult<()> {
        // Read status line and headers
        let start_line_bytes = try!(readers::read_to_crlf(&mut self.connection.stream));
        let headers_bytes = try!(readers::read_to_header_end(&mut self.connection.stream));

        // Create and save struct on self and return
        self.header = Some(HttpHeaderBytes {
            start_line: start_line_bytes,
            headers: headers_bytes,
        });
        Ok(())
    }
}

