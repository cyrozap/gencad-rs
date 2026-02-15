// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD number data type.
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

use nom::number::complete::recognize_float;
use nom::{IResult, ParseTo, Parser};

use crate::types::Number;

pub fn number(s: &str) -> IResult<&str, Number> {
    let (remaining, output) = recognize_float.parse(s)?;
    match output.parse_to() {
        Some(f) => Ok((remaining, f)),
        None => Err(nom::Err::Error(nom::error::Error::new(
            remaining,
            nom::error::ErrorKind::Float,
        ))),
    }
}
