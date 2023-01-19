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

//! A `scythic` backend that uses `softbuffer` to draw to a window.

use crate::{Error, ScytheResult};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use softbuffer::{Context as SoftbufferContext, Surface as SoftbufferSurface};

use std::num::NonZeroU32;

pub(super) struct Context {
    /// The softbuffer context.
    sb: SoftbufferContext,
}

impl Context {
    /// Create a new `Context` from a `RawDisplayHandle`.
    ///
    /// # Safety
    ///
    /// This `Context` must live as long as the `RawDisplayHandle`.
    pub(super) unsafe fn new(display: &impl HasRawDisplayHandle) -> Result<Self, Error> {
        // SAFETY: The `RawDisplayHandle` must live as long as the `Context`.
        let sb = SoftbufferContext::new(display).scythe("Failed to create softbuffer context")?;

        Ok(Self { sb })
    }
}

pub(super) struct Surface {
    /// The softbuffer surface.
    sb: SoftbufferSurface,

    /// The current width of the surface.
    width: NonZeroU32,

    /// The current height of the surface.
    height: NonZeroU32,
}

impl Surface {
    /// Create a new `SoftbufferSurface` from a `RawWindowHandle`.
    ///
    /// # Safety
    ///
    /// This `Surface` must live as long as the `RawWindowHandle`.
    pub(super) unsafe fn new(
        ctx: &Context,
        window: &impl HasRawWindowHandle,
        width: NonZeroU32,
        height: NonZeroU32,
    ) -> Result<Self, Error> {
        // SAFETY: The `RawWindowHandle` must live as long as the `Surface`.
        let sb = SoftbufferSurface::new(&ctx.sb, window)
            .scythe("Failed to create softbuffer surface")?;

        Ok(Self { sb, width, height })
    }
}
