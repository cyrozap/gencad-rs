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
 *         println!("{:?}", section);
 *     }
 *
 *     Ok(())
 * }
 * ```
 */

use nom::bytes::complete::{is_a, tag, take_till, take_while, take_while1};
use nom::combinator::{fail, map_res};
use nom::multi::{many0, many1};
use nom::sequence::delimited;
use nom::{AsChar, IResult, Parser};

use crate::sections::board::Board;
use crate::sections::header::Header;
use crate::sections::pads::{Pad, parse_pads};
use crate::sections::shapes::{Shape, parse_shapes};

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
    delimited(
        tag(b"$".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
    .parse(input)
}

fn section_end(input: &[u8]) -> IResult<&[u8], &str> {
    delimited(
        tag(b"$END".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
    .parse(input)
}

/// A keyword/parameter pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct KeywordParam<'a> {
    /// The keyword that determines how to interpret the parameter.
    pub keyword: &'a str,
    /// The parameter associated with the keyword.
    pub parameter: &'a str,
}

impl<'a> KeywordParam<'a> {
    fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (remaining, (keyword, _, parameter, _)) = (
            map_res(keyword, str::from_utf8),
            tag(b" ".as_slice()),
            map_res(take_till(|c| c == b'\r' || c == b'\n'), str::from_utf8),
            take_newlines,
        )
            .parse(input)?;
        let kp = Self { keyword, parameter };
        Ok((remaining, kp))
    }
}

/// A section in the GenCAD file.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Section<'a> {
    /// The name of the section.
    name: &'a str,
    /// List of parameters in the section.
    parameters: Vec<KeywordParam<'a>>,
}

impl<'a> Section<'a> {
    fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (remaining, (start_tag, parameters, end_tag)) =
            (section_start, many0(KeywordParam::parse), section_end).parse(input)?;

        if start_tag != end_tag {
            return fail().parse(input);
        }

        let name = start_tag;

        Ok((remaining, Self { name, parameters }))
    }
}

fn sections<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<Section<'a>>> {
    many1(Section::parse).parse(input)
}

/// A section in the GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedSection {
    Header(Header),
    Board(Board),
    Pads(Vec<Pad>),
    Shapes(Vec<Shape>),
}

/// A fully parsed GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedGencadFile {
    /// The parsed sections of the file.
    pub sections: Vec<ParsedSection>,
}

impl ParsedGencadFile {
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

        let (_, unparsed_sections) = sections(&buffer).map_err(|err| err.to_owned())?;

        let mut sections = Vec::new();

        for section in unparsed_sections {
            match section.name {
                "HEADER" => sections.push(ParsedSection::Header(Header::new(&section.parameters)?)),
                "BOARD" => sections.push(ParsedSection::Board(Board::new(&section.parameters)?)),
                "PADS" => sections.push(ParsedSection::Pads(parse_pads(&section.parameters)?)),
                "SHAPES" => {
                    sections.push(ParsedSection::Shapes(parse_shapes(&section.parameters)?))
                }
                _ => (),
            }
        }

        Ok(Self { sections })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sections::board::{Artwork, ArtworkComponent, BoardShape, Cutout, Mask, Subsection};
    use crate::sections::pads::PadShape;
    use crate::types::{
        ArcRef, Attribute, CircleRef, CircularArcRef, Dimension, Layer, LineRef, PadType,
        RectangleRef, XYRef,
    };

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
                    ParsedSection::Header(Header {
                        gencad_version: 1.4,
                        user: "Mitron Europe Ltd. Serial Number 00001".to_string(),
                        drawing: "Modem C100 motherboard 1234-5678".to_string(),
                        revision: "Rev 566g 20th September 1990".to_string(),
                        units: Dimension::User(1200),
                        origin: XYRef { x: 0.0, y: 0.0 },
                        intertrack: 0.0,
                        attributes: vec![
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
                    }),
                    ParsedSection::Board(Board {
                        thickness: None,
                        outline_shapes: vec![
                            BoardShape::Line(LineRef {
                                start: XYRef {
                                    x: 1000.0,
                                    y: 2000.0
                                },
                                end: XYRef {
                                    x: 1200.0,
                                    y: 2000.0
                                }
                            }),
                            BoardShape::Arc(ArcRef::Circular(CircularArcRef {
                                start: XYRef {
                                    x: 1200.0,
                                    y: 2000.0
                                },
                                end: XYRef {
                                    x: 1200.0,
                                    y: 3000.0
                                },
                                center: XYRef {
                                    x: 1180.0,
                                    y: 2500.0
                                }
                            })),
                            BoardShape::Line(LineRef {
                                start: XYRef {
                                    x: 1200.0,
                                    y: 3000.0
                                },
                                end: XYRef {
                                    x: 1000.0,
                                    y: 3000.0
                                }
                            }),
                            BoardShape::Line(LineRef {
                                start: XYRef {
                                    x: 1000.0,
                                    y: 3000.0
                                },
                                end: XYRef {
                                    x: 1000.0,
                                    y: 2000.0
                                }
                            })
                        ],
                        attributes: vec![],
                        subsections: vec![
                            Subsection::Cutout(Cutout {
                                name: "TRANSFORMER_HOLE".to_string(),
                                shapes: vec![BoardShape::Circle(CircleRef {
                                    center: XYRef {
                                        x: 1180.0,
                                        y: 2500.0
                                    },
                                    radius: 20.0
                                })],
                                attributes: vec![Attribute {
                                    category: "board".to_string(),
                                    name: "mill".to_string(),
                                    data: "tool 255".to_string()
                                }]
                            }),
                            Subsection::Mask(Mask {
                                name: "Fixture_1".to_string(),
                                layer: Layer::Top,
                                shapes: vec![
                                    BoardShape::Line(LineRef {
                                        start: XYRef {
                                            x: 1005.0,
                                            y: 2005.0
                                        },
                                        end: XYRef {
                                            x: 1195.0,
                                            y: 2005.0
                                        }
                                    }),
                                    BoardShape::Arc(ArcRef::Circular(CircularArcRef {
                                        start: XYRef {
                                            x: 1195.0,
                                            y: 2005.0
                                        },
                                        end: XYRef {
                                            x: 1195.0,
                                            y: 2995.0
                                        },
                                        center: XYRef {
                                            x: 1195.0,
                                            y: 2500.0
                                        }
                                    })),
                                    BoardShape::Line(LineRef {
                                        start: XYRef {
                                            x: 1195.0,
                                            y: 2995.0
                                        },
                                        end: XYRef {
                                            x: 1005.0,
                                            y: 2995.0
                                        }
                                    }),
                                    BoardShape::Line(LineRef {
                                        start: XYRef {
                                            x: 1005.0,
                                            y: 2995.0
                                        },
                                        end: XYRef {
                                            x: 1005.0,
                                            y: 2005.0
                                        }
                                    })
                                ],
                                attributes: vec![]
                            }),
                            Subsection::Artwork(Artwork {
                                name: "ORIGIN_MARKER".to_string(),
                                layer: Layer::Top,
                                components: vec![
                                    ArtworkComponent::Track("10".to_string()),
                                    ArtworkComponent::Filled(true),
                                    ArtworkComponent::Line(LineRef {
                                        start: XYRef { x: -100.0, y: 0.0 },
                                        end: XYRef { x: 100.0, y: 0.0 }
                                    }),
                                    ArtworkComponent::Line(LineRef {
                                        start: XYRef { x: 0.0, y: -100.0 },
                                        end: XYRef { x: 0.0, y: 100.0 }
                                    })
                                ],
                                attributes: vec![]
                            })
                        ]
                    }),
                    ParsedSection::Pads(vec![
                        Pad {
                            name: "p0101".to_string(),
                            ptype: PadType::Finger,
                            drill_size: 32.0,
                            shapes: vec![
                                PadShape::Line(LineRef {
                                    start: XYRef { x: 100.0, y: 50.0 },
                                    end: XYRef { x: -100.0, y: 50.0 }
                                }),
                                PadShape::Arc(ArcRef::Circular(CircularArcRef {
                                    start: XYRef { x: -100.0, y: 50.0 },
                                    end: XYRef {
                                        x: -100.0,
                                        y: -50.0
                                    },
                                    center: XYRef { x: -100.0, y: 0.0 }
                                })),
                                PadShape::Line(LineRef {
                                    start: XYRef {
                                        x: -100.0,
                                        y: -50.0
                                    },
                                    end: XYRef { x: 100.0, y: -50.0 }
                                }),
                                PadShape::Arc(ArcRef::Circular(CircularArcRef {
                                    start: XYRef { x: 100.0, y: -50.0 },
                                    end: XYRef { x: 100.0, y: 50.0 },
                                    center: XYRef { x: 100.0, y: 0.0 }
                                }))
                            ],
                            attributes: vec![]
                        },
                        Pad {
                            name: "p1053".to_string(),
                            ptype: PadType::Round,
                            drill_size: 20.0,
                            shapes: vec![PadShape::Circle(CircleRef {
                                center: XYRef { x: 0.0, y: 0.0 },
                                radius: 30.0
                            })],
                            attributes: vec![]
                        },
                        Pad {
                            name: "p2034".to_string(),
                            ptype: PadType::Bullet,
                            drill_size: 32.0,
                            shapes: vec![
                                PadShape::Arc(ArcRef::Circular(CircularArcRef {
                                    start: XYRef { x: 0.0, y: -50.0 },
                                    end: XYRef { x: 0.0, y: 50.0 },
                                    center: XYRef { x: 0.0, y: 0.0 }
                                })),
                                PadShape::Line(LineRef {
                                    start: XYRef { x: 0.0, y: 50.0 },
                                    end: XYRef { x: -100.0, y: 50.0 }
                                }),
                                PadShape::Line(LineRef {
                                    start: XYRef { x: -100.0, y: 50.0 },
                                    end: XYRef {
                                        x: -100.0,
                                        y: -50.0
                                    }
                                }),
                                PadShape::Line(LineRef {
                                    start: XYRef {
                                        x: -100.0,
                                        y: -50.0
                                    },
                                    end: XYRef { x: 0.0, y: -50.0 }
                                })
                            ],
                            attributes: vec![]
                        },
                        Pad {
                            name: "d_hole_50".to_string(),
                            ptype: PadType::Round,
                            drill_size: 50.0,
                            shapes: vec![PadShape::Circle(CircleRef {
                                center: XYRef { x: 0.0, y: 0.0 },
                                radius: 25.0
                            })],
                            attributes: vec![]
                        },
                        Pad {
                            name: "3".to_string(),
                            ptype: PadType::Rectangular,
                            drill_size: 0.0,
                            shapes: vec![PadShape::Rectangle(RectangleRef {
                                origin: XYRef { x: -5.2, y: -5.2 },
                                x: 10.4,
                                y: 10.4
                            })],
                            attributes: vec![]
                        }
                    ])
                ]
            }
        );
    }
}
