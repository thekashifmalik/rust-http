mod http;


fn main() {
    // let headers = ("kashif", "b2tmanb2tman");
	let response = http::get(
        "http://www.google.com",
    ).ok().expect("There was a problem making the request");

	println!("status code: {}", response.status_code);
	println!("body length: {}", response.text.len());
}
