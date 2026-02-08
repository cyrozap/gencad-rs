// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/parser.rs - Parser library for GenCAD files.
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

/*!
 * # `parser` Module
 *
 * This module provides functionality to parse GenCAD files into structured
 * data.
 *
 * ## Usage Example
 *
 * ```no_run
 * use std::fs::File;
 * use std::io::BufReader;
 *
 * use gencad::parser::ParsedGencadFile;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     // Open the file
 *     let file = File::open("example.cad")?;
 *     let reader = BufReader::new(file);
 *
 *     // Parse the file
 *     let parsed = ParsedGencadFile::new(reader)?;
 *
 *     // Access parsed data
 *     for section in parsed.sections {
 *         println!("{}", section.name);
 *     }
 *
 *     Ok(())
 * }
 * ```
 */

use nom::bytes::complete::{is_a, tag, take_till, take_while, take_while1};
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::{AsChar, IResult, Parser};

fn take_newlines(input: &[u8]) -> IResult<&[u8], &[u8]> {
    // Need to consume CR until first LF, then consume all following CRs and LFs
    let (remaining, _) = (
        take_while(|c| c == b'\r'),
        take_while1(AsChar::is_newline),
        take_while(|c| c == b'\r' || c == b'\n'),
    )
        .parse(input)?;
    Ok((remaining, &[]))
}

fn keyword(input: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

fn section_start(input: &[u8]) -> IResult<&[u8], &str> {
    let (remaining, output) = (
        tag(b"$".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
        .parse(input)?;
    let keyword = output.1;
    Ok((remaining, keyword))
}

fn section_end(input: &[u8]) -> IResult<&[u8], &str> {
    let (remaining, output) = (
        tag(b"$END".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
        .parse(input)?;
    let keyword = output.1;
    Ok((remaining, keyword))
}

/// A keyword/parameter pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordParam {
    /// The keyword that determines how to interpret the parameter.
    pub keyword: String,
    /// The parameter associated with the keyword.
    pub parameter: String,
}

impl KeywordParam {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, output) = (
            map_res(keyword, str::from_utf8),
            tag(b" ".as_slice()),
            map_res(take_till(|c| c == b'\r' || c == b'\n'), str::from_utf8),
            take_newlines,
        )
            .parse(input)?;
        let keyword = output.0.to_string();
        let parameter = output.2.to_string();
        let kp = Self { keyword, parameter };
        Ok((remaining, kp))
    }
}

/// A section in the GenCAD file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// The name of the section.
    pub name: String,
    /// List of parameters in the section.
    pub parameters: Vec<KeywordParam>,
}

impl Section {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, output) =
            (section_start, many0(KeywordParam::parse), section_end).parse(input)?;
        let name = output.0.to_string();
        let parameters = output.1;

        // TODO: Match section end name with section start name?

        Ok((remaining, Self { name, parameters }))
    }
}

/// A fully parsed GenCAD file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedGencadFile {
    /// The parsed sections of the file.
    pub sections: Vec<Section>,
}

