// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD COMPONENTS section.
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
use crate::parser::types::util::spaces;
use crate::parser::types::{
    artwork_name, attrib_ref, component_name, fid_name, flip, layer, mirror, pad_name, part_name,
    rot, shape_name, string, text_par, x_y_ref,
};
use crate::types::{Attribute, Layer, Mirror, Number, TextPar, XYRef};

/// A shape definition used by a component to describe its geometry and orientation.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// The name of the shape as defined in the `SHAPES` section.
    pub name: String,
    /// The mirror state of the shape. Mirroring is applied before rotation.
    pub mirror: Mirror,
    /// The flip state of the shape. Flipping changes the layer of the shape and its features.
    pub flip: bool,
}

impl Shape {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, mirror, flip)) =
            (shape_name, preceded(spaces, mirror), preceded(spaces, flip))
                .parse(params)
                .map_err(|err| err.to_owned())?;

        Ok(Self { name, mirror, flip })
    }
}

/// An artwork feature defined in the `ARTWORKS` section, placed relative to a component.
#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    /// The name of the artwork as defined in the `ARTWORKS` section.
    pub name: String,
    /// The position of the artwork's origin relative to the component origin.
    pub xy: XYRef,
    /// The rotation of the artwork around its origin, in degrees counterclockwise.
    pub rotation: Number,
    /// The mirror state of the artwork. Mirroring is applied before rotation.
    pub mirror: Mirror,
    /// The flip state of the artwork. Flipping changes the layer of the artwork and its features.
    pub flip: bool,
    /// Additional metadata associated with the artwork.
    pub attributes: Vec<Attribute>,
}

impl Artwork {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, xy, rotation, mirror, flip)) = (
            artwork_name,
            preceded(spaces, x_y_ref),
            preceded(spaces, rot),
            preceded(spaces, mirror),
            preceded(spaces, flip),
        )
            .parse(params)
            .map_err(|err| err.to_owned())?;

        let attributes = Vec::new();

        Ok(Self {
            name,
            xy,
            rotation,
            mirror,
            flip,
            attributes,
        })
    }
}

/// A fiducial marker used for alignment, referencing a pad or padstack.
#[derive(Debug, Clone, PartialEq)]
pub struct Fid {
    /// The name of the fiducial. Must be unique within the component.
    pub name: String,
    /// The name of the pad or padstack used for the fiducial, as defined in `PADS` or `PADSTACKS`.
    pub pad_name: String,
    /// The position of the fiducial's center relative to the component origin.
    pub xy: XYRef,
    /// The layer on which the fiducial is placed, relative to the shape's layer.
    pub layer: Layer,
    /// The rotation of the fiducial around its origin, in degrees counterclockwise.
    pub rotation: Number,
    /// The mirror state of the fiducial. Mirroring is applied before rotation.
    pub mirror: Mirror,
    /// The flip state of the fiducial. Flipping changes the layer of the fiducial and its features.
    pub flip: bool,
    /// Additional metadata associated with the fiducial.
    pub attributes: Vec<Attribute>,
}

impl Fid {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, pad_name, xy, layer, rotation, mirror, flip)) = (
            fid_name,
            preceded(spaces, pad_name),
            preceded(spaces, x_y_ref),
            preceded(spaces, layer),
            preceded(spaces, rot),
            preceded(spaces, mirror),
            preceded(spaces, flip),
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
            flip,
            attributes,
        })
    }
}

/// A text string associated with a component, such as its name or label.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    /// The bottom-left corner of the text relative to the component's origin.
    pub origin: XYRef,
    /// Specifies the text's size, rotation, mirror, layer, and bounding rectangle.
    pub text: TextPar,
}

impl Text {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (origin, text)) = (x_y_ref, preceded(spaces, text_par))
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self { origin, text })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubComponent {
    Artwork(Artwork),
    Fid(Fid),
}

#[derive(Debug, Clone, PartialEq)]
enum ComponentParserState {
    Component,
    SubComponent(SubComponent),
}

/// A component placed on the board, referencing a device and shape definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    /// The name of the component. Must be unique per component.
    pub name: String,
    /// The name of the device that this component references, as defined in the `DEVICES` section.
    pub device: String,
    /// The origin of the component on the board, used as a reference for shape and pin positions.
    pub place: XYRef,
    /// The side of the board this component is placed on. Does not imply mirroring.
    pub layer: Layer,
    /// The counterclockwise rotation of the component in degrees, relative to the shape definition.
    pub rotation: Number,
    /// The shape of the component, as defined in the `SHAPES` section.
    pub shape: Shape,
    /// A list of subcomponents such as artwork or fiducials associated with this component.
    pub subcomponents: Vec<SubComponent>,
    /// A list of text strings associated with this component.
    pub texts: Vec<Text>,
    /// The schematic sheet number, zone, or anything else that is a location property of the component.
    pub sheet: Option<String>,
    /// Miscellaneous information that is relevant to this component.
    pub attributes: Vec<Attribute>,
}

