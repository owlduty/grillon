use crate::error::Result;
use crate::Request;
use hyper::{client::HttpConnector, header::HeaderMap, Client, Method, Uri};

/// Top-level instance to configure a REST API http client.
///
/// [`Grillon`] provides everything to configure a REST API http client,
/// and initiate a [`Request`].
pub struct Grillon {
    base_url: Uri,
    client: Client<HttpConnector>,
}

impl Grillon {
    /// Creates a new instance of `Grillon` with the base API url.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let grillon = Grillon::new("http://jsonplaceholder.typicode.com")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function fails if the supplied base url cannot be parsed as a [`Uri`].
    pub fn new(api_base_url: &str) -> Result<Grillon> {
        Ok(Grillon {
            base_url: api_base_url.parse::<Uri>()?,
            client: Client::builder().build_http(),
        })
    }

    /// Creates a new [`Request`] initialized with a GET method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .get("users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, path: &str) -> Request {
        self.http_request(Method::GET, path)
    }

    /// Creates a new [`Request`] initialized with a POST method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .post("users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn post(&self, path: &str) -> Request {
        self.http_request(Method::POST, path)
    }

    /// Creates a new [`Request`] initialized with a PUT method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .put("users/1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn put(&self, path: &str) -> Request {
        self.http_request(Method::PUT, path)
    }

    /// Creates a new [`Request`] initialized with a PATCH method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .patch("users/1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn patch(&self, path: &str) -> Request {
        self.http_request(Method::PATCH, path)
    }

    /// Creates a new [`Request`] initialized with a DELETE method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .delete("users/1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, path: &str) -> Request {
        self.http_request(Method::DELETE, path)
    }

    /// Create a new [`Request`] initialized with the given method and path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Method, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .http_request(Method::GET, "users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn http_request(&self, method: Method, path: &str) -> Request {
        let uri = crate::url::concat(&self.base_url, path).unwrap_or_else(|err| panic!("{}", err));

        Request {
            method,
            uri,
            headers: HeaderMap::new(),
            payload: None,
            client: &self.client,
        }
    }
}
