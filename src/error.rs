//
// Copyright 2018-2019 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # Wallet Error
//!
//! Modules of this library use this error class to indicate problems.
//!

use std::{convert, fmt, io};

use bitcoin::util::bip32;
use crypto::symmetriccipher;

/// An error class to offer a unified error interface upstream
pub enum Error {
    /// Unsupported
    Unsupported(&'static str),
    /// mnemonic related error
    Mnemonic(&'static str),
    /// wrong passphrase
    Passphrase,
    /// wrong network
    Network,
    /// Network IO error
    IO(io::Error),
    /// key derivation error
    KeyDerivation(bip32::Error),
    /// sekp256k1 error
    SecpError(bitcoin::secp256k1::Error),
    /// cipher error
    SymmetricCipherError(symmetriccipher::SymmetricCipherError),
    SigHashError(bitcoin::util::sighash::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            Error::Passphrase => write!(f, "wrong passphrase"),
            Error::Network => write!(f, "wrong network"),
            Error::Unsupported(ref s) => write!(f, "Unsupported: {}", s),
            Error::Mnemonic(ref s) => write!(f, "Mnemonic: {}", s),
            Error::IO(ref err) => write!(f, "IO error: {}", err),
            Error::KeyDerivation(ref err) => write!(f, "BIP32 error: {}", err),
            Error::SecpError(ref err) => write!(f, "Secp256k1 error: {}", err),
            Error::SymmetricCipherError(ref err) => write!(
                f,
                "Cipher error: {}",
                match err {
                    &symmetriccipher::SymmetricCipherError::InvalidLength => "invalid length",
                    &symmetriccipher::SymmetricCipherError::InvalidPadding => "invalid padding",
                }
            ),
            Error::SigHashError(ref err) => write!(f, "Sighash error: {err}"),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl convert::From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        match err {
            Error::IO(e) => e,
            _ => io::Error::new(io::ErrorKind::Other, err.to_string()),
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl convert::From<bip32::Error> for Error {
    fn from(err: bip32::Error) -> Error {
        Error::KeyDerivation(err)
    }
}

impl convert::From<symmetriccipher::SymmetricCipherError> for Error {
    fn from(err: symmetriccipher::SymmetricCipherError) -> Error {
        Error::SymmetricCipherError(err)
    }
}

impl convert::From<bitcoin::secp256k1::Error> for Error {
    fn from(err: bitcoin::secp256k1::Error) -> Error {
        Error::SecpError(err)
    }
}

impl From<bitcoin::util::sighash::Error> for Error {
    fn from(err: bitcoin::util::sighash::Error) -> Error {
        Error::SigHashError(err)
    }
}
