use std::io::{
    BufferedStream,
    TcpStream,
    IoResult,
};


static CRLF: &'static [u8] = b"\r\n";


/// Recursive function to read up to CRLF from a stream
pub fn read_to_crlf(stream: &mut BufferedStream<TcpStream>) -> IoResult<Vec<u8>> {
    // Read to start of CRLF
    let mut read_bytes = try!(stream.read_until(CRLF[0]));

    // Read and add next byte
    let next_byte = try!(stream.read_byte());
    read_bytes.push(next_byte);

    // Reached end of CRLF
    if next_byte == CRLF[1] {
        Ok(read_bytes)

    // Keep searching (Recursion)
    } else {
        Ok(read_bytes.append(try!(read_to_crlf(stream)).as_slice()))
    }
}

/// Recursive function to read an http header from a stream
pub fn read_to_header_end(stream: &mut BufferedStream<TcpStream>) -> IoResult<Vec<u8>> {
    // Read to CRLF
    let read_bytes = try!(read_to_crlf(stream));

    // Reached end of header
    if read_bytes.as_slice() == CRLF {
        return Ok(read_bytes);
    }

    // Read to next CRLF and add to read bytes
    let next_bytes = try!(read_to_crlf(stream));
    let read_bytes = read_bytes.append(next_bytes.as_slice());

    // Reached end of header
    if next_bytes.as_slice() == CRLF {
        Ok(read_bytes)

    // Keep searching (Recursion)
    } else {
        Ok(read_bytes.append(try!(read_to_header_end(stream)).as_slice()))
    }
}
