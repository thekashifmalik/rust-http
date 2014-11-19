use std::io::{
    BufferedStream,
    IoResult,
    TcpStream,
};
use std::io::net::ip::ToSocketAddr;
use readers;

pub struct Connection {
    pub stream: BufferedStream<TcpStream>,
}


impl Connection {
    pub fn new<A: ToSocketAddr>(address: A) -> IoResult<Connection> {
        // Connect to server
        let tcp_stream = try!(TcpStream::connect(address));

        // Return stream stream in connection struct
        Ok(Connection {
            stream: BufferedStream::new(tcp_stream),
        })
    }
}


impl Connection {

    pub fn write(&mut self, payload: &[u8]) -> IoResult<()> {
        try!(self.stream.write(payload));
        try!(self.stream.flush());
        Ok(())
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> IoResult<()> {
        loop {
            match self.stream.read_byte() {
                Ok(byte) => buffer.push(byte),
                _ => break,
            }
        }
        Ok(())
    }

    pub fn read_status_line(&mut self, buffer: &mut Vec<u8>) -> IoResult<()> {
        try!(readers::fill_buffer_to_crlf(&mut self.stream, buffer));
        Ok(())
    }

    pub fn read_header(&mut self, buffer: &mut Vec<u8>) -> IoResult<()> {
        try!(readers::fill_buffer_to_header_end(&mut self.stream, buffer));
        Ok(())
    }
}

