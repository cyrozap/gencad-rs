// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/header.rs - Parser for the GenCAD HEADER section.
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

use crate::parser::KeywordParam;
use crate::types::{
    Attribute, Dimension, Number, XYRef, attrib_ref, dimension, number, string, x_y_ref,
};

/// Represents the `HEADER` section of a GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    /// The GenCAD version.
    pub gencad_version: Number,
    /// The originator of the GenCAD file.
    pub user: String,
    /// The board drawing number or title.
    pub drawing: String,
    /// The revision, issue, or modification status of the board.
    pub revision: String,
    /// The dimensional units used in the file.
    pub units: Dimension,
    /// The CAD coordinates of the origin of the board.
    pub origin: XYRef,
    /// The file change version.
    pub intertrack: Number,
    /// A list of attributes associated with the header.
    pub attributes: Vec<Attribute>,
}

impl Header {
    pub fn new(params: &[KeywordParam]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut gencad_version = None;
        let mut user = None;
        let mut drawing = None;
        let mut revision = None;
        let mut units = None;
        let mut origin = None;
        let mut intertrack = None;
        let mut attributes = Vec::new();

        for param in params {
            match param.keyword.as_str() {
                "GENCAD" => {
                    if gencad_version.is_none() {
                        let (_, value) =
                            number(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        gencad_version = Some(value);
                    }
                }
                "USER" => {
                    if user.is_none() {
                        let (_, value) =
                            string(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        user = Some(value);
                    }
                }
                "DRAWING" => {
                    if drawing.is_none() {
                        let (_, value) =
                            string(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        drawing = Some(value);
                    }
                }
                "REVISION" => {
                    if revision.is_none() {
                        let (_, value) =
                            string(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        revision = Some(value);
                    }
                }
                "UNITS" => {
                    if units.is_none() {
                        let (_, value) =
                            dimension(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        units = Some(value);
                    }
                }
                "ORIGIN" => {
                    if origin.is_none() {
                        let (_, value) =
                            x_y_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        origin = Some(value);
                    }
                }
                "INTERTRACK" => {
                    if intertrack.is_none() {
                        let (_, value) =
                            number(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        intertrack = Some(value);
                    }
                }
                "ATTRIBUTE" => {
                    let (_, value) =
                        attrib_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    attributes.push(value);
                }
                _ => {}
            }
        }

        let gencad_version = gencad_version.ok_or("missing GENCAD")?;
        let user = user.ok_or("missing USER")?;
        let drawing = drawing.ok_or("missing DRAWING")?;
        let revision = revision.ok_or("missing REVISION")?;
        let units = units.ok_or("missing UNITS")?;
        let origin = origin.ok_or("missing ORIGIN")?;
        let intertrack = intertrack.ok_or("missing INTERTRACK")?;

        Ok(Self {
            gencad_version,
            user,
            drawing,
            revision,
            units,
            origin,
            intertrack,
            attributes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_header() {
        let params = vec![
            KeywordParam {
                keyword: "GENCAD".to_string(),
                parameter: "1.4".to_string(),
            },
            KeywordParam {
                keyword: "USER".to_string(),
                parameter: "\"Mitron Europe Ltd. Serial Number 00001\"".to_string(),
            },
            KeywordParam {
                keyword: "DRAWING".to_string(),
                parameter: "\"Modem C100 motherboard 1234-5678\"".to_string(),
            },
            KeywordParam {
                keyword: "REVISION".to_string(),
                parameter: "\"Rev 566g 20th September 1990\"".to_string(),
            },
            KeywordParam {
                keyword: "UNITS".to_string(),
                parameter: "USER 1200".to_string(),
            },
            KeywordParam {
                keyword: "ORIGIN".to_string(),
                parameter: "0 0".to_string(),
            },
            KeywordParam {
                keyword: "INTERTRACK".to_string(),
                parameter: "0".to_string(),
            },
            KeywordParam {
                keyword: "ATTRIBUTE".to_string(),
                parameter: "alpha m_part \"BIS 9600\"".to_string(),
            },
            KeywordParam {
                keyword: "ATTRIBUTE".to_string(),
                parameter: "alpha m_desc \"Issue 2\"".to_string(),
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
                keyword: "GENCAD".to_string(),
                parameter: "1.4".to_string(),
            },
            KeywordParam {
                keyword: "USER".to_string(),
                parameter: "\"Mitron Europe Ltd. Serial Number 00001\"".to_string(),
            },
            KeywordParam {
                keyword: "DRAWING".to_string(),
                parameter: "\"Modem C100 motherboard 1234-5678\"".to_string(),
            },
            KeywordParam {
                keyword: "REVISION".to_string(),
                parameter: "\"Rev 566g 20th September 1990\"".to_string(),
            },
            KeywordParam {
                keyword: "UNITS".to_string(),
                parameter: "USER 1200".to_string(),
            },
            KeywordParam {
                keyword: "ORIGIN".to_string(),
                parameter: "0 0".to_string(),
            },
            KeywordParam {
                keyword: "INTERTRACK".to_string(),
                parameter: "0".to_string(),
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
            params.retain(|p| p.keyword != *keyword);

            let result = Header::new(&params);
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().to_string(),
                format!("missing {}", keyword)
            );
        }
    }
}
