/*

Copyright (c) 2016 Pablo Couto

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
or the MIT license <LICENSE-MIT>, at your option. All files in
the project carrying such notice may not be copied, modified, or
distributed except according to those terms.

*/

use std::convert::From;
use std::error::Error as StdError;
use std::fmt;

use hyper::error::Error as HyperError;
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    Url(UrlParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref e) => e.description(),
            Error::Url(ref e) => e.description(),
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<UrlParseError> for Error {
    fn from(e: UrlParseError) -> Error {
        Error::Url(e)
    }
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}