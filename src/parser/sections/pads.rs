// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD PADS section.
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
use crate::parser::types::{
    arc_ref, attrib_ref, circle_ref, drill_size, line_ref, pad_name, pad_type, rectangle_ref,
    util::spaces,
};
use crate::types::{ArcRef, Attribute, CircleRef, LineRef, Number, PadType, RectangleRef};

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
pub(crate) fn parse_pads(params: &[KeywordParam]) -> Result<Vec<Pad>, Box<dyn std::error::Error>> {
    let mut pads = Vec::new();

    let mut parser_state = ParserState::Init;

    for param in params {
        match param.keyword {
            "LINE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, line) = line_ref(param.parameter).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Line(line));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "ARC" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, arc) = arc_ref(param.parameter).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Arc(arc));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "CIRCLE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, circle) = circle_ref(param.parameter).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Circle(circle));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "RECTANGLE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, rectangle) =
                        rectangle_ref(param.parameter).map_err(|err| err.to_owned())?;
                    pad.shapes.push(PadShape::Rectangle(rectangle));
                    parser_state = ParserState::Pad(pad);
                }
            }
            "ATTRIBUTE" => {
                if let ParserState::Pad(mut pad) = parser_state {
                    let (_, attribute) =
                        attrib_ref(param.parameter).map_err(|err| err.to_owned())?;
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
                    .parse(param.parameter)
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
