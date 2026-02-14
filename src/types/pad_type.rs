// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD pad_type data type.
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

/// Pad types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadType {
    /// A solid rectangular pad with semicircular ends (obround).
    Finger,
    /// A solid circle.
    Round,
    /// Any shape ring of copper (width unspecified).
    Annular,
    /// A solid rectangular pad with one semicircular end.
    Bullet,
    /// A solid rectangle or square.
    Rectangular,
    /// A solid hexagonal pad with equal length sides.
    Hexagon,
    /// A solid octagonal pad with equal length sides.
    Octagon,
    /// A solid polygon defined with LINES and ARCS.
    Polygon,
    /// Unknown shape; not defined with LINES and ARCS.
    Unknown,
}
