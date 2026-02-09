// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser library for GenCAD files.
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
 * # `gencad` Crate
 *
 * A library for parsing GenCAD files.
 *
 * This crate provides a full pipeline for working with GenCAD files:
 *
 * 1. [parser]: Converts the file bytes into structured data.
 * 2. [interpreter]: Interprets the parsed file to create a higher-level
 *    representation of the objects in the file.
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

pub mod interpreter;
pub mod parser;
pub mod serialization;
pub mod types;
