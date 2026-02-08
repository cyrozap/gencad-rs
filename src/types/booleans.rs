// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/booleans.rs - Parser for the GenCAD boolean data types.
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

macro_rules! bool_parser {
    ($name:ident, $true_value:expr) => {
        pub fn $name(s: &str) -> IResult<&str, bool> {
            alt((value(false, tag("0")), value(true, tag($true_value)))).parse(s)
        }
    };
}

bool_parser!(filled_ref, "YES");
//bool_parser!(flip, "FLIP");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filled_ref() {
        assert_eq!(filled_ref("0"), Ok(("", false)));
        assert_eq!(filled_ref("YES"), Ok(("", true)));
    }

    // #[test]
    // fn test_flip() {
    //     assert_eq!(flip("0"), Ok(("", false)));
    //     assert_eq!(flip("FLIP"), Ok(("", true)));
    // }
}
