// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/mirror.rs - Parser for the GenCAD mirror data type.
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

/// Part mirror status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirror {
    /// Not mirrored.
    Not,
    /// Mirrored about the x-axis.
    MirrorX,
    /// Mirrored about the y-axis.
    MirrorY,
}

pub fn mirror(s: &str) -> IResult<&str, Mirror> {
    alt((
        value(Mirror::Not, tag("0")),
        value(Mirror::MirrorX, tag("MIRRORX")),
        value(Mirror::MirrorY, tag("MIRRORY")),
    ))
    .parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_standard() {
        assert_eq!(mirror("0"), Ok(("", Mirror::Not)));
        assert_eq!(mirror("MIRRORX"), Ok(("", Mirror::MirrorX)));
        assert_eq!(mirror("MIRRORY"), Ok(("", Mirror::MirrorY)));
    }
}
