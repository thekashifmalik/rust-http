use std::io::{
    BufferedStream,
    IoResult,
    TcpStream,
};
use std::io::net::ip::ToSocketAddr;


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
