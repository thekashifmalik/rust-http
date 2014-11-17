
const OK_BYTES: &'static [u8] = b"200";
const CREATED_BYTES: &'static [u8] = b"201";
const NOT_FOUND_BYTES: &'static [u8] = b"404";


#[deriving(Show)]
pub enum StatusCode {
    OK = 200,
    CREATED = 201,
    NOT_FOUND = 404,
}

impl StatusCode {

    pub fn to_bytes(self) -> &'static [u8] {
        match self {
            OK => OK_BYTES,
            CREATED => CREATED_BYTES,
            NOT_FOUND => NOT_FOUND_BYTES,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<StatusCode> {
        match bytes {
            OK_BYTES => Some(OK),
            CREATED_BYTES => Some(CREATED),
            NOT_FOUND_BYTES => Some(NOT_FOUND),
            _ => None,
        }
    }

}
