// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD HEADER section.
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

use super::super::header::*;

use crate::parser::KeywordParam;
use crate::types::{Attribute, Dimension, XYRef};

#[test]
fn test_valid_header() {
    let params = vec![
        KeywordParam {
            keyword: "GENCAD",
            parameter: "1.4",
        },
        KeywordParam {
            keyword: "USER",
            parameter: "\"Mitron Europe Ltd. Serial Number 00001\"",
        },
        KeywordParam {
            keyword: "DRAWING",
            parameter: "\"Modem C100 motherboard 1234-5678\"",
        },
        KeywordParam {
            keyword: "REVISION",
            parameter: "\"Rev 566g 20th September 1990\"",
        },
        KeywordParam {
            keyword: "UNITS",
            parameter: "USER 1200",
        },
        KeywordParam {
            keyword: "ORIGIN",
            parameter: "0 0",
        },
        KeywordParam {
            keyword: "INTERTRACK",
            parameter: "0",
        },
        KeywordParam {
            keyword: "ATTRIBUTE",
            parameter: "alpha m_part \"BIS 9600\"",
        },
        KeywordParam {
            keyword: "ATTRIBUTE",
            parameter: "alpha m_desc \"Issue 2\"",
        },
    ];

    let header = Header::new(&params).unwrap();

    assert_eq!(header.gencad_version, 1.4);
    assert_eq!(header.user, "Mitron Europe Ltd. Serial Number 00001");
    assert_eq!(header.drawing, "Modem C100 motherboard 1234-5678");
    assert_eq!(header.revision, "Rev 566g 20th September 1990");
    assert_eq!(header.units, Dimension::User(1200));
    assert_eq!(header.origin, XYRef { x: 0.0, y: 0.0 });
    assert_eq!(header.intertrack, 0.0);
    assert_eq!(
        header.attributes,
        vec![
            Attribute {
                category: "alpha".to_string(),
                name: "m_part".to_string(),
                data: "BIS 9600".to_string()
            },
            Attribute {
                category: "alpha".to_string(),
                name: "m_desc".to_string(),
                data: "Issue 2".to_string()
            }
        ]
    );
}

#[test]
fn test_missing_required_keywords() {
    let base_params = vec![
        KeywordParam {
            keyword: "GENCAD",
            parameter: "1.4",
        },
        KeywordParam {
            keyword: "USER",
            parameter: "\"Mitron Europe Ltd. Serial Number 00001\"",
        },
        KeywordParam {
            keyword: "DRAWING",
            parameter: "\"Modem C100 motherboard 1234-5678\"",
        },
        KeywordParam {
            keyword: "REVISION",
            parameter: "\"Rev 566g 20th September 1990\"",
        },
        KeywordParam {
            keyword: "UNITS",
            parameter: "USER 1200",
        },
        KeywordParam {
            keyword: "ORIGIN",
            parameter: "0 0",
        },
        KeywordParam {
            keyword: "INTERTRACK",
            parameter: "0",
        },
    ];

    let missing_keywords = [
        "GENCAD",
        "USER",
        "DRAWING",
        "REVISION",
        "UNITS",
        "ORIGIN",
        "INTERTRACK",
    ];

    for keyword in missing_keywords {
        let mut params = base_params.clone();
        params.retain(|p| p.keyword != keyword);

        let result = Header::new(&params);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("missing {}", keyword)
        );
    }
}
