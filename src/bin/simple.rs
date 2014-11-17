extern crate http;

use http::Headers;


fn main() {
    // Create request headers
    let mut headers = Headers::new();
    headers.insert("Accept-Encoding", "gzip");
    headers.insert("lol", "gzip");

    // Make request
    let response = match http::get("http://localhost:8000", Some(&headers)) {
        Ok(resp) => resp,
        _        => panic!("There was a problem making the request"),
    };

    // Check status code and response headers
    if response.status_code as int == 200 {
        println!("Time: {}", response.headers.get("date"))
        println!("Header: {}", response.headers.get("x-custom-header"))
    }
}
