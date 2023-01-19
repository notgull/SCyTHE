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

//! Use `scythe` to draw to images.

#![forbid(unsafe_code)]

use scythe::image::format::Format;

/// An image buffer that can be drawn to using `scythe`.
pub struct ImageBuffer<'a> {
    /// The buffer to draw to.
    buffer: &'a mut [u8],

    /// The width of the image.
    width: u32,

    /// The height of the image.
    height: u32,
}

impl<'a> ImageBuffer<'a> {
    /// Create a new image buffer.
    pub fn new(buffer: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }
}

// TODO: Impl surface
