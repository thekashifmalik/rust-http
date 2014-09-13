use std::collections::HashMap;

mod http;


fn main() {

    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(error) => fail!("There was a problem making the request"),
    };

    println!("status code: {}", response.status_code);
    println!("body length: {}", response.text.len());
    println!("header[content-type]: {}", response.headers.find("content-type"));
    // Aspiration -> println!("header[content-type]: {}", response.headers["content-type"]);
    println!("JSON: {}", response.json());  // HACK
}
