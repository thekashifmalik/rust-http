extern crate http;

use http::status_codes;
// use std::num::FromPrimitive;

#[test]
fn compiles() {
    assert!(true);
}

#[test]
fn comprehensive() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };
    assert_eq!(response.status_code as int, 200);
    // assert_eq!(response.status_code, status_codes::OK);
    // assert_eq!(response.headers.find("date"), );
}

#[test]
fn make_request_to_google() {
    match http::get("http://www.google.com") {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}


#[test]
fn status_code_on_response_as_int() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };
    assert_eq!(response.status_code as int, 200);
}

#[test]
fn status_code_on_response_as_type() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };
    // assert_eq!(response.status_code, status_codes::OK);
}

#[test]
fn url_on_response() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };
    assert_eq!(response.url.as_slice(), "http://www.google.com");
}

#[test]
fn get_text() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };

    if response.text.as_slice() == "" {
        assert!(false);
    }
}

#[test]
fn get_server_header() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };

    match response.headers.find("server") {
        Some(v) => assert_eq!(v, "gws"),
        None => assert!(false),
    }
}


#[test]
fn get_date_header() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };

    match response.headers.find("date") {
        Some(_) => {},
        None => assert!(false),
    }

}

#[test]
fn get_json() {
    let response = match http::get("http://www.google.com") {
        Ok(resp) => resp,
        Err(_) => panic!("There was a problem making the request"),
    };

    match response.json() {
        Some(_) => assert!(true),
        None => assert!(false),
    }
}
