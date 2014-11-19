HTTP Library for Rust
=====================

A simple HTTP library for rust. *Because learning.*

```rust
extern crate http;


fn main() {

    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        _        => panic!("There was a problem making the request"),
    };

    if response.status_code as int == 200 {
        println!("Time according to Google: {}", response.headers.get("date"))
    }
}

```
