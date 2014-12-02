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
        // TODO: Remove any allocation from Url::parse
        let url = try!(Url::parse(url_string).map_err(|_| {()}));

        // Check scheme
        let scheme = match Scheme::new(url.scheme[]) {
            Some(a) => a,
            None => return Err(()),
        };

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


enum Scheme {
    HTTP,
    HTTPS,
}

impl Scheme {
    pub fn new(slice: &str) -> Option<Scheme> {
        match slice {
            "http" => Some(Scheme::HTTP),
            "https" => Some(Scheme::HTTPS),
            _ => None,
        }
    }

    pub fn default_port(&self) -> u16 {
        match *self {
            Scheme::HTTP => 80,
            Scheme::HTTPS => 443,
        }
    }
}
