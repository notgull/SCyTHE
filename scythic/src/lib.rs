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

//! Use `scythe` to draw to windowed surfaces.

pub use scythe::Error;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::num::NonZeroU32;

// Softbuffer backend is always enabled.
mod softbuffer;

#[cfg(feature = "gl")]
mod gl;

/// A handle for the display server for the painter.
pub struct Context<T> {
    /// The underlying display handle.
    display: T,

    /// The dispatch table for the context.
    ///
    /// This must be dropped before `display`.
    dispatch: ManuallyDrop<ContextDispatch>,

    /// This type is not thread-safe.
    _marker: PhantomData<*mut ()>,
}

/// A handle for a window to draw to.
pub struct Surface<T> {
    /// The underlying window handle.
    window: T,

    /// The dispatch table for the surface.
    ///
    /// This must be dropped before `window`.
    dispatch: ManuallyDrop<SurfaceDispatch>,

    /// This type is not thread-safe.
    _marker: PhantomData<*mut ()>,
}

impl<T> Drop for Context<T> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.dispatch);
        }
    }
}

impl<T> Drop for Surface<T> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.dispatch);
        }
    }
}

macro_rules! dispatch {
    ($($(#[$meta:meta])* $mod:ident),* $(,)?) => {
        #[allow(non_camel_case_types)]
        enum ContextDispatch {
            $($(#[$meta])* $mod ($mod::Context)),*
        }

        #[allow(non_camel_case_types)]
        enum SurfaceDispatch {
            $($(#[$meta])* $mod ($mod::Surface)),*
        }

        impl ContextDispatch {
            unsafe fn new(display: &impl HasRawDisplayHandle) -> Result<Self, Error> {
                let mut last_error;

                $(
                    {
                        match $mod::Context::new(display) {
                            Ok(context) => { return Ok(Self::$mod(context)); },
                            Err(e) => { last_error = e; },
                        }

                        tracing::warn!(
                            "{} error: {}",
                            stringify!($mod),
                            &last_error
                        );
                    }
                )*

                Err(last_error)
            }
        }

        impl SurfaceDispatch {
            unsafe fn new(
                context: &ContextDispatch,
                window: &impl HasRawWindowHandle,
                width: NonZeroU32,
                height: NonZeroU32
            ) -> Result<Self, Error> {
                match context {
                    $(
                        ContextDispatch::$mod(context) => {
                            $mod::Surface::new(context, window, width, height).map(Self::$mod)
                        }
                    )*
                }
            }
        }
    };
}

dispatch! {
    // In order of preference.

    /// A backend that uses OpenGL.
    #[cfg(feature = "gl")]
    gl,

    /// A software-based backend.
    softbuffer,
}

impl<T: HasRawDisplayHandle> Context<T> {
    /// Create a new `Context` from a display handle.
    pub fn new(display: T) -> Result<Self, Error> {
        unsafe {
            let dispatch = ContextDispatch::new(&display)?;
            Ok(Self {
                display,
                dispatch: ManuallyDrop::new(dispatch),
                _marker: PhantomData,
            })
        }
    }
}

impl<T: HasRawWindowHandle> Surface<T> {
    /// Create a new `Surface` from a window handle.
    pub fn new<D>(
        context: &Context<D>,
        window: T,
        width: NonZeroU32,
        height: NonZeroU32,
    ) -> Result<Self, Error> {
        unsafe {
            let dispatch = SurfaceDispatch::new(&context.dispatch, &window, width, height)?;
            Ok(Self {
                window,
                dispatch: ManuallyDrop::new(dispatch),
                _marker: PhantomData,
            })
        }
    }
}

// TODO: Impl scythe::Surface for Surface

trait ScytheResult<T, E: std::error::Error + 'static> {
    /// Convert to a `scythe` result with a public error.
    fn scythe_pub(self, msg: impl Into<String>) -> Result<T, Error>;

    /// Convert to a `scythe` result with a private error.
    fn scythe(self, msg: impl Into<String>) -> Result<T, Error>;
}

impl<T, E: std::error::Error + 'static> ScytheResult<T, E> for Result<T, E> {
    fn scythe_pub(self, msg: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::from_public_error_with_message(e, msg.into()))
    }

    fn scythe(self, msg: impl Into<String>) -> Result<T, Error> {
        self.map_err(|e| Error::from_error_with_message(e, msg.into()))
    }
}
