// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD PADSTACKS section.
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
use crate::parser::types::{attrib_ref, drill_size, layer, mirror, pad_name, rot};
use crate::types::{Attribute, Layer, Mirror, Number};

/// A single pad within a [Padstack]. All pads in a stack share the same origin.
#[derive(Debug, Clone, PartialEq)]
pub struct Pad {
    /// The name of the pad, referencing a definition in the `PADS` section.
    pub name: String,
    /// The board layer on which the pad is placed (e.g., [Layer::Top], [Layer::Inner]).
    /// Mirroring the pad stack will invert this layer.
    pub layer: Layer,
    /// The rotation angle in degrees, counter-clockwise from the pad's original definition.
    /// Applied after any mirroring.
    pub rotation: Number,
    /// The mirror state of the pad (applied before rotation).
    pub mirror: Mirror,
}

impl Pad {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, layer, rotation, mirror)) = (
            pad_name,
            preceded(spaces, layer),
            preceded(spaces, rot),
            preceded(spaces, mirror),
        )
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self {
            name,
            layer,
            rotation,
            mirror,
        })
    }
}

/// A collection of pads arranged to form a single logical pad stack.
/// Used to define complex pad arrangements for components.
#[derive(Debug, Clone, PartialEq)]
pub struct Padstack {
    /// A unique name for this pad stack. Must not conflict with any pad name in the `PADS` section.
    /// If undefined, sequential names like "padstack1", "padstack2", etc., should be used.
    pub name: String,
    /// The drill hole size in [crate::types::Dimension] units for the entire stack.
    /// - `0.0` means no hole.
    /// - `-1.0` means undefined (use individual pad definitions).
    /// - `-2.0` means use the drill size of the first pad in the stack.
    pub drill_size: Number,
    /// The list of pads in this stack. All pads share the same origin.
    pub pads: Vec<Pad>,
}

impl Padstack {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (name, drill_size)) = (pad_name, preceded(spaces, drill_size))
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self {
            name,
            drill_size,
            pads: Vec::new(),
        })
    }

    fn update(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        match kp.keyword {
            "PAD" => {
                self.pads.push(Pad::from_parameters(kp.parameter)?);
                Ok(())
            }

            _ => Err(format!("Unexpected keyword in padstack: {}", kp.keyword).into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PadstacksParserState {
    Reset,
    Device(Padstack),
}

struct PadstacksParser {
    state: PadstacksParserState,
    padstacks: Vec<Padstack>,
    attributes: Vec<Attribute>,
}

impl PadstacksParser {
    fn new() -> Self {
        let state = PadstacksParserState::Reset;
        let padstacks = Vec::new();
        let attributes = Vec::new();
        Self {
            state,
            padstacks,
            attributes,
        }
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        if let PadstacksParserState::Device(ref mut device) = self.state {
            match kp.keyword {
                "PADSTACK" => {
                    self.padstacks.push(device.clone());
                    let device = Padstack::from_parameters(kp.parameter)?;
                    self.state = PadstacksParserState::Device(device);
                    Ok(())
                }
                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.attributes.push(attribute);
                    Ok(())
                }
                _ => device.update(kp),
            }
        } else {
            match kp.keyword {
                "PADSTACK" => {
                    let device = Padstack::from_parameters(kp.parameter)?;
                    self.state = PadstacksParserState::Device(device);
                    Ok(())
                }
                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.attributes.push(attribute);
                    Ok(())
                }
                _ => Err(format!("Unexpected keyword in padstacks: {}", kp.keyword).into()),
            }
        }
    }

    fn finalize(mut self) -> Result<Padstacks, Box<dyn std::error::Error>> {
        if let PadstacksParserState::Device(device) = self.state {
            self.padstacks.push(device.clone());
            self.state = PadstacksParserState::Reset;
        }
        Ok(Padstacks {
            padstacks: self.padstacks,
            attributes: self.attributes,
        })
    }
}

/// Represents the optional `PADSTACKS` section of a GenCAD file.
/// Defines how multiple pads can be grouped into a single logical pad stack.
#[derive(Debug, Clone, PartialEq)]
pub struct Padstacks {
    /// All defined pad stacks in the section.
    pub padstacks: Vec<Padstack>,
    /// Additional metadata associated with the `PADSTACKS` section.
    pub attributes: Vec<Attribute>,
}

impl Padstacks {
    pub(crate) fn new(params: &[KeywordParam]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut p = PadstacksParser::new();
        for param in params {
            p.ingest(param)?;
        }
        p.finalize()
    }
}
