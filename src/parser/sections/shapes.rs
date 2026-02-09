// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD SHAPES section.
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

use crate::impl_to_gencad_string_for_section;
use crate::parser::KeywordParam;
use crate::parser::types::util::spaces;
use crate::parser::types::{
    arc_ref, artwork_name, attrib_ref, circle_ref, fid_name, height, layer, line_ref, mirror,
    pad_name, pin_name, rectangle_ref, rot, shape_name, string, x_y_ref,
};
use crate::serialization::ToGencadString;
use crate::types::{
    ArcRef, Attribute, CircleRef, Layer, LineRef, Mirror, Number, RectangleRef, XYRef,
};

/// Geometric elements that define the outline of a component shape.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeElement {
    /// A straight line forming part of the shape.
    Line(LineRef),
    /// A circular or elliptical arc forming part of the shape.
    Arc(ArcRef),
    /// A full circle forming part of the shape.
    Circle(CircleRef),
    /// A rectangle forming part of the shape.
    Rectangle(RectangleRef),
    /// A fiducial point defined by its coordinates relative to the shape origin.
    Fiducial(XYRef),
}

impl ToGencadString for ShapeElement {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Line(line) => format!("LINE {}", line.to_gencad_string()),
            Self::Arc(arc) => format!("ARC {}", arc.to_gencad_string()),
            Self::Circle(circle) => format!("CIRCLE {}", circle.to_gencad_string()),
            Self::Rectangle(rect) => format!("RECTANGLE {}", rect.to_gencad_string()),
            Self::Fiducial(xy) => format!("FIDUCIAL {}", xy.to_gencad_string()),
        }
    }
}

/// Optional package style for component insertion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Insert {
    /// Through-hole package.
    Th,
    /// Axial leaded package.
    Axial,
    /// Radial leaded package.
    Radial,
    /// Dual in-line package.
    Dip,
    /// Single in-line package.
    Sip,
    /// Zig-zag in-line package.
    Zip,
    /// Through-hole connector package.
    Conn,
    /// Surface-mount device package.
    Smd,
    /// Package styles other than through-hole or surface-mount.
    Other,
}

impl Insert {
    fn new(s: &str) -> Result<Self, String> {
        match s {
            "TH" => Ok(Self::Th),
            "AXIAL" => Ok(Self::Axial),
            "RADIAL" => Ok(Self::Radial),
            "DIP" => Ok(Self::Dip),
            "SIP" => Ok(Self::Sip),
            "ZIP" => Ok(Self::Zip),
            "CONN" => Ok(Self::Conn),
            "SMD" => Ok(Self::Smd),
            "OTHER" => Ok(Self::Other),
            _ => Err(format!("Unexpected INSERT statement: {}", s)),
        }
    }
}

impl ToGencadString for Insert {
    fn to_gencad_string(&self) -> String {
        format!(
            "INSERT {}",
            match self {
                Self::Th => "TH",
                Self::Axial => "AXIAL",
                Self::Radial => "RADIAL",
                Self::Dip => "DIP",
                Self::Sip => "SIP",
                Self::Zip => "ZIP",
                Self::Conn => "CONN",
                Self::Smd => "SMD",
                Self::Other => "OTHER",
            }
        )
    }
}

/// An artwork feature defined in the `ARTWORKS` section, placed relative to the shape origin.
#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    /// The name of the artwork as defined in the `ARTWORKS` section.
    pub name: String,
    /// The position of the artwork's origin relative to the shape origin.
    pub xy: XYRef,
    /// The rotation of the artwork around its origin, in degrees counterclockwise.
    pub rotation: Number,
    /// The mirror state of the artwork (applied before rotation).
    pub mirror: Mirror,
    /// Additional metadata associated with the artwork.
    pub attributes: Vec<Attribute>,
}

impl Artwork {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, xy, rotation, mirror)) = (
            artwork_name,
            preceded(spaces, x_y_ref),
            preceded(spaces, rot),
            preceded(spaces, mirror),
        )
            .parse(params)
            .map_err(|err| err.to_owned())?;

        let attributes = Vec::new();

        Ok(Self {
            name,
            xy,
            rotation,
            mirror,
            attributes,
        })
    }
}

