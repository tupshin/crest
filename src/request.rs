/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

 */

/*!
REST requests.
*/

use hyper::header::Headers;
use hyper::client::Response;
use url::Url;

use error::Result;
use Endpoint;

macro_rules! impl_Request {
    ($ty: ident, $method: ident) => (
        impl<'a> Request<'a> for $ty<'a> {
            fn new<P>(
                endpoint: &'a Endpoint,
                path: P,
            ) -> Self where
                P: IntoIterator<Item = &'a str>
            {
                let path = path.into_iter().collect::<Vec<_>>().join("/");
                let url = endpoint.base.join(&path).unwrap();
                let data = Data {
                    url: url,
                    headers: None,
                    body: None,
                };

                $ty {
                    endpoint: endpoint,
                    data: data,
                }
            }

            fn parameters<P>(&mut self, params: P) where
                P: IntoIterator<Item = (&'a str, &'a str)>
            {
                let mut params = params.into_iter()
                    .map(|(x, y)| (x.into(), y.into()))
                    .collect();

                let new_params;
                if let Some(mut found_params) = self.data.url.query_pairs() {
                    found_params.append(&mut params);
                    new_params = found_params
                } else {
                    new_params = params
                };
                self.data.url.set_query_from_pairs(new_params);
            }

            fn headers(&mut self) -> &mut Headers {
                if let Some(ref mut headers) = self.data.headers {
                    headers
                } else {
                    self.data.headers = Some(Headers::new());
                    self.data.headers.as_mut().unwrap()
                }
            }

            fn send(self) -> Result<Response> {
                let mut request = self.endpoint.client
                    .$method(self.data.url);

                if let Some(headers) = self.data.headers {
                    request = request.headers(headers);
                }

                if let Some(body) = self.data.body {
                    request = request.body(body);
                }

                let response = request
                    .send()
                    .map_err(From::from);

                response
            }
        }
    )
}

pub trait Request<'a> {
    /**
    Constructs a REST request from a given `Endpoint`.

    The `path` argument locates a REST resource; for example, a resource at
    `/status/418` can be referenced like this:

    ```
    let resource = vec!["status", "418"];
    ```
     */
    fn new<P>(
        endpoint: &'a Endpoint,
        path: P,
    ) -> Self where
        P: IntoIterator<Item = &'a str>;

    /**
    Appends the passed parameters to the HTTP query.

    Parameters may, for example, be stored like this:

    ```
    let params = vec![
        ("param1", "value1"),
        ("param2", "value2"),
    ];
    ```
     */
    fn parameters<P>(&mut self, params: P) where
        P: IntoIterator<Item = (&'a str, &'a str)>;

    /**
    Gives a mutable reference to the `Headers` inside a `Request`.

    For example, to declare a header with `Connection: close`:

    ```
    # extern crate hyper;
    # extern crate crest;
    use hyper::header;
    # use crest::prelude::*;

    # fn main() {
    # let endpoint = Endpoint::new("https://httpbin.org/").unwrap();
    # let mut request = endpoint.get(vec!["ip"]);
    // assuming a declared `request`
    request.headers().set(header::Connection::close());
    # }
    ```
     */
    fn headers(&mut self) -> &mut Headers;

    /**
    Performs the request.
     */
    fn send(self) -> Result<Response>;
}

pub trait Body<'a> where
    Self: Request<'a>
{
    /**
    Sets the body of a `Request`.
     */
    fn body(&mut self, body: &'a str);
}

struct Data<'a> {
    url: Url,
    headers: Option<Headers>,
    body: Option<&'a str>,
}

/**
A `GET` request.
 */
pub struct Get<'a> {
    endpoint: &'a Endpoint,
    data: Data<'a>,
}

impl_Request!(Get, get);

/**
A `POST` request.
 */
pub struct Post<'a> {
    endpoint: &'a Endpoint,
    data: Data<'a>,
}

impl_Request!(Post, post);

impl<'a> Body<'a> for Post<'a> {
    fn body(&mut self, body: &'a str) {
        self.data.body = Some(body);
    }
}

/**
A `DELETE` request.
 */
pub struct Delete<'a> {
    endpoint: &'a Endpoint,
    data: Data<'a>,
}

impl_Request!(Delete, delete);
