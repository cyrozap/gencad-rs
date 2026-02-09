// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD arc_ref data type.
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

/// Specifications for a circular arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularArcRef {
    /// The start coordinate of the arc.
    pub start: XYRef,
    /// The end coordinate of the arc.
    pub end: XYRef,
    /// The center of the circular arc.
    pub center: XYRef,
}

/// Specifications for an elliptical arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticalArcRef {
    /// The start coordinate of the arc.
    pub start: XYRef,
    /// The end coordinate of the arc.
    pub end: XYRef,
    /// The center of the circular arc.
    pub center: XYRef,
    /// Major radius of the ellipse in [super::Dimension] units.
    pub major_radius: Number,
    /// Minor radius of the ellipse in [super::Dimension] units.
    pub minor_radius: Number,
}

/// Specifications for an arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcRef {
    Circular(CircularArcRef),
    Elliptical(EllipticalArcRef),
}

impl ToGencadString for ArcRef {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Circular(arc) => format!(
                "{} {} {}",
                arc.start.to_gencad_string(),
                arc.end.to_gencad_string(),
                arc.center.to_gencad_string()
            ),
            Self::Elliptical(arc) => format!(
                "{} {} {} {} {}",
                arc.start.to_gencad_string(),
                arc.end.to_gencad_string(),
                arc.center.to_gencad_string(),
                arc.major_radius,
                arc.minor_radius
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::types::arc_ref;

    #[test]
    fn test_serialization() {
        let circular = "1000 -200 1000 200 1000 0";
        assert_eq!(
            circular.to_string(),
            arc_ref(circular).unwrap().1.to_gencad_string()
        );

        let elliptical = "1000 -200 1000 200 1000 0 1000 200";
        assert_eq!(
            elliptical.to_string(),
            arc_ref(elliptical).unwrap().1.to_gencad_string()
        );
    }
}
