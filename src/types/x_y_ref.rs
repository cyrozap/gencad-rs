// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD x_y_ref data type.
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

use super::Number;

use crate::serialization::ToGencadString;

/// A pair of numbers defining the x and y coordinates of a point on or off board.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYRef {
    /// The x-coordinate in [super::Dimension] units.
    pub x: Number,
    /// The y-coordinate in [super::Dimension] units.
    pub y: Number,
}

impl ToGencadString for XYRef {
    fn to_gencad_string(&self) -> String {
        format!("{} {}", self.x, self.y)
    }
}
