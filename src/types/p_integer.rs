// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/p_integer.rs - Parser for the GenCAD p_integer data type.
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

use nom::character::char;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::sequence::preceded;
use nom::{IResult, ParseTo, Parser};

pub fn p_integer(s: &str) -> IResult<&str, u16> {
    map_res(preceded(opt(char('+')), digit1), |num: &str| {
        num.parse_to().ok_or(())
    })
    .parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        assert_eq!(p_integer("0"), Ok(("", 0u16)));
        assert_eq!(p_integer("+0"), Ok(("", 0u16)));
        assert_eq!(p_integer("1"), Ok(("", 1u16)));
        assert_eq!(p_integer("+1"), Ok(("", 1u16)));
        assert_eq!(p_integer("65535"), Ok(("", 65535u16)));
        assert_eq!(p_integer("+65535"), Ok(("", 65535u16)));
    }

    #[test]
    fn test_errors() {
        assert!(p_integer("-1").is_err());
        assert!(p_integer("65536").is_err());
    }
}