impl ParsedGencadFile {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, sections) = many1(Section::parse).parse(input)?;

        Ok((remaining, Self { sections }))
    }

    /// Parses a GenCAD file into a structured format.
    ///
    /// # Arguments
    ///
    /// * `reader` - A reader over the file data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed file or an error.
    pub fn new<R: std::io::Read>(mut reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let (_, parsed) = Self::parse(&buffer).map_err(|err| err.to_owned())?;

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = vec![
            "$HEADER",
            "GENCAD 1.4",
            "USER \"Mitron Europe Ltd. Serial Number 00001\"",
            "DRAWING \"Modem C100 motherboard 1234-5678\"",
            "REVISION \"Rev 566g 20th September 1990\"",
            "UNITS USER 1200",
            "ORIGIN 0 0",
            "INTERTRACK 0",
            "ATTRIBUTE alpha m_part \"BIS 9600\"",
            "ATTRIBUTE alpha m_desc \"Issue 2\"",
            "$ENDHEADER",
            "",
            "$BOARD",
            "LINE 1000 2000 1200 2000",
            "ARC 1200 2000 1200 3000 1180 2500",
            "LINE 1200 3000 1000 3000",
            "LINE 1000 3000 1000 2000",
            "CUTOUT TRANSFORMER_HOLE",
            "CIRCLE 1180 2500 20",
            "ATTRIBUTE board mill \"tool 255\"",
            "MASK Fixture_1 TOP",
            "LINE 1005 2005 1195 2005",
            "ARC 1195 2005 1195 2995 1195 2500",
            "LINE 1195 2995 1005 2995",
            "LINE 1005 2995 1005 2005",
            "ARTWORK ORIGIN_MARKER TOP",
            "TRACK 10",
            "FILLED YES",
            "LINE -100 0 100 0",
            "LINE 0 -100 0 100",
            "$ENDBOARD",
            "",
            "$PADS",
            "PAD p0101 FINGER 32",
            "LINE 100 50 -100 50",
            "ARC -100 50 -100 -50 -100 0",
            "LINE -100 -50 100 -50",
            "ARC 100 -50 100 50 100 0",
            "PAD p1053 ROUND 20",
            "CIRCLE 0 0 30",
            "PAD p2034 BULLET 32",
            "ARC 0 -50 0 50 0 0",
            "LINE 0 50 -100 50",
            "LINE -100 50 -100 -50",
            "LINE -100 -50 0 -50",
            "PAD d_hole_50 ROUND 50",
            "CIRCLE 0 0 25",
            "PAD 3 RECTANGULAR 0",
            "RECTANGLE -5.2 -5.2 10.4 10.4",
            "$ENDPADS",
            "",
            "$SHAPES",
            "SHAPE CAP_SUPPRESS_TYPE_____24",
            "LINE -1000 200 -1000 -200",
            "LINE -1000 -200 1000 -200",
            "ARC 1000 -200 1000 200 1000 0",
            "LINE 1000 200 -1000 200",
            "PIN 1 p102_4 -100 100 TOP 315 0",
            "PIN 1 s106_6 -100 100 BOTTOM 315 MIRRORX",
            "PIN 2 p102_4 100 -100 TOP 135 0",
            "PIN 2 s106_6 100 -100 BOTTOM 135 MIRRORX",
            "ARTWORK PIN1_MARKER 0 400 0 0",
            "FID PRIMARY OPTICAL1 0 0 TOP 0 0",
            "$ENDSHAPES",
        ]
        .join("\r\n");

        let parsed = ParsedGencadFile::new(example.as_bytes()).unwrap();

        assert_eq!(
            parsed,
            ParsedGencadFile {
                sections: vec![
                    Section {
                        name: "HEADER".to_string(),
                        parameters: vec![
                            KeywordParam {
                                keyword: "GENCAD".to_string(),
                                parameter: "1.4".to_string()
                            },
                            KeywordParam {
                                keyword: "USER".to_string(),
                                parameter: "\"Mitron Europe Ltd. Serial Number 00001\"".to_string()
                            },
                            KeywordParam {
                                keyword: "DRAWING".to_string(),
                                parameter: "\"Modem C100 motherboard 1234-5678\"".to_string()
                            },
                            KeywordParam {
                                keyword: "REVISION".to_string(),
                                parameter: "\"Rev 566g 20th September 1990\"".to_string()
                            },
                            KeywordParam {
                                keyword: "UNITS".to_string(),
                                parameter: "USER 1200".to_string()
                            },
                            KeywordParam {
                                keyword: "ORIGIN".to_string(),
                                parameter: "0 0".to_string()
                            },
                            KeywordParam {
                                keyword: "INTERTRACK".to_string(),
                                parameter: "0".to_string()
                            },
                            KeywordParam {
                                keyword: "ATTRIBUTE".to_string(),
                                parameter: "alpha m_part \"BIS 9600\"".to_string()
                            },
                            KeywordParam {
                                keyword: "ATTRIBUTE".to_string(),
                                parameter: "alpha m_desc \"Issue 2\"".to_string()
                            }
                        ]
                    },
                    Section {
                        name: "BOARD".to_string(),
                        parameters: vec![
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "1000 2000 1200 2000".to_string()
                            },
                            KeywordParam {
                                keyword: "ARC".to_string(),
                                parameter: "1200 2000 1200 3000 1180 2500".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "1200 3000 1000 3000".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "1000 3000 1000 2000".to_string()
                            },
                            KeywordParam {
                                keyword: "CUTOUT".to_string(),
                                parameter: "TRANSFORMER_HOLE".to_string()
                            },
                            KeywordParam {
                                keyword: "CIRCLE".to_string(),
                                parameter: "1180 2500 20".to_string()
                            },
                            KeywordParam {
                                keyword: "ATTRIBUTE".to_string(),
                                parameter: "board mill \"tool 255\"".to_string()
                            },
                            KeywordParam {
                                keyword: "MASK".to_string(),
                                parameter: "Fixture_1 TOP".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "1005 2005 1195 2005".to_string()
                            },
                            KeywordParam {
                                keyword: "ARC".to_string(),
                                parameter: "1195 2005 1195 2995 1195 2500".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "1195 2995 1005 2995".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "1005 2995 1005 2005".to_string()
                            },
                            KeywordParam {
                                keyword: "ARTWORK".to_string(),
                                parameter: "ORIGIN_MARKER TOP".to_string()
                            },
                            KeywordParam {
                                keyword: "TRACK".to_string(),
                                parameter: "10".to_string()
                            },
                            KeywordParam {
                                keyword: "FILLED".to_string(),
                                parameter: "YES".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "-100 0 100 0".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "0 -100 0 100".to_string()
                            }
                        ]
                    },
                    Section {
                        name: "PADS".to_string(),
                        parameters: vec![
                            KeywordParam {
                                keyword: "PAD".to_string(),
                                parameter: "p0101 FINGER 32".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "100 50 -100 50".to_string()
                            },
                            KeywordParam {
                                keyword: "ARC".to_string(),
                                parameter: "-100 50 -100 -50 -100 0".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "-100 -50 100 -50".to_string()
                            },
                            KeywordParam {
                                keyword: "ARC".to_string(),
                                parameter: "100 -50 100 50 100 0".to_string()
                            },
                            KeywordParam {
                                keyword: "PAD".to_string(),
                                parameter: "p1053 ROUND 20".to_string()
                            },
                            KeywordParam {
                                keyword: "CIRCLE".to_string(),
                                parameter: "0 0 30".to_string()
                            },
                            KeywordParam {
                                keyword: "PAD".to_string(),
                                parameter: "p2034 BULLET 32".to_string()
                            },
                            KeywordParam {
                                keyword: "ARC".to_string(),
                                parameter: "0 -50 0 50 0 0".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "0 50 -100 50".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "-100 50 -100 -50".to_string()
                            },
                            KeywordParam {
                                keyword: "LINE".to_string(),
                                parameter: "-100 -50 0 -50".to_string()
                            },
                            KeywordParam {
                                keyword: "PAD".to_string(),
                                parameter: "d_hole_50 ROUND 50".to_string()
                            },
                            KeywordParam {
                                keyword: "CIRCLE".to_string(),
                                parameter: "0 0 25".to_string()
                            },
                            KeywordParam {
                                keyword: "PAD".to_string(),
                                parameter: "3 RECTANGULAR 0".to_string()
                            },
                            KeywordParam {
                                keyword: "RECTANGLE".to_string(),
                                parameter: "-5.2 -5.2 10.4 10.4".to_string()
                            }
                        ]
                    }
                ]
            }
        );
    }
}
