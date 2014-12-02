
pub use self::Method::*;

const HEAD_BYTES: &'static [u8] = b"HEAD";
const GET_BYTES: &'static [u8] = b"GET";
const POST_BYTES: &'static [u8] = b"POST";
const PUT_BYTES: &'static [u8] = b"PUT";
const DELETE_BYTES: &'static [u8] = b"DELETE";


#[deriving(Show)]
pub enum Method {
    HEAD,
    GET,
    POST,
    PUT,
    DELETE,
}


impl Method {
    pub fn to_bytes(self) -> &'static [u8] {
        match self {
            HEAD => HEAD_BYTES,
            GET => GET_BYTES,
            POST => POST_BYTES,
            PUT => PUT_BYTES,
            DELETE => DELETE_BYTES,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Method> {
        match bytes {
            HEAD_BYTES => Some(HEAD),
            GET_BYTES => Some(GET),
            POST_BYTES => Some(POST),
            PUT_BYTES => Some(PUT),
            DELETE_BYTES => Some(DELETE),
            _ => None,
        }
    }
}
