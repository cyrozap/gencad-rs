// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD arc_ref data type.
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

use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::{IResult, Parser};

use super::util::spaces;
use super::{Number, XYRef, number, x_y_ref};

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

/// Specifications for a circular arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticalArcRef {
    /// The start coordinate of the arc.
    pub start: XYRef,
    /// The end coordinate of the arc.
    pub end: XYRef,
    /// The center of the circular arc.
    pub center: XYRef,
    /// Major radius of the ellipse.
    pub major_radius: Number,
    /// Minor radius of the ellipse.
    pub minor_radius: Number,
}

/// Specifications for an arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcRef {
    Circular(CircularArcRef),
    Elliptical(EllipticalArcRef),
}

impl ArcRef {
    fn new_circular(v: (XYRef, XYRef, XYRef)) -> Self {
        let (start, end, center) = v;
        Self::Circular(CircularArcRef { start, end, center })
    }

    fn new_elliptical(v: (XYRef, XYRef, XYRef, Number, Number)) -> Self {
        let (start, end, center, major_radius, minor_radius) = v;
        Self::Elliptical(EllipticalArcRef {
            start,
            end,
            center,
            major_radius,
            minor_radius,
        })
    }
}

pub fn arc_ref(s: &str) -> IResult<&str, ArcRef> {
    alt((
        map(
            (
                x_y_ref,
                preceded(spaces, x_y_ref),
                preceded(spaces, x_y_ref),
                preceded(spaces, number),
                preceded(spaces, number),
            ),
            ArcRef::new_elliptical,
        ),
        map(
            (
                x_y_ref,
                preceded(spaces, x_y_ref),
                preceded(spaces, x_y_ref),
            ),
            ArcRef::new_circular,
        ),
    ))
    .parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular() {
        assert_eq!(
            arc_ref("1000 -200 1000 200 1000 0"),
            Ok((
                "",
                ArcRef::Circular(CircularArcRef {
                    start: XYRef {
                        x: 1000.0,
                        y: -200.0
                    },
                    end: XYRef {
                        x: 1000.0,
                        y: 200.0
                    },
                    center: XYRef { x: 1000.0, y: 0.0 }
                })
            ))
        );
    }
    #[test]
    fn test_elliptical() {
        assert_eq!(
            arc_ref("1000 -200 1000 200 1000 0 1000 200"),
            Ok((
                "",
                ArcRef::Elliptical(EllipticalArcRef {
                    start: XYRef {
                        x: 1000.0,
                        y: -200.0
                    },
                    end: XYRef {
                        x: 1000.0,
                        y: 200.0
                    },
                    center: XYRef { x: 1000.0, y: 0.0 },
                    major_radius: 1000.0,
                    minor_radius: 200.0,
                })
            ))
        );
    }
}
