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
