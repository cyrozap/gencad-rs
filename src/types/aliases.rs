// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/types/aliases.rs - Parser functions that alias other GenCAD type parsers.
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

use nom::IResult;

use crate::types::{Number, number, string};

macro_rules! string_alias {
    ($name:ident) => {
        /// Alias of [string].
        pub fn $name(s: &str) -> IResult<&str, String> {
            string(s)
        }
    };
}

macro_rules! number_alias {
    ($name:ident) => {
        /// Alias of [number].
        pub fn $name(s: &str) -> IResult<&str, Number> {
            number(s)
        }
    };
}

string_alias!(artwork_name);

string_alias!(component_name);

number_alias!(drill_size);

string_alias!(fid_name);

string_alias!(filename);

number_alias!(height);

string_alias!(pad_name);

string_alias!(part_name);

string_alias!(pin_name);

string_alias!(probe);

number_alias!(rot);

string_alias!(shape_name);

string_alias!(sig_name);

string_alias!(tan);

string_alias!(testpad_name);

string_alias!(tin);

string_alias!(tp_name);

string_alias!(track_name);

number_alias!(track_width);

string_alias!(via_name);
