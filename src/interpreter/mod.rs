// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Interpreter module for GenCAD files.
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

/*!
 * # `interpreter` Module
 *
 * This module provides functionality to interpret parsed GenCAD files.
 *
 * ## Usage Example
 *
 * ```no_run
 * use std::fs::File;
 * use std::io::BufReader;
 *
 * use gencad::parser::ParsedGencadFile;
 * use gencad::interpreter::InterpretedGencadFile;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     // Open the file
 *     let file = File::open("example.cad")?;
 *     let reader = BufReader::new(file);
 *
 *     // Parse the file
 *     let parsed = ParsedGencadFile::new(reader)?;
 *
 *     // Interpret the parsed file
 *     let interpreted = InterpretedGencadFile::new(parsed)?;
 *
 *     // Access interpreted data
 *     println!("GenCAD version: {}", interpreted.header.gencad_version);
 *
 *     Ok(())
 * }
 * ```
 */

use std::collections::HashMap;

use crate::parser::sections::components::Component;
use crate::parser::sections::devices::Device;
use crate::parser::sections::header::Header;
use crate::parser::sections::pads::Pad;
use crate::parser::sections::padstacks::Padstack;
use crate::parser::sections::shapes::Shape;
use crate::parser::{ParsedGencadFile, ParsedSection};

/// A fully interpreted GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct InterpretedGencadFile {
    pub header: Header,
    pub pads: HashMap<String, Pad>,
    pub padstacks: HashMap<String, Padstack>,
    pub shapes: HashMap<String, Shape>,
    pub components: HashMap<String, Component>,
    pub devices: HashMap<String, Device>,
}

impl InterpretedGencadFile {
    pub fn new(parsed: ParsedGencadFile) -> Result<Self, Box<dyn std::error::Error>> {
        let mut header_section = None;
        let mut pads_section = None;
        let mut padstacks_section = None;
        let mut shapes_section = None;
        let mut components_section = None;
        let mut devices_section = None;

        for section in parsed.sections {
            match section {
                ParsedSection::Header(s) => header_section = Some(s),
                ParsedSection::Pads(s) => pads_section = Some(s),
                ParsedSection::Padstacks(s) => padstacks_section = Some(s),
                ParsedSection::Shapes(s) => shapes_section = Some(s),
                ParsedSection::Components(s) => components_section = Some(s),
                ParsedSection::Devices(s) => devices_section = Some(s),
                _ => (),
            }
        }

        let header =
            header_section.ok_or_else(|| "Missing header section in GenCAD file".to_owned())?;

        let mut pads = HashMap::new();
        if let Some(pads_vec) = pads_section {
            for pad in pads_vec {
                pads.insert(pad.name.clone(), pad);
            }
        }

        let mut padstacks = HashMap::new();
        if let Some(p) = padstacks_section {
            for padstack in p.padstacks {
                padstacks.insert(padstack.name.clone(), padstack);
            }
        }

        let mut shapes = HashMap::new();
        if let Some(shapes_vec) = shapes_section {
            for shape in shapes_vec {
                shapes.insert(shape.name.clone(), shape);
            }
        }

        let mut components = HashMap::new();
        if let Some(components_vec) = components_section {
            for component in components_vec {
                components.insert(component.name.clone(), component);
            }
        }

        let mut devices = HashMap::new();
        if let Some(devices_vec) = devices_section {
            for device in devices_vec {
                devices.insert(device.name.clone(), device);
            }
        }

        Ok(Self {
            header,
            pads,
            padstacks,
            shapes,
            components,
            devices,
        })
    }
}
