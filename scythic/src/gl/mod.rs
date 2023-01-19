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

use crate::{Error, ScytheResult};
use glow::Context as GlContext;
use glutin::{
    display::{Display, DisplayApiPreference},
    surface::Surface as GlSurface,
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::num::NonZeroU32;

pub(super) struct Context {
    /// The OpenGL display root.
    display: Display,
}

impl Context {
    /// Create a new OpenGL context.
    ///
    /// # Safety
    ///
    /// The raw display handle mut outlive this context.
    pub(super) unsafe fn new(display: &impl HasRawDisplayHandle) -> Result<Self, Error> {
        let display = Display::new(display.raw_display_handle(), DisplayApiPreference::Egl)
            .scythe("Failed to create OpenGL display")?;

        Ok(Self { display })
    }
}

pub(super) struct Surface {}

impl Surface {
    /// Create a new OpenGL surface.
    ///
    /// # Safety
    ///
    /// The raw window handle must outlive this surface.
    pub(super) unsafe fn new(
        ctx: &Context,
        window: &impl HasRawWindowHandle,
        width: NonZeroU32,
        height: NonZeroU32,
    ) -> Result<Self, Error> {
        todo!()
    }
}
