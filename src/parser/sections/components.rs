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

#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub name: String,
    pub mirror: Mirror,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    pub name: String,
    pub xy: XYRef,
    pub rotation: Number,
    pub mirror: Mirror,
    pub flip: bool,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Fid {
    pub name: String,
    pub pad_name: String,
    pub xy: XYRef,
    pub layer: Layer,
    pub rotation: Number,
    pub mirror: Mirror,
    pub flip: bool,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub origin: XYRef,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub name: String,
    pub device: String,
    pub place: XYRef,
    pub layer: Layer,
    pub rotation: Number,
    pub shape: Shape,
    pub subcomponents: Vec<SubComponent>,
    pub texts: Vec<Text>,
    pub sheet: Option<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
struct ComponentPrototype {
    pub name: String,
    pub device: Option<String>,
    pub place: Option<XYRef>,
    pub layer: Option<Layer>,
    pub rotation: Option<Number>,
    pub shape: Option<Shape>,
    pub subcomponents: Vec<SubComponent>,
    pub texts: Vec<Text>,
    pub sheet: Option<String>,
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
