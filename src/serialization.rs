// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/serialization.rs - Serialization trait for the GenCAD format.
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

pub trait ToGencadString {
    fn to_gencad_string(&self) -> String;
}

#[macro_export]
macro_rules! impl_to_gencad_string_for_vec {
    ($t:ty) => {
        impl ToGencadString for Vec<$t> {
            fn to_gencad_string(&self) -> String {
                self.iter()
                    .map(|item| item.to_gencad_string())
                    .collect::<Vec<_>>()
                    .join("\r\n")
            }
        }
    };
}

#[macro_export]
macro_rules! impl_to_gencad_string_for_section {
    ($t:ty, $header:expr, $footer:expr) => {
        impl ToGencadString for Vec<$t> {
            fn to_gencad_string(&self) -> String {
                let mut lines = Vec::new();
                lines.push($header.to_string());
                lines.extend(self.iter().map(|item| item.to_gencad_string()));
                lines.push($footer.to_string());
                lines.join("\r\n")
            }
        }
    };
}
