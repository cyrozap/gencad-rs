// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD dimension data type.
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
use nom::combinator::{map, value};
use nom::sequence::separated_pair;
use nom::{IResult, Parser};

use super::p_integer;
use super::util::spaces;

use crate::types::Dimension;

impl Dimension {
    fn from_pair(user: (&str, u16)) -> Self {
        let (k, v) = user;
        match k {
            "USER" => Self::User(v),
            "USERM" => Self::UserM(v),
            "USERMM" => Self::UserMm(v),
            _ => panic!("This should never happen!"),
        }
    }
}

pub fn dimension(s: &str) -> IResult<&str, Dimension> {
    alt((
        alt((
            value(Dimension::Inch, tag("INCH")),
            value(Dimension::Thou, tag("THOU")),
            value(Dimension::Mm100, tag("MM100")),
            value(Dimension::Mm, tag("MM")),
        )),
        map(
            alt((
                separated_pair(tag("USERMM"), spaces, p_integer),
                separated_pair(tag("USERM"), spaces, p_integer),
                separated_pair(tag("USER"), spaces, p_integer),
            )),
            Dimension::from_pair,
        ),
    ))
    .parse(s)
}
