// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD text_par data type.
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
use nom::sequence::preceded;
use nom::{IResult, Parser};

use super::util::spaces;
use super::{layer, mirror, number, rectangle_ref, rot, string};

use crate::types::{Layer, Mirror, Number, RectangleRef, TextPar};

impl TextPar {
    fn new(v: (Number, Number, Mirror, Layer, String, RectangleRef)) -> Self {
        let (text_size, rotation, mirror, layer, text, area) = v;
        Self {
            text_size,
            rotation,
            mirror,
            layer,
            text,
            area,
        }
    }
}

pub fn text_par(s: &str) -> IResult<&str, TextPar> {
    map(
        (
            number,
            preceded(spaces, rot),
            preceded(spaces, mirror),
            preceded(spaces, layer),
            preceded(spaces, string),
            preceded(spaces, rectangle_ref),
        ),
        TextPar::new,
    )
    .parse(s)
}
