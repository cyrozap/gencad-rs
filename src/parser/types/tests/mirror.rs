// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD mirror data type.
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

use crate::types::Mirror;

#[test]
fn tests_standard() {
    assert_eq!(mirror("0"), Ok(("", Mirror::Not)));
    assert_eq!(mirror("MIRRORX"), Ok(("", Mirror::MirrorX)));
    assert_eq!(mirror("MIRRORY"), Ok(("", Mirror::MirrorY)));
}
