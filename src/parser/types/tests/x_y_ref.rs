// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD x_y_ref data type.
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

use super::super::*;

use crate::types::XYRef;

#[test]
fn tests_standard() {
    // Examples from the standard
    assert_eq!(
        x_y_ref("1200 +3000"),
        Ok((
            "",
            XYRef {
                x: 1200.0,
                y: 3000.0,
            }
        ))
    );
    assert_eq!(
        x_y_ref("1.2005 0.0035"),
        Ok((
            "",
            XYRef {
                x: 1.2005,
                y: 0.0035,
            }
        ))
    );
}
