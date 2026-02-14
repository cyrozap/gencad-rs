// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD text_par data type.
 *  Copyright (C) 2026  Forest Crossman <cyrozap@gmail.com>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::{Layer, Mirror, Number, RectangleRef};

/// Specifications for a text object.
#[derive(Debug, Clone, PartialEq)]
pub struct TextPar {
    /// The text size in [super::Dimension] units.
    pub text_size: Number,
    /// The rotation of the text in degrees.
    pub rotation: Number,
    /// The mirror status of the text.
    pub mirror: Mirror,
    /// The layer this text belongs to.
    pub layer: Layer,
    /// The text itself.
    pub text: String,
    /// The rectangular area the text must fit within.
    pub area: RectangleRef,
}
