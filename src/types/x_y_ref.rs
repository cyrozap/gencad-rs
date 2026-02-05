// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/x_y_ref.rs - Parser for the GenCAD x_y_ref data type.
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

use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

use crate::types::util::spaces;
use crate::types::{Number, number};

/// A pair of numbers defining the x and y coordinates of a point on or off board.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYRef {
    /// The x-coordinate.
    pub x: Number,
    /// The y-coordinate.
    pub y: Number,
}

impl XYRef {
    fn new(p: (Number, Number)) -> Self {
        let (x, y) = p;
        Self { x, y }
    }
}

pub fn x_y_ref(s: &str) -> IResult<&str, XYRef> {
    map(separated_pair(number, spaces, number), XYRef::new).parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_standard() {
        // Examples from the standard
        assert_eq!(
            x_y_ref("1200 +3000"),
            Ok((
                "",
                XYRef {
                    x: 1200.0,
                    y: 3000.0,
                }
            ))
        );
        assert_eq!(
            x_y_ref("1.2005 0.0035"),
            Ok((
                "",
                XYRef {
                    x: 1.2005,
                    y: 0.0035,
                }
            ))
        );
    }
}
