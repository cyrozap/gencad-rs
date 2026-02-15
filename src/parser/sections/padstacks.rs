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

#[derive(Debug, Clone, PartialEq)]
pub struct Pad {
    pub name: String,
    pub layer: Layer,
    pub rotation: Number,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Padstack {
    pub name: String,
    pub drill_size: Number,
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

/// Represents the `PADSTACKS` section of a GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Padstacks {
    pub padstacks: Vec<Padstack>,
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
