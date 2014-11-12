extern crate url;

use self::url::Url;


#[deriving(Show)]
pub struct HttpUrl<'a> {
    pub url: &'a str,
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}


impl<'a> HttpUrl<'a> {

    pub fn from_str(url_string: &str) -> Result<HttpUrl, ()> {
        // Parse URL
        let url = try!(Url::parse(url_string).map_err(|_| {()}));

        // Check scheme
        if url.scheme.as_slice() != "http" && url.scheme.as_slice() != "https" {
            return Err(());
        }

        // Check host
        let host = match url.domain() {
            Some(h) => h.into_string(),
            None => return Err(()),
        };

        // Create and format path
        let path = match url.path() {
            Some(path_vector) => format!("/{}", path_vector.connect("/")),
            None => "/".to_string(),
        };

        Ok(HttpUrl {
            url: url_string,
            host: host,
            port: url.port(),
            scheme: url.scheme,
            path: path,
        })
    }
}
