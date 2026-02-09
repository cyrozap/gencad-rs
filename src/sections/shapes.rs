// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/shapes.rs - Parser for the GenCAD SHAPES section.
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
use crate::serialization::ToGencadString;
use crate::types::util::spaces;
use crate::types::{
    ArcRef, Attribute, CircleRef, Layer, LineRef, Mirror, Number, RectangleRef, XYRef, arc_ref,
    artwork_name, attrib_ref, circle_ref, fid_name, height, layer, line_ref, mirror, pad_name,
    pin_name, rectangle_ref, rot, shape_name, string, x_y_ref,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeElement {
    Line(LineRef),
    Arc(ArcRef),
    Circle(CircleRef),
    Rectangle(RectangleRef),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Insert {
    Th,
    Axial,
    Radial,
    Dip,
    Sip,
    Zip,
    Conn,
    Smd,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    pub name: String,
    pub xy: XYRef,
    pub rotation: Number,
    pub mirror: Mirror,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Fid {
    pub name: String,
    pub pad_name: String,
    pub xy: XYRef,
    pub layer: Layer,
    pub rotation: Number,
    pub mirror: Mirror,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Pin {
    pub name: String,
    pub pad_name: String,
    pub xy: XYRef,
    pub layer: Layer,
    pub rotation: Number,
    pub mirror: Mirror,
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

#[derive(Debug, Clone, PartialEq)]
pub enum SubShape {
    Artwork(Artwork),
    Fid(Fid),
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

#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub name: String,
    pub elements: Vec<ShapeElement>,
    pub insert: Option<Insert>,
    pub height: Option<Number>,
    pub subshapes: Vec<SubShape>,
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

#[cfg(test)]
mod tests {
    use crate::types::CircularArcRef;

    use super::*;

    #[test]
    fn test_example_shape() {
        let params = vec![
            KeywordParam {
                keyword: "SHAPE",
                parameter: "CAP_SUPPRESS_TYPE_____24",
            },
            KeywordParam {
                keyword: "LINE",
                parameter: "-1000 200 -1000 -200",
            },
            KeywordParam {
                keyword: "LINE",
                parameter: "-1000 -200 1000 -200",
            },
            KeywordParam {
                keyword: "ARC",
                parameter: "1000 -200 1000 200 1000 0",
            },
            KeywordParam {
                keyword: "LINE",
                parameter: "1000 200 -1000 200",
            },
            KeywordParam {
                keyword: "PIN",
                parameter: "1 p102_4 -100 100 TOP 315 0",
            },
            KeywordParam {
                keyword: "PIN",
                parameter: "1 s106_6 -100 100 BOTTOM 315 MIRRORX",
            },
            KeywordParam {
                keyword: "PIN",
                parameter: "2 p102_4 100 -100 TOP 135 0",
            },
            KeywordParam {
                keyword: "PIN",
                parameter: "2 s106_6 100 -100 BOTTOM 135 MIRRORX",
            },
            KeywordParam {
                keyword: "ARTWORK",
                parameter: "PIN1_MARKER 0 400 0 0",
            },
            KeywordParam {
                keyword: "FID",
                parameter: "PRIMARY OPTICAL1 0 0 TOP 0 0",
            },
        ];

        let shapes = parse_shapes(&params).unwrap();

        assert_eq!(
            shapes,
            vec![Shape {
                name: "CAP_SUPPRESS_TYPE_____24".to_string(),
                elements: vec![
                    ShapeElement::Line(LineRef {
                        start: XYRef {
                            x: -1000.0,
                            y: 200.0
                        },
                        end: XYRef {
                            x: -1000.0,
                            y: -200.0
                        }
                    }),
                    ShapeElement::Line(LineRef {
                        start: XYRef {
                            x: -1000.0,
                            y: -200.0
                        },
                        end: XYRef {
                            x: 1000.0,
                            y: -200.0
                        }
                    }),
                    ShapeElement::Arc(ArcRef::Circular(CircularArcRef {
                        start: XYRef {
                            x: 1000.0,
                            y: -200.0
                        },
                        end: XYRef {
                            x: 1000.0,
                            y: 200.0
                        },
                        center: XYRef { x: 1000.0, y: 0.0 },
                    })),
                    ShapeElement::Line(LineRef {
                        start: XYRef {
                            x: 1000.0,
                            y: 200.0
                        },
                        end: XYRef {
                            x: -1000.0,
                            y: 200.0
                        }
                    }),
                ],
                insert: None,
                height: None,
                subshapes: vec![
                    SubShape::Pin(Pin {
                        name: "1".to_string(),
                        pad_name: "p102_4".to_string(),
                        xy: XYRef {
                            x: -100.0,
                            y: 100.0
                        },
                        layer: Layer::Top,
                        rotation: 315.0,
                        mirror: Mirror::Not,
                        attributes: vec![]
                    }),
                    SubShape::Pin(Pin {
                        name: "1".to_string(),
                        pad_name: "s106_6".to_string(),
                        xy: XYRef {
                            x: -100.0,
                            y: 100.0
                        },
                        layer: Layer::Bottom,
                        rotation: 315.0,
                        mirror: Mirror::MirrorX,
                        attributes: vec![]
                    }),
                    SubShape::Pin(Pin {
                        name: "2".to_string(),
                        pad_name: "p102_4".to_string(),
                        xy: XYRef {
                            x: 100.0,
                            y: -100.0
                        },
                        layer: Layer::Top,
                        rotation: 135.0,
                        mirror: Mirror::Not,
                        attributes: vec![]
                    }),
                    SubShape::Pin(Pin {
                        name: "2".to_string(),
                        pad_name: "s106_6".to_string(),
                        xy: XYRef {
                            x: 100.0,
                            y: -100.0
                        },
                        layer: Layer::Bottom,
                        rotation: 135.0,
                        mirror: Mirror::MirrorX,
                        attributes: vec![]
                    }),
                    SubShape::Artwork(Artwork {
                        name: "PIN1_MARKER".to_string(),
                        xy: XYRef { x: 0.0, y: 400.0 },
                        rotation: 0.0,
                        mirror: Mirror::Not,
                        attributes: vec![]
                    }),
                    SubShape::Fid(Fid {
                        name: "PRIMARY".to_string(),
                        pad_name: "OPTICAL1".to_string(),
                        xy: XYRef { x: 0.0, y: 0.0 },
                        layer: Layer::Top,
                        rotation: 0.0,
                        mirror: Mirror::Not,
                        attributes: vec![]
                    })
                ],
                attributes: vec![]
            }]
        );
    }
}
