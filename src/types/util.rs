// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/util.rs - Parser helper functions that are not GenCAD types.
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

use nom::bytes::complete::take_while1;
use nom::{IResult, Parser};

pub fn spaces(s: &str) -> IResult<&str, &str> {
    take_while1(|c| c == ' ').parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spaces() {
        assert_eq!(spaces("     "), Ok(("", "     ")));
        assert_eq!(spaces("   a  "), Ok(("a  ", "   ")));

        assert!(spaces("").is_err());
        assert!(spaces("a").is_err());
    }
}
