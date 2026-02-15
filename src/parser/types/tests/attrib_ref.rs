// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD attrib_ref data type.
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

use crate::types::Attribute;

#[test]
fn test_ok() {
    assert_eq!(
        attrib_ref("a b c"),
        Ok((
            "",
            Attribute {
                category: "a".to_string(),
                name: "b".to_string(),
                data: "c".to_string()
            }
        ))
    );

    // Examples from the standard
    assert_eq!(
        attrib_ref(r#"alpha m_part "BIS 9600""#),
        Ok((
            "",
            Attribute {
                category: "alpha".to_string(),
                name: "m_part".to_string(),
                data: "BIS 9600".to_string()
            }
        ))
    );
    assert_eq!(
        attrib_ref(r#"alpha m_desc "Issue 2""#),
        Ok((
            "",
            Attribute {
                category: "alpha".to_string(),
                name: "m_desc".to_string(),
                data: "Issue 2".to_string()
            }
        ))
    );
}
