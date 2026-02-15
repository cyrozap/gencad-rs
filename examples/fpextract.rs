// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  fpextract.rs - Footprint extraction demo for GenCAD files.
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

use std::fs;
use std::fs::File;
use std::fs::create_dir_all;
use std::io::BufReader;
use std::path::Path;

use chrono;
use clap::Parser;

use gencad::interpreter::InterpretedGencadFile;
use gencad::parser::ParsedGencadFile;
use gencad::parser::sections::pads::{Pad, PadShape};
use gencad::parser::sections::shapes::SubShape;
use gencad::types::{CircleRef, PadType, XYRef};
use gencad::types::{Dimension, Number};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file to read.
    file: String,
}

fn units_to_mm(value_units: Number, units: &Dimension) -> Number {
    match units {
        Dimension::Inch => value_units * 25.4,
        Dimension::Thou => value_units * 0.0254,
        Dimension::Mm => value_units,
        Dimension::Mm100 => value_units * 0.01,
        Dimension::User(units_per_inch) => {
            (25.4 * value_units) / <u16 as Into<Number>>::into(*units_per_inch)
        }
        Dimension::UserM(units_per_cm) => {
            (10.0 * value_units) / <u16 as Into<Number>>::into(*units_per_cm)
        }
        Dimension::UserMm(units_per_mm) => value_units / <u16 as Into<Number>>::into(*units_per_mm),
    }
}

fn mm_to_units(value_mm: Number, units: &Dimension) -> Number {
    match units {
        Dimension::Inch => value_mm / 25.4,
        Dimension::Thou => value_mm / 0.0254,
        Dimension::Mm => value_mm,
        Dimension::Mm100 => value_mm * 100.0,
        Dimension::User(units_per_inch) => {
            (value_mm * <u16 as Into<Number>>::into(*units_per_inch)) / 25.4
        }
        Dimension::UserM(units_per_cm) => {
            (value_mm * <u16 as Into<Number>>::into(*units_per_cm)) / 10.0
        }
        Dimension::UserMm(units_per_mm) => value_mm * <u16 as Into<Number>>::into(*units_per_mm),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let file = File::open(&args.file)?;
    let reader = BufReader::new(file);

    let parsed = ParsedGencadFile::new(reader)?;
    let interpreted = InterpretedGencadFile::new(parsed)?;

    // Create output directory based on input filename
    let base_name = Path::new(&args.file)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    let input_dir = Path::new(&args.file)
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let output_dir = input_dir.join(format!("{}.pretty", base_name));
    create_dir_all(&output_dir)?;

    // Generate .kicad_mod files for each footprint
    for (name, component) in &interpreted.components {
        let mut content = String::new();

        let shape = interpreted
            .shapes
            .get(&component.shape.name)
            .ok_or(format!(
                "Shape not found for component {}: {}",
                name, component.shape.name
            ))?;

        let device = interpreted.devices.get(&component.device);

        let part = device
            .as_ref()
            .and_then(|d| d.part.as_ref().cloned())
            .unwrap_or_default();

        let description = device
            .as_ref()
            .and_then(|d| d.desc.as_ref().cloned())
            .unwrap_or_default();

        // Write KiCad footprint header
        content.push_str(&format!("(footprint \"{}\"\n", name));
        let now = chrono::Utc::now();
        let current_date = now.format("%Y%m%d").to_string();
        content.push_str(&format!("  (version {})\n", current_date));
        content.push_str("  (generator gencad_fpextract)\n");
        if !description.is_empty() {
            content.push_str(&format!("  (descr \"{}\")\n", description));
        } else if !part.is_empty() {
            content.push_str(&format!("  (descr \"{}\")\n", part));
        }
        content.push_str("  (tags \"generated\")\n");

        // Add reference and value text
        content.push_str("  (property \"Reference\" \"U\" (at 0 0) (effects (font (size 1 1) (thickness 0.15))))\n");
        content.push_str("  (property \"Value\" \"U1\" (at 0 1.5) (effects (font (size 1 1) (thickness 0.15))))\n");

        // Add reference and value text objects
        content.push_str("  (fp_text reference \"U\" (at 0 0) (layer \"F.SilkS\")\n");
        content.push_str("    (effects (font (size 1 1) (thickness 0.15)))\n");
        content.push_str("  )\n");
        content.push_str("  (fp_text value \"U1\" (at 0 1.5) (layer \"F.Fab\")\n");
        content.push_str("    (effects (font (size 1 1) (thickness 0.15)))\n");
        content.push_str("  )\n");

        let default_pad = Pad {
            name: "default".to_string(),
            ptype: PadType::Round,
            drill_size: 0.0,
            shapes: vec![PadShape::Circle(CircleRef {
                center: XYRef { x: 0.0, y: 0.0 },
                radius: mm_to_units(0.5, &interpreted.header.units),
            })],
            attributes: Vec::new(),
        };

        // Add pads for each pin
        for subshape in &shape.subshapes {
            if let SubShape::Pin(pin) = subshape {
                let pad = interpreted.pads.get(&pin.pad_name).unwrap_or(&default_pad);

                // Convert to KiCad coordinates
                let x = pin.xy.x;
                let y = -pin.xy.y;

                // Convert to KiCad units
                let x_mm = units_to_mm(x, &interpreted.header.units);
                let y_mm = units_to_mm(y, &interpreted.header.units);

                // Get the radius from the pad's shape if it is a circle
                let radius = pad
                    .shapes
                    .iter()
                    .find_map(|shape| {
                        if let PadShape::Circle(circle) = shape {
                            Some(circle.radius)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(mm_to_units(0.5, &interpreted.header.units)); // Default radius of 0.5mm if no circle found

                let radius_mm = units_to_mm(radius, &interpreted.header.units);

                content.push_str(&format!(
                    "  (pad \"{}\" smd circle (at {:.6} {:.6}) (size {:.6} {:.6}) (layers F.Cu F.Paste F.Mask)\n",
                    pin.name,
                    x_mm,
                    y_mm,
                    radius_mm,
                    radius_mm
                ));
                content.push_str("  )\n");
            }
        }

        // Close the footprint
        content.push_str(")\n");

        // Write to file
        let filename = format!("{}/{}.kicad_mod", output_dir.display(), name);
        if let Err(e) = fs::write(&filename, content) {
            eprintln!("Failed to write file {}: {}", filename, e);
        }
    }

    Ok(())
}
