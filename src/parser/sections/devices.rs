// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD DEVICES section.
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
use crate::parser::types::{attrib_ref, p_integer, part_name, pin_name, string};
use crate::types::Attribute;

/// A pin description for a device, typically used to describe the function or role of a pin.
#[derive(Debug, Clone, PartialEq)]
pub struct PinDesc {
    /// The name of the pin as defined in the device's shape and used in signals.
    pub pin_name: String,
    /// A textual description of the pin's purpose or function.
    pub text: String,
}

impl PinDesc {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (pin_name, text)) = (pin_name, preceded(spaces, string))
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self { pin_name, text })
    }
}

/// A pin function for a device, typically used to describe the pin's role in tester output data.
#[derive(Debug, Clone, PartialEq)]
pub struct PinFunct {
    /// The name of the pin as defined in the device's shape and used in signals.
    pub pin_name: String,
    /// A textual description of the pin's functional behavior.
    pub text: String,
}

impl PinFunct {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, (pin_name, text)) = (pin_name, preceded(spaces, string))
            .parse(params)
            .map_err(|err| err.to_owned())?;

        Ok(Self { pin_name, text })
    }
}

/// A device used on the board. These descriptions are independent of board geometry and are used for cross-referencing components.
#[derive(Debug, Clone, PartialEq)]
pub struct Device {
    /// The name of the device. Must be unique per device.
    pub name: String,
    /// An optional corporate part number or CAD part name. Does not need to be unique.
    pub part: Option<String>,
    /// An optional field to define the component type (e.g., "RES" for resistor).
    pub dtype: Option<String>,
    /// An optional field to enhance the `dtype` with additional information.
    pub style: Option<String>,
    /// An optional field to define the physical package type (e.g., "DIL_8", "TO99").
    pub package: Option<String>,
    /// A list of pin descriptions, typically used for CAD data.
    pub pin_descriptions: Vec<PinDesc>,
    /// A list of pin functions, typically used for tester output data.
    pub pin_functions: Vec<PinFunct>,
    /// An optional field to define the number of physical pins on the device.
    pub pincount: Option<u16>,
    /// An optional field to define the value of the device (e.g., resistance for resistors).
    pub value: Option<String>,
    /// An optional field to define a ± tolerance for the device.
    pub tol: Option<String>,
    /// An optional field to define the negative (minimum) tolerance for the device.
    pub ntol: Option<String>,
    /// An optional field to define the positive (maximum) tolerance for the device.
    pub ptol: Option<String>,
    /// An optional field to define a voltage rating for the device.
    pub volts: Option<String>,
    /// An optional free text field to describe the device.
    pub desc: Option<String>,
    /// A list of additional attributes associated with the device.
    pub attributes: Vec<Attribute>,
}

impl Device {
    fn from_parameters(params: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (_, name) = part_name(params).map_err(|err| err.to_owned())?;

        Ok(Self {
            name,
            part: None,
            dtype: None,
            style: None,
            package: None,
            pin_descriptions: Vec::new(),
            pin_functions: Vec::new(),
            pincount: None,
            value: None,
            tol: None,
            ntol: None,
            ptol: None,
            volts: None,
            desc: None,
            attributes: Vec::new(),
        })
    }

    fn update(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        match kp.keyword {
            "PART" => {
                if self.part.is_none() {
                    let (_, dev) = part_name(kp.parameter).map_err(|err| err.to_owned())?;
                    self.part = Some(dev);
                }
                Ok(())
            }
            "TYPE" => {
                if self.dtype.is_none() {
                    let (_, dtype) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.dtype = Some(dtype);
                }
                Ok(())
            }
            "STYLE" => {
                if self.style.is_none() {
                    let (_, style) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.style = Some(style);
                }
                Ok(())
            }
            "PACKAGE" => {
                if self.package.is_none() {
                    let (_, package) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.package = Some(package);
                }
                Ok(())
            }
            "PINDESC" => {
                self.pin_descriptions
                    .push(PinDesc::from_parameters(kp.parameter)?);
                Ok(())
            }
            "PINFUNCT" => {
                self.pin_functions
                    .push(PinFunct::from_parameters(kp.parameter)?);
                Ok(())
            }
            "PINCOUNT" => {
                if self.pincount.is_none() {
                    let (_, pincount) = p_integer(kp.parameter).map_err(|err| err.to_owned())?;
                    self.pincount = Some(pincount);
                }
                Ok(())
            }
            "VALUE" => {
                if self.value.is_none() {
                    let (_, value) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.value = Some(value);
                }
                Ok(())
            }
            "TOL" => {
                if self.tol.is_none() {
                    let (_, tol) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.tol = Some(tol);
                }
                Ok(())
            }
            "NTOL" => {
                if self.ntol.is_none() {
                    let (_, ntol) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.ntol = Some(ntol);
                }
                Ok(())
            }
            "PTOL" => {
                if self.ptol.is_none() {
                    let (_, ptol) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.ptol = Some(ptol);
                }
                Ok(())
            }
            "VOLTS" => {
                if self.volts.is_none() {
                    let (_, volts) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.volts = Some(volts);
                }
                Ok(())
            }
            "DESC" => {
                if self.desc.is_none() {
                    let (_, desc) = string(kp.parameter).map_err(|err| err.to_owned())?;
                    self.desc = Some(desc);
                }
                Ok(())
            }
            "ATTRIBUTE" => {
                let (_, attribute) = attrib_ref(kp.parameter).map_err(|err| err.to_owned())?;
                self.attributes.push(attribute);
                Ok(())
            }

            _ => Err(format!("Unexpected keyword in component: {}", kp.keyword).into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DevicesParserState {
    Reset,
    Device(Device),
}

struct DevicesParser {
    state: DevicesParserState,
    devices: Vec<Device>,
}

impl DevicesParser {
    fn new() -> Self {
        let state = DevicesParserState::Reset;
        let devices = Vec::new();
        Self { state, devices }
    }

    fn ingest(&mut self, kp: &KeywordParam) -> Result<(), Box<dyn std::error::Error>> {
        if let DevicesParserState::Device(ref mut device) = self.state {
            match kp.keyword {
                "DEVICE" => {
                    self.devices.push(device.clone());
                    let device = Device::from_parameters(kp.parameter)?;
                    self.state = DevicesParserState::Device(device);
                    Ok(())
                }
                _ => device.update(kp),
            }
        } else {
            match kp.keyword {
                "DEVICE" => {
                    let device = Device::from_parameters(kp.parameter)?;
                    self.state = DevicesParserState::Device(device);
                    Ok(())
                }
                _ => Err(format!("Unexpected keyword in devices: {}", kp.keyword).into()),
            }
        }
    }

    fn finalize(mut self) -> Result<Vec<Device>, Box<dyn std::error::Error>> {
        if let DevicesParserState::Device(device) = self.state {
            self.devices.push(device.clone());
            self.state = DevicesParserState::Reset;
        }
        Ok(self.devices)
    }
}

/// Parse the `DEVICES` section of a GenCAD file.
pub(crate) fn parse_devices(
    params: &[KeywordParam],
) -> Result<Vec<Device>, Box<dyn std::error::Error>> {
    let mut sp = DevicesParser::new();
    for param in params {
        sp.ingest(param)?;
    }
    sp.finalize()
}
