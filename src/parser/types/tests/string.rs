// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD string data type.
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

#[test]
fn test_string() {
    // Unquoted strings
    assert_eq!(string(""), Ok(("", "".to_string())));
    assert_eq!(string("A"), Ok(("", "A".to_string())));
    assert_eq!(string(r#"A""#), Ok(("", r#"A""#.to_string())));

    // Quoted strings
    assert_eq!(string(r#""A\"""#), Ok(("", "A\"".to_string())));
    assert_eq!(
        string(r#""ABCD EFGH \IJKL\ \"MNOP\" QRST WXYZ""#),
        Ok(("", "ABCD EFGH \\IJKL\\ \"MNOP\" QRST WXYZ".to_string()))
    );

    // Examples from the standard
    assert_eq!(
        string(r#""Mitron Europe Ltd. Serial Number 00001""#),
        Ok(("", "Mitron Europe Ltd. Serial Number 00001".to_string()))
    );
    assert_eq!(
        string(r#""Modem C100 motherboard 1234-5678""#),
        Ok(("", "Modem C100 motherboard 1234-5678".to_string()))
    );
    assert_eq!(
        string(r#""Rev 566g 20th September 1990""#),
        Ok(("", "Rev 566g 20th September 1990".to_string()))
    );
}
