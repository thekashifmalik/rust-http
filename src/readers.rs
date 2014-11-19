use std::io::{
    IoResult,
    Reader,
};

use CR;
use LF;
use CRLF;


/// Custom read_until implementation that does not allocate
// TODO: Add timeout
fn fill_buffer_until<A: Reader>(target: u8, reader: &mut A, buffer: &mut Vec<u8>) -> IoResult<()> {
    loop {
        // Read and push byte
        buffer.push(try!(reader.read_byte()));

        // Check if byte matches target
        if buffer[buffer.len() - 1] == target {
            return Ok(());
        }
    }
}

/// Recursive function to read up to CRLF from a reader into a buffer
pub fn fill_buffer_to_crlf<A: Reader>(reader: &mut A, buffer: &mut Vec<u8>) -> IoResult<()> {
    // Read to start of CRLF
    try!(fill_buffer_until(CR, reader, buffer));

    // Read and add next byte
    let next_byte = try!(reader.read_byte());
    buffer.push(next_byte);

    // Keep searching (Recursion)
    if next_byte != LF {
        try!(fill_buffer_to_crlf(reader, buffer));
    }

    Ok(())
}

/// Recursive function to read up to the end of an HTTP header from a reader into a buffer
pub fn fill_buffer_to_header_end<A: Reader>(reader: &mut A, buffer: &mut Vec<u8>) -> IoResult<()> {
    // Read to CRLF
    let offset = buffer.len();
    try!(fill_buffer_to_crlf(reader, buffer));

    // Reached end of header
    if buffer[offset..] == CRLF {
        return Ok(());
    }

    // Read to next CRLF
    let second_offset = buffer.len();
    try!(fill_buffer_to_crlf(reader, buffer));

    // Keep searching (Recursion)
    if buffer[second_offset..] != CRLF {
        try!(fill_buffer_to_header_end(reader, buffer));
    }

    Ok(())
}



pub trait HttpReader: Reader {
    fn read_header(&mut self, vector: &mut Vec<u8>) -> IoResult<uint>;
    fn read_response(&mut self, vector: &mut Vec<u8>) -> IoResult<uint>;
}
