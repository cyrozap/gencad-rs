// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD attrib_ref data type.
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

use crate::impl_to_gencad_string_for_vec;
use crate::serialization::ToGencadString;

/// Additional data in a section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    /// The category of the attribute. Can be used to group a set of attributes.
    pub category: String,
    /// The name of the attribute.
    pub name: String,
    /// The attribute data.
    pub data: String,
}

impl ToGencadString for Attribute {
    fn to_gencad_string(&self) -> String {
        format!(
            "ATTRIBUTE {} {} {}",
            self.category.to_gencad_string(),
            self.name.to_gencad_string(),
            self.data.to_gencad_string()
        )
    }
}

impl_to_gencad_string_for_vec!(Attribute);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::types::attrib_ref;

    #[test]
    fn test_serialization() {
        let attr = r#"alpha m_part "BIS 9600""#;
        assert_eq!(
            format!("ATTRIBUTE {}", attr),
            attrib_ref(attr).unwrap().1.to_gencad_string()
        );
    }
}
