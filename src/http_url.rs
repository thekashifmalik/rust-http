extern crate url;

use self::url::Url;


#[deriving(Show)]
pub struct HttpUrl {
    pub url: String,
    pub scheme: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
}


impl HttpUrl {

    pub fn from_str(url_string: &str) -> Result<HttpUrl, ()> {
        // Parse URL
        let url = try!(Url::parse(url_string).map_err(|_| {()}));

        // Check scheme
        if url.scheme.as_slice() != "http" && url.scheme.as_slice() != "https" {
            return Err(());
        }

        // Check host
        let host = optional_try!(url.domain());

        // Create and format path
        let path = match url.path() {
            Some(path_vector) => format!("/{}", path_vector.connect("/")),
            None => "/".to_string(),
        };

        // TODO: Allocation
        Ok(HttpUrl {
            url: url_string.to_string(),
            scheme: url.scheme.clone(),
            host: host.to_string(),
            path: path,
            port: url.port(),
        })
    }
}
