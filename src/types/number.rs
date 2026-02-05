// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/number.rs - Parser for the GenCAD number data type.
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

pub type Number = f32;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integers() {
        assert_eq!(number("1"), Ok(("", 1.0)));
        assert_eq!(number("+1"), Ok(("", 1.0)));
        assert_eq!(number("-1"), Ok(("", -1.0)));
    }

    #[test]
    fn test_decimals() {
        assert_eq!(number("0.1"), Ok(("", 0.1)));
        assert_eq!(number("+0.1"), Ok(("", 0.1)));
        assert_eq!(number("-0.1"), Ok(("", -0.1)));
        assert_eq!(number("+.1"), Ok(("", 0.1)));
        assert_eq!(number("-.1"), Ok(("", -0.1)));

        assert_eq!(number("1.0"), Ok(("", 1.0)));
        assert_eq!(number("+1.0"), Ok(("", 1.0)));
        assert_eq!(number("-1.0"), Ok(("", -1.0)));
        assert_eq!(number("+1."), Ok(("", 1.0)));
        assert_eq!(number("-1."), Ok(("", -1.0)));
    }

    #[test]
    fn test_scientific() {
        assert_eq!(number("2.99792458e8"), Ok(("", 299792458.0)));
        assert_eq!(number("2.99792458E8"), Ok(("", 299792458.0)));
        assert_eq!(number("+2.99792458e8"), Ok(("", 299792458.0)));
        assert_eq!(number("-2.99792458e8"), Ok(("", -299792458.0)));

        // Limits in the standard
        assert_eq!(number("3.4e-38"), Ok(("", 3.4e-38)));
        assert_eq!(number("3.4e38"), Ok(("", 3.4e38)));
    }

    #[test]
    fn test_errors() {
        // Unlike normal floating point numbers, the GenCAD number type does not support NaN/infinity.
        assert!(number("nan").is_err());
        assert!(number("infinity").is_err());
        assert!(number("inf").is_err());
    }
}
