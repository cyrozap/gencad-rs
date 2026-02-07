// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/pads.rs - Parser for the GenCAD PADS section.
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

use nom::Parser;
use nom::sequence::preceded;

use crate::parser::KeywordParam;
use crate::types::{
    ArcRef, Attribute, CircleRef, LineRef, Number, PadType, RectangleRef, arc_ref, attrib_ref,
    circle_ref, drill_size, line_ref, pad_name, pad_type, rectangle_ref, util::spaces,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PadShape {
    Line(LineRef),
    Arc(ArcRef),
    Circle(CircleRef),
    Rectangle(RectangleRef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pad {
    pub name: String,
    pub ptype: PadType,
    pub drill_size: Number,
    pub shapes: Vec<PadShape>,
    pub attributes: Vec<Attribute>,
}

impl Pad {
    fn new(name: &str, ptype: PadType, drill_size: Number) -> Self {
        let name = name.to_string();
        let shapes = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            ptype,
            drill_size,
            shapes,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    Init,
    Pad(Pad),
}

/// Parse the `PADS` section of a GenCAD file.
pub fn parse_pads(params: &[KeywordParam]) -> Result<Vec<Pad>, Box<dyn std::error::Error>> {
    let mut pads = Vec::new();

    let mut parser_state = ParserState::Init;

    for param in params {
        match param.keyword.as_str() {
            "LINE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, line) =
                        line_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Line(line));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "ARC" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, arc) =
                        arc_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Arc(arc));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "CIRCLE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, circle) =
                        circle_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Circle(circle));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "RECTANGLE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, rectangle) =
                        rectangle_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Rectangle(rectangle));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "ATTRIBUTE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, attribute) =
                        attrib_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    pad.attributes.push(attribute);
                    parser_state = ParserState::Pad(pad);
                }
            }

            // Pads
            "PAD" => {
                if let ParserState::Pad(pad) = parser_state {
                    pads.push(pad);
                }
                let (_, (name, ptype, drill_size)) = (
                    pad_name,
                    preceded(spaces, pad_type),
                    preceded(spaces, drill_size),
                )
                    .parse(param.parameter.as_str())
                    .map_err(|err| err.to_owned())?;
                parser_state = ParserState::Pad(Pad::new(name.as_str(), ptype, drill_size))
            }
            _ => {}
        }
    }

    if let ParserState::Pad(pad) = parser_state {
        pads.push(pad);
    }

    Ok(pads)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CircularArcRef, XYRef};

    #[test]
    fn test_example_pads() {
        let params = vec![
            KeywordParam {
                keyword: "PAD".to_string(),
                parameter: "p0101 FINGER 32".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "100 50 -100 50".to_string(),
            },
            KeywordParam {
                keyword: "ARC".to_string(),
                parameter: "-100 50 -100 -50 -100 0".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "-100 -50 100 -50".to_string(),
            },
            KeywordParam {
                keyword: "ARC".to_string(),
                parameter: "100 -50 100 50 100 0".to_string(),
            },
            KeywordParam {
                keyword: "PAD".to_string(),
                parameter: "p1053 ROUND 20".to_string(),
            },
            KeywordParam {
                keyword: "CIRCLE".to_string(),
                parameter: "0 0 30".to_string(),
            },
            KeywordParam {
                keyword: "PAD".to_string(),
                parameter: "p2034 BULLET 32".to_string(),
            },
            KeywordParam {
                keyword: "ARC".to_string(),
                parameter: "0 -50 0 50 0 0".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "0 50 -100 50".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "-100 50 -100 -50".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "-100 -50 0 -50".to_string(),
            },
            KeywordParam {
                keyword: "PAD".to_string(),
                parameter: "d_hole_50 ROUND 50".to_string(),
            },
            KeywordParam {
                keyword: "CIRCLE".to_string(),
                parameter: "0 0 25".to_string(),
            },
            KeywordParam {
                keyword: "PAD".to_string(),
                parameter: "3 RECTANGULAR 0".to_string(),
            },
            KeywordParam {
                keyword: "RECTANGLE".to_string(),
                parameter: "-5.2 -5.2 10.4 10.4".to_string(),
            },
        ];

        let pads = parse_pads(&params).unwrap();

        assert_eq!(
            pads,
            vec![
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
            ]
        );
    }
}
