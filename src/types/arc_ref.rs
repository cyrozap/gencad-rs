// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD arc_ref data type.
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

use super::{Number, XYRef};

/// Specifications for a circular arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularArcRef {
    /// The start coordinate of the arc.
    pub start: XYRef,
    /// The end coordinate of the arc.
    pub end: XYRef,
    /// The center of the circular arc.
    pub center: XYRef,
}

/// Specifications for an elliptical arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipticalArcRef {
    /// The start coordinate of the arc.
    pub start: XYRef,
    /// The end coordinate of the arc.
    pub end: XYRef,
    /// The center of the circular arc.
    pub center: XYRef,
    /// Major radius of the ellipse in [super::Dimension] units.
    pub major_radius: Number,
    /// Minor radius of the ellipse in [super::Dimension] units.
    pub minor_radius: Number,
}

/// Specifications for an arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArcRef {
    Circular(CircularArcRef),
    Elliptical(EllipticalArcRef),
}