impl ToGencadString for Artwork {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "ARTWORK {} {} {} {}",
            self.name.to_gencad_string(),
            self.xy.to_gencad_string(),
            self.rotation,
            self.mirror.to_gencad_string()
        ));
        lines.push(self.attributes.to_gencad_string());
        lines.join("\r\n")
    }
}

/// A fiducial marker using a pad or padstack, defined relative to the shape origin.
#[derive(Debug, Clone, PartialEq)]
pub struct Fid {
    /// The name of the fiducial (must be unique within the shape).
    pub name: String,
    /// The name of the pad or padstack used for the fiducial.
    pub pad_name: String,
    /// The position of the fiducial's center relative to the shape origin.
    pub xy: XYRef,
    /// The layer on which the fiducial is placed, relative to the shape's layer.
    pub layer: Layer,
    /// The rotation of the fiducial around its origin, in degrees counterclockwise.
    pub rotation: Number,
    /// The mirror state of the fiducial (applied before rotation).
    pub mirror: Mirror,
    /// Additional metadata associated with the fiducial.
    pub attributes: Vec<Attribute>,
}

impl Fid {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, pad_name, xy, layer, rotation, mirror)) = (
            fid_name,
            preceded(spaces, pad_name),
            preceded(spaces, x_y_ref),
            preceded(spaces, layer),
            preceded(spaces, rot),
            preceded(spaces, mirror),
        )
            .parse(params)
            .map_err(|err| err.to_owned())?;

        let attributes = Vec::new();

        Ok(Self {
            name,
            pad_name,
            xy,
            layer,
            rotation,
            mirror,
            attributes,
        })
    }
}

impl ToGencadString for Fid {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "FID {} {} {} {} {} {}",
            self.name.to_gencad_string(),
            self.pad_name.to_gencad_string(),
            self.xy.to_gencad_string(),
            self.layer.to_gencad_string(),
            self.rotation,
            self.mirror.to_gencad_string()
        ));
        lines.push(self.attributes.to_gencad_string());
        lines.join("\r\n")
    }
}

/// A pin defined using a pad or padstack, placed relative to the shape origin.
#[derive(Debug, Clone, PartialEq)]
pub struct Pin {
    /// The name of the pin (must match device pin names).
    pub name: String,
    /// The name of the pad or padstack used for the pin.
    pub pad_name: String,
    /// The position of the pin's center relative to the shape origin.
    pub xy: XYRef,
    /// The layer on which the pin is placed, relative to the shape's layer.
    pub layer: Layer,
    /// The rotation of the pin around its origin, in degrees counterclockwise.
    pub rotation: Number,
    /// The mirror state of the pin (applied before rotation).
    pub mirror: Mirror,
    /// Additional metadata associated with the pin.
    pub attributes: Vec<Attribute>,
}

impl Pin {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, pad_name, xy, layer, rotation, mirror)) = (
            pin_name,
            preceded(spaces, pad_name),
            preceded(spaces, x_y_ref),
            preceded(spaces, layer),
            preceded(spaces, rot),
            preceded(spaces, mirror),
        )
            .parse(params)
            .map_err(|err| err.to_owned())?;

        let attributes = Vec::new();

        Ok(Self {
            name,
            pad_name,
            xy,
            layer,
            rotation,
            mirror,
            attributes,
        })
    }
}

impl ToGencadString for Pin {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "PIN {} {} {} {} {} {}",
            self.name.to_gencad_string(),
            self.pad_name.to_gencad_string(),
            self.xy.to_gencad_string(),
            self.layer.to_gencad_string(),
            self.rotation,
            self.mirror.to_gencad_string()
        ));
        lines.push(self.attributes.to_gencad_string());
        lines.join("\r\n")
    }
}

