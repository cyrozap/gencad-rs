// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD pad_type data type.
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
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::{IResult, Parser};

use crate::types::PadType;

pub fn pad_type(s: &str) -> IResult<&str, PadType> {
    alt((
        value(PadType::Finger, tag("FINGER")),
        value(PadType::Round, tag("ROUND")),
        value(PadType::Annular, tag("ANNULAR")),
        value(PadType::Bullet, tag("BULLET")),
        value(PadType::Rectangular, tag("RECTANGULAR")),
        value(PadType::Hexagon, tag("HEXAGON")),
        value(PadType::Octagon, tag("OCTAGON")),
        value(PadType::Polygon, tag("POLYGON")),
        value(PadType::Unknown, tag("UNKNOWN")),
    ))
    .parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_standard() {
        assert_eq!(pad_type("FINGER"), Ok(("", PadType::Finger)));
        assert_eq!(pad_type("ROUND"), Ok(("", PadType::Round)));
        assert_eq!(pad_type("ANNULAR"), Ok(("", PadType::Annular)));
        assert_eq!(pad_type("BULLET"), Ok(("", PadType::Bullet)));
        assert_eq!(pad_type("RECTANGULAR"), Ok(("", PadType::Rectangular)));
        assert_eq!(pad_type("HEXAGON"), Ok(("", PadType::Hexagon)));
        assert_eq!(pad_type("OCTAGON"), Ok(("", PadType::Octagon)));
        assert_eq!(pad_type("POLYGON"), Ok(("", PadType::Polygon)));
        assert_eq!(pad_type("UNKNOWN"), Ok(("", PadType::Unknown)));
    }
}
