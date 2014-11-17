use std::io::{
    IoResult,
    Buffer,
};

use CRLF;


/// Recursive function to read up to CRLF from a stream
pub fn read_to_crlf<A: Buffer>(stream: &mut A) -> IoResult<Vec<u8>> {
    // Read to start of CRLF
    let mut read_bytes = try!(stream.read_until(CRLF[0]));

    // Read and add next byte
    let next_byte = try!(stream.read_byte());
    read_bytes.push(next_byte);

    // Keep searching (Recursion)
    if next_byte != CRLF[1] {
        read_bytes.extend(try!(read_to_crlf(stream)).into_iter());
    }

    Ok(read_bytes)
}

/// Recursive function to read an http header from a stream
pub fn read_to_header_end<A: Buffer>(stream: &mut A) -> IoResult<Vec<u8>> {
    // Read to CRLF
    let mut read_bytes = try!(read_to_crlf(stream));

    // Reached end of header
    if read_bytes[] == CRLF {
        return Ok(read_bytes);
    }

    // Read to next CRLF and add to read bytes
    let next_bytes = try!(read_to_crlf(stream));
    read_bytes.extend(next_bytes.iter().map(|&x| x));

    // Keep searching (Recursion)
    if next_bytes[] != CRLF {
        read_bytes.extend(try!(read_to_header_end(stream)).into_iter());
    }

    Ok(read_bytes)
}
