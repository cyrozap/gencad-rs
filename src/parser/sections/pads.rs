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

/// A geometric shape that is used to define the outer edge of a pad. All coordinates are relative to the pad's origin.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PadShape {
    /// A straight line forming part of the pad's outer edge.
    Line(LineRef),
    /// A circular or elliptical arc forming part of the pad's outer edge.
    Arc(ArcRef),
    /// A full circle forming the pad's outer edge.
    Circle(CircleRef),
    /// A rectangle forming the pad's outer edge.
    Rectangle(RectangleRef),
}

/// A pad on the circuit board. Pads define the physical shape and drill hole of contact points on the board.
#[derive(Debug, Clone, PartialEq)]
pub struct Pad {
    /// The name of the pad. Must be unique per pad and used consistently
    /// throughout the file. If the CAD system does not assign names, sequential
    /// names like "pad1", "pad2", etc., must be used.
    pub name: String,
    /// The type of the pad.
    pub ptype: PadType,
    /// The drill hole size in [crate::types::Dimension] units. A value of `0.0`
    /// indicates no hole. If undefined, a value of `-1.0` must be used.
    pub drill_size: Number,
    /// The list of shapes that define the outer edge of the pad. All
    /// coordinates are relative to the pad's origin.
    pub shapes: Vec<PadShape>,
    /// Optional metadata associated with the pad.
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
