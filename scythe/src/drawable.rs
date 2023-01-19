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

use crate::clip::Clip;
use crate::pattern::Pattern;
use crate::Error;
use geometry::composite::CompositeOperation;
use geometry::region::Region;

/// A surface that can be drawn on.
pub trait Drawable {
    /// Paint a pattern to the surface.
    fn paint(
        &mut self,
        op: CompositeOperation,
        pattern: &mut Pattern<'_>,
        clip: Clip<'_>,
    ) -> Result<(), Error>;

    /// Paint a pattern to the surface using another pattern as a mask.
    fn mask(
        &mut self,
        op: CompositeOperation,
        pattern: &mut Pattern<'_>,
        mask: &mut Pattern<'_>,
        clip: Clip<'_>,
    ) -> Result<(), Error>;
}
