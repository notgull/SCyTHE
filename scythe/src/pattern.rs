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

use crate::drawable::Drawable;
use geometry::Color;
use image::Image;

/// A pattern to use as a source or mask for a drawing operation.
#[non_exhaustive]
pub enum Pattern<'a> {
    /// A solid color.
    SolidColor(Color<u8>),

    /// An image.
    Image(&'a dyn Image),

    /// Another surface.
    Surface(&'a mut dyn Drawable),
}
