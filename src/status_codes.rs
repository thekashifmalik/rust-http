
type StatusCodeBytes = &'static [u8];

const OK: StatusCodeBytes = b"200";
const CREATED: StatusCodeBytes = b"201";
const NOT_FOUND: StatusCodeBytes = b"404";


#[deriving(Show, Eq, PartialEq, FromPrimitive)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NotFound = 404,
}

pub fn from_bytes(bytes: &[u8]) -> Option<StatusCode> {
    match bytes {
        OK => Some(Ok),
        CREATED => Some(Created),
        NOT_FOUND => Some(NotFound),
        _ => None,
    }
}
