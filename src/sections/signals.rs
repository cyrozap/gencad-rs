// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/signals.rs - Parser for the GenCAD SIGNALS section.
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
    Attribute, Layer, XYRef, attrib_ref, component_name, layer, pin_name, probe, sig_name, tan,
    tin, tp_name, x_y_ref,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub component_name: String,
    pub pin_name: String,
}

impl Node {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (component_name, pin_name)) = (component_name, preceded(spaces, pin_name))
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self {
            component_name,
            pin_name,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NailLoc {
    pub component_name: String,
    pub pin_name: String,
    pub tp_name: String,
    pub xy: XYRef,
    pub tan: String,
    pub tin: String,
    pub probe: String,
    pub layer: Layer,
}

impl NailLoc {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (component_name, pin_name, tp_name, xy, tan, tin, probe, layer)) = (
            component_name,
            preceded(spaces, pin_name),
            preceded(spaces, tp_name),
            preceded(spaces, x_y_ref),
            preceded(spaces, tan),
            preceded(spaces, tin),
            preceded(spaces, probe),
            preceded(spaces, layer),
        )
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self {
            component_name,
            pin_name,
            tp_name,
            xy,
            tan,
            tin,
            probe,
            layer,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    pub name: String,
    pub nodes: Vec<Node>,
    pub nail_locations: Vec<NailLoc>,
}

impl Signal {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, name) = sig_name(params).map_err(|err| err.to_owned())?;

        Ok(Self {
            name,
            nodes: Vec::new(),
            nail_locations: Vec::new(),
        })
    }

    fn update(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        match kp.keyword {
            "NODE" => {
                self.nodes.push(Node::from_parameters(kp.parameter)?);
                Ok(())
            }
            "NAILLOC" => {
                self.nail_locations
                    .push(NailLoc::from_parameters(kp.parameter)?);
                Ok(())
            }

            _ => Err(format!("Unexpected keyword in signal: {}", kp.keyword).into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SignalsParserState {
    Reset,
    Device(Signal),
}

struct SignalsParser {
    state: SignalsParserState,
    signals: Vec<Signal>,
    attributes: Vec<Attribute>,
}

impl SignalsParser {
    fn new() -> Self {
        let state = SignalsParserState::Reset;
        let signals = Vec::new();
        let attributes = Vec::new();
        Self {
            state,
            signals,
            attributes,
        }
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        if let SignalsParserState::Device(ref mut device) = self.state {
            match kp.keyword {
                "SIGNAL" => {
                    self.signals.push(device.clone());
                    let device = Signal::from_parameters(kp.parameter)?;
                    self.state = SignalsParserState::Device(device);
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
                "SIGNAL" => {
                    let device = Signal::from_parameters(kp.parameter)?;
                    self.state = SignalsParserState::Device(device);
                    Ok(())
                }
                "ATTRIBUTE" => {
                    let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                    self.attributes.push(attribute);
                    Ok(())
                }
                _ => Err(format!("Unexpected keyword in signals: {}", kp.keyword).into()),
            }
        }
    }

    fn finalize(mut self) -> Result<Signals, Box<dyn std::error::Error>> {
        if let SignalsParserState::Device(device) = self.state {
            self.signals.push(device.clone());
            self.state = SignalsParserState::Reset;
        }
        Ok(Signals {
            signals: self.signals,
            attributes: self.attributes,
        })
    }
}

/// Represents the `SIGNALS` section of a GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Signals {
    pub signals: Vec<Signal>,
    pub attributes: Vec<Attribute>,
}

impl Signals {
    pub(crate) fn new(params: &[KeywordParam]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut p = SignalsParser::new();
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
    fn test_example_signals() {
        let params = vec![
            KeywordParam {
                keyword: "SIGNAL",
                parameter: "data_bus_7",
            },
            KeywordParam {
                keyword: "NODE",
                parameter: "IC3 2",
            },
            KeywordParam {
                keyword: "NODE",
                parameter: "R2 2",
            },
            KeywordParam {
                keyword: "NAILLOC",
                parameter: "R2 2 -1 500 2500 -1 -1 100T BOTTOM",
            },
            KeywordParam {
                keyword: "NODE",
                parameter: "IC4 2",
            },
            KeywordParam {
                keyword: "NODE",
                parameter: "6Ic2 p34A",
            },
            KeywordParam {
                keyword: "NAILLOC",
                parameter: "6Ic2 p34A -1 800 3000 -1 -1 75T BOTTOM",
            },
            KeywordParam {
                keyword: "SIGNAL",
                parameter: "ADDRESS_BUS_4",
            },
            KeywordParam {
                keyword: "NODE",
                parameter: "U1 2",
            },
            KeywordParam {
                keyword: "NODE",
                parameter: "PL12 132",
            },
            KeywordParam {
                keyword: "NAILLOC",
                parameter: "PL12 132 -1 200 200 -1 -1 100T BOTTOM",
            },
        ];

        let signals = Signals::new(&params).unwrap();

        assert_eq!(
            signals,
            Signals {
                signals: vec![
                    Signal {
                        name: "data_bus_7".to_string(),
                        nodes: vec![
                            Node {
                                component_name: "IC3".to_string(),
                                pin_name: "2".to_string()
                            },
                            Node {
                                component_name: "R2".to_string(),
                                pin_name: "2".to_string()
                            },
                            Node {
                                component_name: "IC4".to_string(),
                                pin_name: "2".to_string()
                            },
                            Node {
                                component_name: "6Ic2".to_string(),
                                pin_name: "p34A".to_string()
                            }
                        ],
                        nail_locations: vec![
                            NailLoc {
                                component_name: "R2".to_string(),
                                pin_name: "2".to_string(),
                                tp_name: "-1".to_string(),
                                xy: XYRef {
                                    x: 500.0,
                                    y: 2500.0
                                },
                                tan: "-1".to_string(),
                                tin: "-1".to_string(),
                                probe: "100T".to_string(),
                                layer: Layer::Bottom
                            },
                            NailLoc {
                                component_name: "6Ic2".to_string(),
                                pin_name: "p34A".to_string(),
                                tp_name: "-1".to_string(),
                                xy: XYRef {
                                    x: 800.0,
                                    y: 3000.0
                                },
                                tan: "-1".to_string(),
                                tin: "-1".to_string(),
                                probe: "75T".to_string(),
                                layer: Layer::Bottom
                            }
                        ]
                    },
                    Signal {
                        name: "ADDRESS_BUS_4".to_string(),
                        nodes: vec![
                            Node {
                                component_name: "U1".to_string(),
                                pin_name: "2".to_string()
                            },
                            Node {
                                component_name: "PL12".to_string(),
                                pin_name: "132".to_string()
                            }
                        ],
                        nail_locations: vec![NailLoc {
                            component_name: "PL12".to_string(),
                            pin_name: "132".to_string(),
                            tp_name: "-1".to_string(),
                            xy: XYRef { x: 200.0, y: 200.0 },
                            tan: "-1".to_string(),
                            tin: "-1".to_string(),
                            probe: "100T".to_string(),
                            layer: Layer::Bottom
                        }]
                    }
                ],
                attributes: vec![]
            }
        );
    }
}
