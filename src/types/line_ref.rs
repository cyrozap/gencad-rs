// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD line_ref data type.
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

use super::XYRef;

use crate::serialization::ToGencadString;

/// Specifications for a line.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineRef {
    /// The start of the line.
    pub start: XYRef,
    /// The end of the line.
    pub end: XYRef,
}

impl ToGencadString for LineRef {
    fn to_gencad_string(&self) -> String {
        format!(
            "{} {}",
            self.start.to_gencad_string(),
            self.end.to_gencad_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::types::line_ref;

    #[test]
    fn test_serialization() {
        let line = "1000 -200 200 -1000";
        assert_eq!(
            line.to_string(),
            line_ref(line).unwrap().1.to_gencad_string()
        );
    }
}
