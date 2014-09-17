
#[deriving(Show, Eq, PartialEq, FromPrimitive)]
pub enum StatusCode {
    OK = 200,
    CREATED = 201,

    NOT_FOUND = 404,
    // NOTFOUND = 404,
    // NotFound = 404,

    // UNIMPLEMENTED(u16),
}

// impl StatusCode {
//     fn from_int(number: int) -> StatusCode {
//         match number {
//             Some() => expr,
//         }
//     }
// }