/// A prototype for a component being parsed, used to build a fully constructed `Component`.
#[derive(Debug, Clone, PartialEq)]
struct ComponentPrototype {
    /// The name of the component. Must be unique per component.
    pub name: String,
    /// The name of the device that this component references, as defined in the `DEVICES` section.
    pub device: Option<String>,
    /// The origin of the component on the board, used as a reference for shape and pin positions.
    pub place: Option<XYRef>,
    /// The side of the board this component is placed on. Does not imply mirroring.
    pub layer: Option<Layer>,
    /// The counterclockwise rotation of the component in degrees, relative to the shape definition.
    pub rotation: Option<Number>,
    /// The shape of the component, as defined in the `SHAPES` section.
    pub shape: Option<Shape>,
    /// A list of subcomponents such as artwork or fiducials associated with this component.
    pub subcomponents: Vec<SubComponent>,
    /// A list of text strings associated with this component.
    pub texts: Vec<Text>,
    /// The schematic sheet number, zone, or anything else that is a location property of the component.
    pub sheet: Option<String>,
    /// Miscellaneous information that is relevant to this component.
    pub attributes: Vec<Attribute>,
}

impl ComponentPrototype {
    fn to_component(&self) -> Result<Component, Box<dyn std::error::Error>> {
        let name = self.name.clone();
        let device = self.device.clone().ok_or("")?;
        let place = self.place.ok_or("")?;
        let layer = self.layer.ok_or("")?;
        let rotation = self.rotation.ok_or("")?;
        let shape = self.shape.clone().ok_or("")?;
        let subcomponents = self.subcomponents.clone();
        let texts = self.texts.clone();
        let sheet = self.sheet.clone();
        let attributes = self.attributes.clone();

        Ok(Component {
            name,
            device,
            place,
            layer,
            rotation,
            shape,
            subcomponents,
            texts,
            sheet,
            attributes,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ComponentParser {
    state: ComponentParserState,
    prototype: ComponentPrototype,
}

impl ComponentParser {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, name) = component_name(params).map_err(|err| err.to_owned())?;

        let state = ComponentParserState::Component;
        let name = name.to_string();
        let device = None;
        let place = None;
        let layer = None;
        let rotation = None;
        let shape = None;
        let subcomponents = Vec::new();
        let texts = Vec::new();
        let sheet = None;
        let attributes = Vec::new();

        let shape = ComponentPrototype {
            name,
            device,
            place,
            layer,
            rotation,
            shape,
            subcomponents,
            texts,
            sheet,
            attributes,
        };

        Ok(Self {
            state,
            prototype: shape,
        })
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.state {
            ComponentParserState::Component => match kp.keyword {
                "DEVICE" => {
                    if self.prototype.device.is_none() {
                        let (_, dev) = part_name(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.device = Some(dev);
                    }
                    Ok(())
                }
                "PLACE" => {
                    if self.prototype.place.is_none() {
                        let (_, place) = x_y_ref(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.place = Some(place);
                    }
                    Ok(())
                }
                "LAYER" => {
                    if self.prototype.layer.is_none() {
                        let (_, layer) = layer(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.layer = Some(layer);
                    }
                    Ok(())
                }
                "ROTATION" => {
                    if self.prototype.rotation.is_none() {
                        let (_, rotation) = rot(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.rotation = Some(rotation);
                    }
                    Ok(())
                }
                "SHAPE" => {
                    if self.prototype.shape.is_none() {
                        self.prototype.shape = Some(Shape::from_parameters(kp.parameter)?);
                    }
                    Ok(())
                }
                "TEXT" => {
                    self.prototype
                        .texts
                        .push(Text::from_parameters(kp.parameter)?);
                    Ok(())
                }
                "SHEET" => {
                    if self.prototype.sheet.is_none() {
                        let (_, sheet) = string(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.sheet = Some(sheet);
                    }
                    Ok(())
                }

                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.prototype.attributes.push(attribute);
                    Ok(())
                }

                "ARTWORK" => {
                    let artwork = Artwork::from_parameters(kp.parameter)?;
                    self.state = ComponentParserState::SubComponent(SubComponent::Artwork(artwork));
                    Ok(())
                }
                "FID" => {
                    let fid = Fid::from_parameters(kp.parameter)?;
                    self.state = ComponentParserState::SubComponent(SubComponent::Fid(fid));
                    Ok(())
                }

                _ => Err(format!("Unexpected keyword in component: {}", kp.keyword).into()),
            },
            ComponentParserState::SubComponent(subcomponent) => match kp.keyword {
                "DEVICE" => {
                    self.done();
                    if self.prototype.device.is_none() {
                        let (_, dev) = part_name(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.device = Some(dev);
                    }
                    self.state = ComponentParserState::Component;
                    Ok(())
                }
                "PLACE" => {
                    self.done();
                    if self.prototype.place.is_none() {
                        let (_, place) = x_y_ref(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.place = Some(place);
                    }
                    self.state = ComponentParserState::Component;
                    Ok(())
                }
                "LAYER" => {
                    self.done();
                    if self.prototype.layer.is_none() {
                        let (_, layer) = layer(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.layer = Some(layer);
                    }
                    self.state = ComponentParserState::Component;
                    Ok(())
                }
                "ROTATION" => {
                    self.done();
                    if self.prototype.rotation.is_none() {
                        let (_, rotation) = rot(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.rotation = Some(rotation);
                    }
                    self.state = ComponentParserState::Component;
                    Ok(())
                }
                "SHAPE" => {
                    self.done();
                    if self.prototype.shape.is_none() {
                        self.prototype.shape = Some(Shape::from_parameters(kp.parameter)?);
                    }
                    self.state = ComponentParserState::Component;
                    Ok(())
                }
                "TEXT" => {
                    self.done();
                    self.prototype
                        .texts
                        .push(Text::from_parameters(kp.parameter)?);
                    self.state = ComponentParserState::Component;
                    Ok(())
                }
                "SHEET" => {
                    self.done();
                    if self.prototype.sheet.is_none() {
                        let (_, sheet) = string(kp.parameter).map_err(|err| err.to_owned())?;
                        self.prototype.sheet = Some(sheet);
                    }
                    self.state = ComponentParserState::Component;
                    Ok(())
                }

                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    match subcomponent {
                        SubComponent::Artwork(a) => a.attributes.push(attribute),
                        SubComponent::Fid(f) => f.attributes.push(attribute),
                    }
                    Ok(())
                }

                "ARTWORK" => {
                    self.done();
                    let artwork = Artwork::from_parameters(kp.parameter)?;
                    self.state = ComponentParserState::SubComponent(SubComponent::Artwork(artwork));
                    Ok(())
                }
                "FID" => {
                    self.done();
                    let fid = Fid::from_parameters(kp.parameter)?;
                    self.state = ComponentParserState::SubComponent(SubComponent::Fid(fid));
                    Ok(())
                }

                _ => Err(format!("Unexpected keyword in subcomponent: {}", kp.keyword).into()),
            },
        }
    }

    fn done(&mut self) {
        match &self.state {
            ComponentParserState::Component => (),
            ComponentParserState::SubComponent(subcomponent) => {
                self.prototype.subcomponents.push(subcomponent.clone())
            }
        }
        self.state = ComponentParserState::Component;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ComponentsParserState {
    Reset,
    ComponentParser(ComponentParser),
}

struct ComponentsParser {
    state: ComponentsParserState,
    components: Vec<Component>,
}

impl ComponentsParser {
    fn new() -> Self {
        let state = ComponentsParserState::Reset;
        let components = Vec::new();
        Self { state, components }
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        if let ComponentsParserState::ComponentParser(ref mut parser) = self.state {
            match kp.keyword {
                "COMPONENT" => {
                    parser.done();
                    self.components.push(parser.prototype.to_component()?);
                    let parser = ComponentParser::from_parameters(kp.parameter)?;
                    self.state = ComponentsParserState::ComponentParser(parser);
                    Ok(())
                }
                _ => parser.ingest(kp),
            }
        } else {
            match kp.keyword {
                "COMPONENT" => {
                    let parser = ComponentParser::from_parameters(kp.parameter)?;
                    self.state = ComponentsParserState::ComponentParser(parser);
                    Ok(())
                }
                _ => Err(format!("Unexpected keyword in components: {}", kp.keyword).into()),
            }
        }
    }

    fn finalize(mut self) -> Result<Vec<Component>, Box<dyn std::error::Error>> {
        if let ComponentsParserState::ComponentParser(mut parser) = self.state {
            parser.done();
            self.components.push(parser.prototype.to_component()?);
            self.state = ComponentsParserState::Reset;
        }
        Ok(self.components)
    }
}

/// Parse the `COMPONENTS` section of a GenCAD file.
pub(crate) fn parse_components(
    params: &[KeywordParam],
) -> Result<Vec<Component>, Box<dyn std::error::Error>> {
    let mut sp = ComponentsParser::new();
    for param in params {
        sp.ingest(param)?;
    }
    sp.finalize()
}
