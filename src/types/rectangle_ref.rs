// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD rectangle_ref data type.
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

use super::{Number, XYRef};

use crate::serialization::ToGencadString;

/// Specifications for a rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangleRef {
    /// The origin of the rectangle.
    pub origin: XYRef,
    /// The x-dimension of the rectangle in [super::Dimension] units.
    pub x: Number,
    /// The y-dimension of the rectangle in [super::Dimension] units.
    pub y: Number,
}

impl ToGencadString for RectangleRef {
    fn to_gencad_string(&self) -> String {
        format!("{} {} {}", self.origin.to_gencad_string(), self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::types::rectangle_ref;

    #[test]
    fn test_serialization() {
        let rectangle = "1000 -200 20 -10";
        assert_eq!(
            rectangle.to_string(),
            rectangle_ref(rectangle).unwrap().1.to_gencad_string()
        );
    }
}
