// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/mod.rs - Module to contain GenCAD section parsers.
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

pub mod board;
pub mod components;
pub mod devices;
pub mod header;
pub mod pads;
pub mod padstacks;
pub mod shapes;
pub mod signals;
pub mod unknown;
