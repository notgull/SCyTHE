# scythe

This workspace contains several components of the `scythe` project, which is a set of libraries for drawing and manipulating 2D graphics.

Specifically, there are three important crates.

* `scythe` is the core library, which provides the `Canvas` type and the `Drawable` trait.
* `sythe-image` uses the [`tiny-skia`] crate to draw to raw images.
* `scythic` is for drawing to windowed surfaces via [`raw-window-handle`].

[`tiny-skia`]: https://docs.rs/tiny-skia
[`raw-window-handle`]: https://docs.rs/raw-window-handle

## License

Copyright 2023 John Nunley

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
