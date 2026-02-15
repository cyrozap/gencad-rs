// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD BOARD section.
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
    arc_ref, attrib_ref, circle_ref, filled_ref, layer, line_ref, number, rectangle_ref, string,
    text_par, track_name, util::spaces, x_y_ref,
};
use crate::types::{
    ArcRef, Attribute, CircleRef, Layer, LineRef, Number, RectangleRef, TextPar, XYRef,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardShape {
    Line(LineRef),
    Arc(ArcRef),
    Circle(CircleRef),
    Rectangle(RectangleRef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cutout {
    pub name: String,
    pub shapes: Vec<BoardShape>,
    pub attributes: Vec<Attribute>,
}

impl Cutout {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let shapes = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            shapes,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mask {
    pub name: String,
    pub layer: Layer,
    pub shapes: Vec<BoardShape>,
    pub attributes: Vec<Attribute>,
}

impl Mask {
    fn new(name: &str, layer: Layer) -> Self {
        let name = name.to_string();
        let shapes = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            layer,
            shapes,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub origin: XYRef,
    pub text: TextPar,
}

impl Text {
    fn new(origin: XYRef, text: TextPar) -> Self {
        Self { origin, text }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArtworkComponent {
    Line(LineRef),
    Arc(ArcRef),
    Circle(CircleRef),
    Rectangle(RectangleRef),
    Track(String),
    Filled(bool),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    pub name: String,
    pub layer: Layer,
    pub components: Vec<ArtworkComponent>,
    pub attributes: Vec<Attribute>,
}

impl Artwork {
    fn new(name: &str, layer: Layer) -> Self {
        let name = name.to_string();
        let components = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            layer,
            components,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Subsection {
    Cutout(Cutout),
    Mask(Mask),
    Artwork(Artwork),
}

#[derive(Debug, Clone, PartialEq)]
enum BoardParserState {
    Board,
    Subsection(Subsection),
}

/// Represents the `BOARD` section of a GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    /// The board thickness, if specified.
    pub thickness: Option<Number>,
    /// The shapes that make up the board outline.
    pub outline_shapes: Vec<BoardShape>,
    /// A list of attributes associated with the board.
    pub attributes: Vec<Attribute>,
    /// A list of subsections in the board.
    pub subsections: Vec<Subsection>,
}

impl Board {
    pub(crate) fn new(params: &[KeywordParam]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut thickness = None;
        let mut outline_shapes = Vec::new();
        let mut attributes = Vec::new();
        let mut subsections = Vec::new();

        let mut parser_state = BoardParserState::Board;

        for param in params {
            match param.keyword {
                "THICKNESS" => {
                    if parser_state == BoardParserState::Board && thickness.is_none() {
                        let (_, value) = number(param.parameter).map_err(|err| err.to_owned())?;
                        thickness = Some(value);
                    }
                }
                "LINE" => {
                    let (_, line) = line_ref(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Line(line))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Line(line))
                            }
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Line(line))
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Line(line));
                    }
                }
                "ARC" => {
                    let (_, arc) = arc_ref(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Arc(arc))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Arc(arc))
                            }
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Arc(arc))
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Arc(arc));
                    }
                }
                "CIRCLE" => {
                    let (_, circle) = circle_ref(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Circle(circle))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Circle(circle))
                            }
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Circle(circle))
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Circle(circle));
                    }
                }
                "RECTANGLE" => {
                    let (_, rectangle) =
                        rectangle_ref(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Rectangle(rectangle))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Rectangle(rectangle))
                            }
                            Subsection::Artwork(ref mut artwork) => artwork
                                .components
                                .push(ArtworkComponent::Rectangle(rectangle)),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Rectangle(rectangle));
                    }
                }
                "ATTRIBUTE" => {
                    let (_, attribute) =
                        attrib_ref(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => cutout.attributes.push(attribute),
                            Subsection::Mask(ref mut mask) => mask.attributes.push(attribute),
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.attributes.push(attribute)
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        attributes.push(attribute);
                    }
                }

                // Artwork-only keywords
                "TRACK" => {
                    let (_, track) = track_name(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Track(track))
                            }
                            _ => return Err("TRACK is only supported in ARTWORK section!".into()),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        return Err("TRACK is only supported in ARTWORK section!".into());
                    }
                }
                "FILLED" => {
                    let (_, filled) = filled_ref(param.parameter).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Filled(filled))
                            }
                            _ => return Err("FILLED is only supported in ARTWORK section!".into()),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        return Err("FILLED is only supported in ARTWORK section!".into());
                    }
                }
                "TEXT" => {
                    let (_, (origin, text)) = (x_y_ref, preceded(spaces, text_par))
                        .parse(param.parameter)
                        .map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Artwork(ref mut artwork) => artwork
                                .components
                                .push(ArtworkComponent::Text(Text::new(origin, text))),
                            _ => return Err("TEXT is only supported in ARTWORK section!".into()),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        return Err("TEXT is only supported in ARTWORK section!".into());
                    }
                }

                // Subsections
                "CUTOUT" => {
                    if let BoardParserState::Subsection(subsection) = parser_state {
                        subsections.push(subsection);
                    }
                    let (_, cutout_name) = string(param.parameter).map_err(|err| err.to_owned())?;
                    parser_state = BoardParserState::Subsection(Subsection::Cutout(Cutout::new(
                        cutout_name.as_str(),
                    )))
                }
                "MASK" => {
                    if let BoardParserState::Subsection(subsection) = parser_state {
                        subsections.push(subsection);
                    }
                    let (_, (mask_name, mask_layer)) = (string, preceded(spaces, layer))
                        .parse(param.parameter)
                        .map_err(|err| err.to_owned())?;
                    parser_state = BoardParserState::Subsection(Subsection::Mask(Mask::new(
                        mask_name.as_str(),
                        mask_layer,
                    )))
                }
                "ARTWORK" => {
                    if let BoardParserState::Subsection(subsection) = parser_state {
                        subsections.push(subsection);
                    }
                    let (_, (mask_name, mask_layer)) = (string, preceded(spaces, layer))
                        .parse(param.parameter)
                        .map_err(|err| err.to_owned())?;
                    parser_state = BoardParserState::Subsection(Subsection::Artwork(Artwork::new(
                        mask_name.as_str(),
                        mask_layer,
                    )))
                }
                _ => {}
            }
        }

        if let BoardParserState::Subsection(subsection) = parser_state {
            subsections.push(subsection);
        }

        Ok(Self {
            thickness,
            outline_shapes,
            attributes,
            subsections,
        })
    }
}
