// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/padstacks.rs - Parser for the GenCAD PADSTACKS section.
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
use crate::types::util::spaces;
use crate::types::{
    Attribute, Layer, Mirror, Number, attrib_ref, drill_size, layer, mirror, pad_name, rot,
};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_padstacks() {
        let params = vec![
            KeywordParam {
                keyword: "PADSTACK",
                parameter: "p_stack1 -1",
            },
            KeywordParam {
                keyword: "PAD",
                parameter: "p102_4 TOP 180 0",
            },
            KeywordParam {
                keyword: "PAD",
                parameter: "s102_4 BOTTOM 0 0",
            },
            KeywordParam {
                keyword: "PADSTACK",
                parameter: "p_stack2 -1",
            },
            KeywordParam {
                keyword: "PAD",
                parameter: "r_r3 TOP 180 MIRRORX",
            },
            KeywordParam {
                keyword: "PAD",
                parameter: "r_r0 INNER1 180 MIRRORX",
            },
            KeywordParam {
                keyword: "PAD",
                parameter: "r_r0 INNER2 180 MIRRORX",
            },
            KeywordParam {
                keyword: "PAD",
                parameter: "r_r3 BOTTOM 180 MIRRORY",
            },
        ];

        let padstacks = Padstacks::new(&params).unwrap();

        assert_eq!(
            padstacks,
            Padstacks {
                padstacks: vec![
                    Padstack {
                        name: "p_stack1".to_string(),
                        drill_size: -1.0,
                        pads: vec![
                            Pad {
                                name: "p102_4".to_string(),
                                layer: Layer::Top,
                                rotation: 180.0,
                                mirror: Mirror::Not
                            },
                            Pad {
                                name: "s102_4".to_string(),
                                layer: Layer::Bottom,
                                rotation: 0.0,
                                mirror: Mirror::Not
                            }
                        ]
                    },
                    Padstack {
                        name: "p_stack2".to_string(),
                        drill_size: -1.0,
                        pads: vec![
                            Pad {
                                name: "r_r3".to_string(),
                                layer: Layer::Top,
                                rotation: 180.0,
                                mirror: Mirror::MirrorX
                            },
                            Pad {
                                name: "r_r0".to_string(),
                                layer: Layer::InnerX(1),
                                rotation: 180.0,
                                mirror: Mirror::MirrorX
                            },
                            Pad {
                                name: "r_r0".to_string(),
                                layer: Layer::InnerX(2),
                                rotation: 180.0,
                                mirror: Mirror::MirrorX
                            },
                            Pad {
                                name: "r_r3".to_string(),
                                layer: Layer::Bottom,
                                rotation: 180.0,
                                mirror: Mirror::MirrorY
                            }
                        ]
                    }
                ],
                attributes: vec![]
            }
        );
    }
}