/// A subcomponent (artwork, fiducial, or pin) associated with a shape.
#[derive(Debug, Clone, PartialEq)]
pub enum SubShape {
    /// An artwork feature defined in the `ARTWORKS` section.
    Artwork(Artwork),
    /// A fiducial marker using a pad or padstack.
    Fid(Fid),
    /// A pin defined using a pad or padstack.
    Pin(Pin),
}

impl ToGencadString for SubShape {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Artwork(artwork) => artwork.to_gencad_string(),
            Self::Fid(fid) => fid.to_gencad_string(),
            Self::Pin(pin) => pin.to_gencad_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ShapeParserState {
    Shape,
    SubShape(SubShape),
}

/// A reusable component outline defined in the `SHAPES` section.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// The unique name of the shape.
    pub name: String,
    /// Geometric elements defining the shape's outline.
    pub elements: Vec<ShapeElement>,
    /// Optional package style for component insertion.
    pub insert: Option<Insert>,
    /// The maximum height of the component from the board's surface.
    pub height: Option<Number>,
    /// Subcomponents (artwork, fiducials, or pins) associated with the shape.
    pub subshapes: Vec<SubShape>,
    /// Additional metadata associated with the shape.
    pub attributes: Vec<Attribute>,
}

impl ToGencadString for Shape {
    fn to_gencad_string(&self) -> String {
        let mut lines = Vec::new();

        // Start with the SHAPE line
        lines.push(format!("SHAPE {}", self.name.to_gencad_string()));

        // Add elements
        lines.extend(
            self.elements
                .iter()
                .map(|e| e.to_gencad_string())
                .collect::<Vec<_>>(),
        );

        // Add optional INSERT
        if let Some(insert) = &self.insert {
            lines.push(insert.to_gencad_string());
        }

        // Add optional HEIGHT
        if let Some(height) = &self.height {
            lines.push(format!("HEIGHT {}", height));
        }

        // Add subshapes
        lines.extend(
            self.subshapes
                .iter()
                .map(|shape| shape.to_gencad_string())
                .collect::<Vec<_>>(),
        );

        // Add attributes
        lines.push(self.attributes.to_gencad_string());

        lines.join("\r\n")
    }
}

impl_to_gencad_string_for_section!(Shape, "$SHAPES", "$ENDSHAPES");

#[derive(Debug, Clone, PartialEq)]
struct ShapeParser {
    state: ShapeParserState,
    shape: Shape,
}

impl ShapeParser {
    fn from_parameters(
        params: &str,
        insert: Option<Insert>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, name) = shape_name(params).map_err(|err| err.to_owned())?;

        let state = ShapeParserState::Shape;
        let name = name.to_string();
        let elements = Vec::new();
        let height = None;
        let subshapes = Vec::new();
        let attributes = Vec::new();

        let shape = Shape {
            name,
            elements,
            insert,
            height,
            subshapes,
            attributes,
        };

