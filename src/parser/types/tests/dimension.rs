// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD dimension data type.
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

use crate::types::Dimension;

#[test]
fn tests_standard() {
    assert_eq!(dimension("INCH"), Ok(("", Dimension::Inch)));
    assert_eq!(dimension("THOU"), Ok(("", Dimension::Thou)));
    assert_eq!(dimension("MM"), Ok(("", Dimension::Mm)));
    assert_eq!(dimension("MM100"), Ok(("", Dimension::Mm100)));
    assert_eq!(dimension("USER 1"), Ok(("", Dimension::User(1))));
    assert_eq!(dimension("USER +1"), Ok(("", Dimension::User(1))));
    assert_eq!(dimension("USER 65535"), Ok(("", Dimension::User(65535))));
    assert_eq!(dimension("USERM 1"), Ok(("", Dimension::UserM(1))));
    assert_eq!(dimension("USERM +1"), Ok(("", Dimension::UserM(1))));
    assert_eq!(dimension("USERM 65535"), Ok(("", Dimension::UserM(65535))));
    assert_eq!(dimension("USERMM 1"), Ok(("", Dimension::UserMm(1))));
    assert_eq!(dimension("USERMM +1"), Ok(("", Dimension::UserMm(1))));
    assert_eq!(
        dimension("USERMM 65535"),
        Ok(("", Dimension::UserMm(65535)))
    );
}
