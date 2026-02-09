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

use crate::impl_to_gencad_string_for_vec;
use crate::parser::KeywordParam;
use crate::parser::types::{
    arc_ref, attrib_ref, circle_ref, filled_ref, layer, line_ref, number, rectangle_ref, string,
    text_par, track_name, util::spaces, x_y_ref,
};
use crate::serialization::ToGencadString;
use crate::types::{
    ArcRef, Attribute, CircleRef, Layer, LineRef, Number, RectangleRef, TextPar, XYRef,
};

/// Represents a geometric shape used in board outlines, cutouts, or masks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardShape {
    /// A straight line forming part of the board outline, cutout, or mask.
    Line(LineRef),
    /// A circular or elliptical arc forming part of the board outline, cutout, or mask.
    Arc(ArcRef),
    /// A full circle forming part of the board outline, cutout, or mask.
    Circle(CircleRef),
    /// A rectangle forming part of the board outline, cutout, or mask.
    Rectangle(RectangleRef),
}

impl ToGencadString for BoardShape {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Line(line) => format!("LINE {}", line.to_gencad_string()),
            Self::Arc(arc) => format!("ARC {}", arc.to_gencad_string()),
            Self::Circle(circle) => format!("CIRCLE {}", circle.to_gencad_string()),
            Self::Rectangle(rect) => format!("RECTANGLE {}", rect.to_gencad_string()),
        }
    }
}

impl_to_gencad_string_for_vec!(BoardShape);

/// Represents an internal area of the board where all layers are cut away.
#[derive(Debug, Clone, PartialEq)]
pub struct Cutout {
    /// A unique identifier for the cutout (e.g., "cutout1", "cutout2").
    pub name: String,
    /// Geometric shapes defining the cutout's boundary.
    pub shapes: Vec<BoardShape>,
    /// Additional metadata associated with the cutout.
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

impl ToGencadString for Cutout {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("CUTOUT {}", self.name.to_gencad_string()));
        lines.push(self.shapes.to_gencad_string());
        lines.push(self.attributes.to_gencad_string());
        lines.join("\r\n")
    }
}

/// Represents an area of the board that is inaccessible to test pins.
#[derive(Debug, Clone, PartialEq)]
pub struct Mask {
    /// A unique identifier for the masked area (e.g., "mask1", "mask2").
    pub name: String,
    /// The board layer to which this mask applies (e.g., [Layer::Top], [Layer::Bottom]).
    pub layer: Layer,
    /// Geometric shapes defining the masked area's boundary.
    pub shapes: Vec<BoardShape>,
    /// Additional metadata associated with the mask.
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

impl ToGencadString for Mask {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "MASK {} {}",
            self.name.to_gencad_string(),
            self.layer.to_gencad_string()
        ));
        lines.push(self.shapes.to_gencad_string());
        lines.push(self.attributes.to_gencad_string());
        lines.join("\r\n")
    }
}

/// Represents a text string attached to a component or artwork feature.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    /// The bottom-left corner of the text relative to the component's origin.
    pub origin: XYRef,
    /// Specifies the text's size, rotation, mirror, layer, and bounding rectangle.
    pub text: TextPar,
}

impl Text {
    fn new(origin: XYRef, text: TextPar) -> Self {
        Self { origin, text }
    }
}

impl ToGencadString for Text {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "TEXT {} {}",
            self.origin.to_gencad_string(),
            self.text.to_gencad_string()
        ));
        lines.join("\r\n")
    }
}

/// Represents a component of an artwork feature on the board.
#[derive(Debug, Clone, PartialEq)]
pub enum ArtworkComponent {
    /// A straight line forming part of the artwork.
    Line(LineRef),
    /// A circular arc forming part of the artwork.
    Arc(ArcRef),
    /// A full circle forming part of the artwork.
    Circle(CircleRef),
    /// A rectangle forming part of the artwork.
    Rectangle(RectangleRef),
    /// A track type defined in the `TRACKS` section.
    Track(String),
    /// Indicates whether the following shapes form an enclosed area.
    Filled(bool),
    /// A text string attached to the artwork.
    Text(Text),
}

impl ToGencadString for ArtworkComponent {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Line(line) => format!("LINE {}", line.to_gencad_string()),
            Self::Arc(arc) => format!("ARC {}", arc.to_gencad_string()),
            Self::Circle(circle) => format!("CIRCLE {}", circle.to_gencad_string()),
            Self::Rectangle(rect) => format!("RECTANGLE {}", rect.to_gencad_string()),
            Self::Track(track) => format!("TRACK {}", track.to_gencad_string()),
            Self::Filled(filled) => {
                format!("FILLED {}", if *filled { "YES" } else { "0" }.to_string())
            }
            Self::Text(text) => text.to_gencad_string(),
        }
    }
}

impl_to_gencad_string_for_vec!(ArtworkComponent);

/// Represents an artwork feature on the board (e.g., silkscreen, routing).
#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    /// A unique identifier for the artwork feature (e.g., "artwork1", "artwork2").
    pub name: String,
    /// The board layer to which this artwork applies (e.g., [Layer::Top], [Layer::Bottom]).
    pub layer: Layer,
    /// Components (lines, arcs, text, etc.) defining the artwork.
    pub components: Vec<ArtworkComponent>,
    /// Additional metadata associated with the artwork.
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

impl ToGencadString for Artwork {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "ARTWORK {} {}",
            self.name.to_gencad_string(),
            self.layer.to_gencad_string()
        ));
        lines.push(self.components.to_gencad_string());
        lines.push(self.attributes.to_gencad_string());
        lines.join("\r\n")
    }
}

/// Represents a subsection within the `BOARD` section (e.g., cutouts, masks, artwork).
#[derive(Debug, Clone, PartialEq)]
pub enum Subsection {
    /// A named internal area of the board where all layers are cut away.
    Cutout(Cutout),
    /// A named area of the board that is inaccessible to test pins.
    Mask(Mask),
    /// A named artwork feature on the board.
    Artwork(Artwork),
}

impl ToGencadString for Subsection {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Cutout(s) => s.to_gencad_string(),
            Self::Mask(s) => s.to_gencad_string(),
            Self::Artwork(s) => s.to_gencad_string(),
        }
    }
}

impl_to_gencad_string_for_vec!(Subsection);

#[derive(Debug, Clone, PartialEq)]
enum BoardParserState {
    Board,
    Subsection(Subsection),
}

/// Represents the `BOARD` section of a GenCAD file, defining the board's outer shape and internal features.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    /// The thickness of the board in the [crate::types::Dimension] units specified in the `HEADER` section.
    pub thickness: Option<Number>,
    /// Geometric shapes defining the board's outer edge.
    pub outline_shapes: Vec<BoardShape>,
    /// Additional metadata associated with the board.
    pub attributes: Vec<Attribute>,
    /// Subsections such as cutouts, masks, and artwork features.
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

impl ToGencadString for Board {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push("$BOARD".to_string());
        if let Some(thickness) = self.thickness {
            lines.push(format!("THICKNESS {}", thickness));
        }
        lines.push(self.outline_shapes.to_gencad_string());
        lines.push(self.attributes.to_gencad_string());
        lines.push(self.subsections.to_gencad_string());
        lines.push("$ENDBOARD".to_string());
        lines.join("\r\n")
    }
}