        Ok(Self { state, shape })
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.state {
            ShapeParserState::Shape => match kp.keyword {
                "LINE" => {
                    let (_, line) = line_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.elements.push(ShapeElement::Line(line));
                    Ok(())
                }
                "ARC" => {
                    let (_, arc) = arc_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.elements.push(ShapeElement::Arc(arc));
                    Ok(())
                }
                "CIRCLE" => {
                    let (_, circle) = circle_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.elements.push(ShapeElement::Circle(circle));
                    Ok(())
                }
                "RECTANGLE" => {
                    let (_, rectangle) =
                        rectangle_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.elements.push(ShapeElement::Rectangle(rectangle));
                    Ok(())
                }
                "FIDUCIAL" => {
                    let (_, fiducial) = x_y_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.elements.push(ShapeElement::Fiducial(fiducial));
                    Ok(())
                }

                "INSERT" => {
                    let (_, insert) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.insert = Some(Insert::new(&insert)?);
                    Ok(())
                }
                "HEIGHT" => {
                    let (_, height) = height(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.height = Some(height);
                    Ok(())
                }

                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.shape.attributes.push(attribute);
                    Ok(())
                }

                "ARTWORK" => {
                    let artwork = Artwork::from_parameters(kp.parameter)?;
                    self.state = ShapeParserState::SubShape(SubShape::Artwork(artwork));
                    Ok(())
                }
                "FID" => {
                    let fid = Fid::from_parameters(kp.parameter)?;
                    self.state = ShapeParserState::SubShape(SubShape::Fid(fid));
                    Ok(())
                }
                "PIN" => {
                    let pin = Pin::from_parameters(kp.parameter)?;
                    self.state = ShapeParserState::SubShape(SubShape::Pin(pin));
                    Ok(())
                }

                _ => Err(format!("Unexpected keyword in shape: {}", kp.keyword).into()),
            },
            ShapeParserState::SubShape(subshape) => match kp.keyword {
                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    match subshape {
                        SubShape::Artwork(a) => a.attributes.push(attribute),
                        SubShape::Fid(f) => f.attributes.push(attribute),
                        SubShape::Pin(p) => p.attributes.push(attribute),
                    }
                    Ok(())
                }

                "ARTWORK" => {
                    self.done();
                    let artwork = Artwork::from_parameters(kp.parameter)?;
                    self.state = ShapeParserState::SubShape(SubShape::Artwork(artwork));
                    Ok(())
                }
                "FID" => {
                    self.done();
                    let fid = Fid::from_parameters(kp.parameter)?;
                    self.state = ShapeParserState::SubShape(SubShape::Fid(fid));
                    Ok(())
                }
                "PIN" => {
                    self.done();
                    let pin = Pin::from_parameters(kp.parameter)?;
                    self.state = ShapeParserState::SubShape(SubShape::Pin(pin));
                    Ok(())
                }

                _ => Err(format!("Unexpected keyword in subshape: {}", kp.keyword).into()),
            },
        }
    }

    fn done(&mut self) {
        match &self.state {
            ShapeParserState::Shape => (),
            ShapeParserState::SubShape(subshape) => self.shape.subshapes.push(subshape.clone()),
        }
        self.state = ShapeParserState::Shape;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ShapesParserState {
    Reset,
    ShapeParser(ShapeParser),
}

struct ShapesParser {
    state: ShapesParserState,
    insert: Option<Insert>,
    shapes: Vec<Shape>,
}

impl ShapesParser {
    fn new() -> Self {
        let state = ShapesParserState::Reset;
        let insert = None;
        let shapes = Vec::new();
        Self {
            state,
            insert,
            shapes,
        }
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        if let ShapesParserState::ShapeParser(ref mut parser) = self.state {
            match kp.keyword {
                "SHAPE" => {
                    parser.done();
                    self.shapes.push(parser.shape.clone());
                    let parser = ShapeParser::from_parameters(kp.parameter, self.insert)?;
                    self.state = ShapesParserState::ShapeParser(parser);
                    Ok(())
                }
                "INSERT" => {
                    let (_, insert) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.insert = Some(Insert::new(&insert)?);
                    Ok(())
                }
                _ => parser.ingest(kp),
            }
        } else {
            match kp.keyword {
                "SHAPE" => {
                    let parser = ShapeParser::from_parameters(kp.parameter, self.insert)?;
                    self.state = ShapesParserState::ShapeParser(parser);
                    Ok(())
                }
                "INSERT" => {
                    let (_, insert) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.insert = Some(Insert::new(&insert)?);
                    Ok(())
                }
                _ => Err(format!("Unexpected keyword in shapes: {}", kp.keyword).into()),
            }
        }
    }

    fn finalize(mut self) -> Vec<Shape> {
        if let ShapesParserState::ShapeParser(mut parser) = self.state {
            parser.done();
            self.shapes.push(parser.shape);
            self.state = ShapesParserState::Reset;
        }
        self.shapes
    }
}

/// Parse the `SHAPES` section of a GenCAD file.
pub(crate) fn parse_shapes(
    params: &[KeywordParam],
) -> Result<Vec<Shape>, Box<dyn std::error::Error>> {
    let mut sp = ShapesParser::new();
    for param in params {
        sp.ingest(param)?;
    }
    let shapes = sp.finalize();
    Ok(shapes)
}
