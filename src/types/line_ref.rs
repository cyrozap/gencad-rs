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

/// Specifications for a line.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineRef {
    /// The start of the line.
    pub start: XYRef,
    /// The end of the line.
    pub end: XYRef,
}
