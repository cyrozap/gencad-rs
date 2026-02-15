// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD SIGNALS section.
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
    attrib_ref, component_name, layer, pin_name, probe, sig_name, tan, tin, tp_name, x_y_ref,
};
use crate::types::{Attribute, Layer, XYRef};

/// A connection point on a component, defined by its component and pin names.
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    /// The name of the component as defined in the `COMPONENTS` section.
    pub component_name: String,
    /// The name of the pin as defined in the `SHAPES` section.
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

/// A preferred test point location for a signal, used in bed-of-nails testing.
#[derive(Debug, Clone, PartialEq)]
pub struct NailLoc {
    /// The name of the component as defined in the `COMPONENTS` section.
    pub component_name: String,
    /// The name of the pin as defined in the `SHAPES` section.
    pub pin_name: String,
    /// The test pin name. Use "-1" if no unique name is assigned.
    pub tp_name: String,
    /// The absolute coordinate of the nail location. Use "-32767 -32767" to inherit the node's position.
    pub xy: XYRef,
    /// Tester Assigned Number. Use "-1" if undefined.
    pub tan: String,
    /// Tester Interface Name. Use "-1" if undefined.
    pub tin: String,
    /// Probe type (e.g., "100T", "75C"). Use "-1" if undefined.
    pub probe: String,
    /// The layer on which the probe is applied. Only [Layer::Top] or [Layer::Bottom] are valid.
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

/// A signal or net defined in the `SIGNALS` section, representing electrical connectivity.
#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    /// The unique name of the signal or net, used in the `ROUTES` section.
    pub name: String,
    /// A list of [Node] objects defining the connections to components and pins.
    pub nodes: Vec<Node>,
    /// A list of [NailLoc] objects defining preferred test point locations for this signal.
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

/// Represents the `SIGNALS` section of a GenCAD file, defining all connectivity information.
#[derive(Debug, Clone, PartialEq)]
pub struct Signals {
    /// A list of all defined signals and their connections.
    pub signals: Vec<Signal>,
    /// Additional metadata associated with the `SIGNALS` section.
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
