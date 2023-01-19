// Copyright 2023 John Nunley
//
// This file is part of Scythe.
//
// Scythe is free software: you can redistribute it and/or modify it under the
// terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Scythe is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Scythe. If not, see <https://www.gnu.org/licenses/>.

//! A common interface for two-dimensional vector graphics.

#![forbid(unsafe_code)]

/// Geometric primitives from the `blood_geometry` crate.
pub extern crate blood_geometry as geometry;

/// Image manipulation from the `genimage` crate.
pub extern crate genimage as image;

mod canvas;
mod clip;
mod drawable;
mod pattern;

pub use canvas::Canvas;
pub use clip::Clip;
pub use drawable::Drawable;
pub use pattern::Pattern;

use std::error::Error as StdError;
use std::fmt;

/// The error type used by `scythe`.
pub struct Error(Box<Impl>);

struct Impl {
    /// The underlying system error.
    source: Box<dyn StdError>,

    /// A message associated with the error.
    message: Option<Box<str>>,
}

impl Error {
    /// Creates an `Error` from a system error that can be exposed in the public interface.
    pub fn from_public_error<E>(source: E) -> Self
    where
        E: Into<Box<dyn StdError>>,
    {
        Self::new(source.into(), None)
    }

    /// Creates an `Error` from a system error that can be exposed in the public interface and a
    /// message.
    pub fn from_public_error_with_message<E, M>(source: E, message: M) -> Self
    where
        E: Into<Box<dyn StdError>>,
        M: Into<String>,
    {
        Self::new(source.into(), Some(message.into().into_boxed_str()))
    }

    /// Creates an `Error` from a system error that cannot be exposed in the public interface.
    pub fn from_error<E: StdError + 'static>(source: E) -> Self {
        Self::new(Box::new(PrivateError(source)), None)
    }

    /// Creates an `Error` from a system error that cannot be exposed in the public interface and a
    /// message.
    pub fn from_error_with_message<E, M>(source: E, message: M) -> Self
    where
        E: StdError + 'static,
        M: Into<String>,
    {
        Self::new(
            Box::new(PrivateError(source)),
            Some(message.into().into_boxed_str()),
        )
    }

    fn new(source: Box<dyn StdError>, message: Option<Box<str>>) -> Self {
        Self(Box::new(Impl { source, message }))
    }

    /// Get the message associated with the error.
    pub fn message(&self) -> Option<&str> {
        self.0.message.as_deref()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("source", &self.0.source)
            .field("message", &self.0.message)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(message) = &self.0.message {
            write!(f, "{}: {}", message, self.0.source)
        } else {
            write!(f, "{}", self.0.source)
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.0.source)
    }
}

/// A wrapper error type that external crates cannot downcast to.
struct PrivateError<E>(E);

impl<E: StdError> fmt::Debug for PrivateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<E: StdError> fmt::Display for PrivateError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

// Can't expose any sources or risk exposing a private interface.
impl<E: StdError> StdError for PrivateError<E> {}
