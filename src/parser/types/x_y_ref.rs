// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD x_y_ref data type.
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

use super::number;
use super::util::spaces;

use crate::types::{Number, XYRef};

impl XYRef {
    fn new(p: (Number, Number)) -> Self {
        let (x, y) = p;
        Self { x, y }
    }
}

pub fn x_y_ref(s: &str) -> IResult<&str, XYRef> {
    map(separated_pair(number, spaces, number), XYRef::new).parse(s)
}
