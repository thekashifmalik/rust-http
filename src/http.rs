
extern crate url;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::io::{BufferedStream, TcpStream};

use self::url::Url;
use std::io::{IoResult, IoError};
use std::io::Stream;


struct Response {
    pub status_code: uint,
    pub text: String
}


fn read_to_crlf<T: Stream>(stream: &mut BufferedStream<T>) -> IoResult<Vec<u8>> {
	let mut read_upto = try!(stream.read_until('\r' as u8));

	let next_char = try!(stream.read_byte());
	read_upto.push(next_char);

	if next_char  == '\n' as u8 {
		return Ok(read_upto);
	
	} else {
		return Ok(read_upto.append(try!(read_to_crlf(stream)).as_slice()));
	}
}


fn read_to_headers<T: Stream>(stream: &mut BufferedStream<T>) -> IoResult<Vec<u8>> {
	let mut read_upto = try!(read_to_crlf(stream));

	if read_upto.as_slice() == b"\r\n" {
		return Ok(read_upto);
	}

	let next_char = try!(read_to_crlf(stream));
	
	let mut read_upto = read_upto.append(next_char.as_slice());
	
	if next_char.as_slice() == b"\r\n" {
		return Ok(read_upto);
	
	} else {
		return Ok(read_upto.append(try!(read_to_headers(stream)).as_slice()));
	}
}

pub fn get(url_string: &str) -> Result<Response, ()> {
	// Get address
	let url = match Url::parse(url_string) {
		Ok(x) => x,
		Err(_) => return Err(()),
	};

	let host = match url.domain() {
    	Some(x) => (x),
    	None => return Err(()),
	};

	let port = match url.port() {
		Some(x) => x,
		None => if url.scheme.as_slice() == "https" { 443 } else { 80 },
	};

	let path = match url.path() {
		Some(x) => format!("/{}", x.connect("/")),
    	None => return Err(()),
	};

	let mut stream = match TcpStream::connect(host, port) {
		Ok(x) => BufferedStream::new(x),
		Err(_) => return Err(()),
	};

	let status_line_to_send = format!("GET {} HTTP/1.0\n\n", path);
	println!("{}", status_line_to_send);
	stream.write(status_line_to_send.as_bytes());
	stream.flush();

	let status_line = stream.read_line().unwrap();
	let status_vector: Vec<&str> = status_line.as_slice().trim().splitn(2, ' ').collect();

	let header_bytes = read_to_headers(&mut stream).unwrap();

	let header_string = match String::from_utf8(header_bytes) {
		Ok(x) => x,
		Err(_) => return Err(()),
	};

	println!("{}", header_string);
	println!("{}", status_vector);

	let text_bytes = stream.read_to_end().unwrap();	

	Ok(Response {
	    status_code: from_str(status_vector[1]).unwrap(),
	    text: String::from_utf8(text_bytes).unwrap(),
	})
}
