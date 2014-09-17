extern crate http;


fn main() {

    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        _        => fail!("There was a problem making the request"),
    };

    if response.status_code as int == 200 {
        println!("Time according to Google: {}", response.headers.get("date"))
    }

    // println!("URL:\n----\n{}\n----", response.url)
    // println!("Headers:\n----\n{}----", response.headers)
}
