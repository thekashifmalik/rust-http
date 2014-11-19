extern crate url;
extern crate serialize;

use std::io::IoResult;
use std::io::net::ip::ToSocketAddr;

use readers;
use connections::Connection;


pub struct HttpMessageBytes {
    bytes: Vec<u8>,
    start_line_length: uint,
    header_length: uint,
}

impl HttpMessageBytes {

    pub fn new(bytes: Vec<u8>, start_line_length: uint, header_length: uint) -> HttpMessageBytes {
        HttpMessageBytes {
            bytes: bytes,
            start_line_length: start_line_length,
            header_length: header_length,
        }
    }

    pub fn get_start_line_bytes(&self) -> &[u8] {
        self.bytes[..self.start_line_length]
    }

    pub fn get_header_bytes(&self) -> &[u8] {
        self.bytes[..self.header_length]
    }

    pub fn get_headers_bytes(&self) -> &[u8] {
        self.bytes[self.start_line_length..self.header_length]
    }

    pub fn get_body_bytes(&self) -> &[u8] {
        self.bytes[self.header_length..]
    }

    pub fn get_bytes(&self) -> &[u8] {
        self.bytes[]
    }

    pub fn unwrap(self) -> Vec<u8> {
        self.bytes
    }
}


struct ReadLengths {
    start_line: uint,
    header: uint,
}


// Manages an HTTP transaction
pub struct Transaction<'a> {
    connection: &'a mut Connection,
    buffer: &'a mut Vec<u8>,
    read_lengths: Option<ReadLengths>,
}

impl<'a> Transaction<'a> {
    pub fn new(connection: &'a mut Connection, buffer: &'a mut Vec<u8>) -> IoResult<Transaction<'a>> {
        Ok(Transaction {
            connection: connection,
            buffer: buffer,
            read_lengths: None,
        })
    }

    pub fn perform(mut self, payload: &[u8]) -> IoResult<HttpMessageBytes> {

        try!(self.connection.write(payload));
        self.read_response()
    }

    fn read_response(mut self) -> IoResult<HttpMessageBytes> {
        // Make sure header has been recieved
        match self.read_lengths {
            None => try!(self.fetch_header()),
            _ => (),
        };


        // Read body
        loop {
            match self.connection.stream.read_byte() {
                Ok(byte) => self.buffer.push(byte),
                _ => break,
            }
        }

        let thing = HttpMessageBytes::new(
            self.buffer.clone(),
            self.read_lengths.unwrap().start_line,
            self.read_lengths.unwrap().header,
        );

        Ok(thing)
    }

    pub fn fetch_header(&mut self) -> IoResult<()> {
        // Read status_line
        let starting_length = self.buffer.len();
        try!(self.connection.read_status_line(self.buffer));
        let start_line_length = self.buffer.len() - starting_length;

        // Read rest of header
        try!(self.connection.read_header(self.buffer));
        let header_length = self.buffer.len() - starting_length;

        self.read_lengths = Some(ReadLengths {
            start_line: start_line_length,
            header: header_length,
        });

        Ok(())
    }
}

