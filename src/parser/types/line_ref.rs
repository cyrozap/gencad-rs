// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD line_ref data type.
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
use super::x_y_ref;

use crate::types::{LineRef, XYRef};

impl LineRef {
    fn new(v: (XYRef, XYRef)) -> Self {
        let (start, end) = v;
        Self { start, end }
    }
}

pub fn line_ref(s: &str) -> IResult<&str, LineRef> {
    map((x_y_ref, preceded(spaces, x_y_ref)), LineRef::new).parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(
            line_ref("1000 -200 200 -1000"),
            Ok((
                "",
                LineRef {
                    start: XYRef {
                        x: 1000.0,
                        y: -200.0
                    },
                    end: XYRef {
                        x: 200.0,
                        y: -1000.0
                    },
                }
            ))
        );
    }
}
