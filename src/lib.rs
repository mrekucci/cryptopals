// Copyright (c) 2018, Peter Mrekaj. All rights reserved.
// Use of this source code is governed by a ISC-style
// license that can be found in the LICENSE.txt file.

//! Solutions for [the cryptopals crypto challenges][crypto].
//!
//! [crypto]: http://cryptopals.com/

extern crate hex;

use std::error;
use std::fmt;
use std::result;
use std::string;

/// A type alias for `Result<T, cryptopals::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when executing functions that solve the cryptopals crypto challenges.
#[derive(Debug)]
pub enum Error {
    FromHex(hex::FromHexError),
    FromUtf8(string::FromUtf8Error),
    FromBase64(base64::DecodeError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::FromHex(ref err) => err.fmt(f),
            Error::FromUtf8(ref err) => err.fmt(f),
            Error::FromBase64(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::FromHex(ref err) => err.description(),
            Error::FromUtf8(ref err) => err.description(),
            Error::FromBase64(ref err) => err.description(),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Self {
        Error::FromHex(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::FromUtf8(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Self {
        Error::FromBase64(err)
    }
}

pub mod set1;
