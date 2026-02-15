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
 * # `parser` Module
 *
 * This module provides functionality to parse GenCAD files into structured
 * data.
 *
 * ## Usage Example
 *
 * ```no_run
 * use std::fs::File;
 * use std::io::BufReader;
 *
 * use gencad::parser::ParsedGencadFile;
 *
 * fn main() -> Result<(), Box<dyn std::error::Error>> {
 *     // Open the file
 *     let file = File::open("example.cad")?;
 *     let reader = BufReader::new(file);
 *
 *     // Parse the file
 *     let parsed = ParsedGencadFile::new(reader)?;
 *
 *     // Access parsed data
 *     for section in parsed.sections {
 *         println!("{:?}", section);
 *     }
 *
 *     Ok(())
 * }
 * ```
 */

pub mod sections;
mod types;

use nom::bytes::complete::{is_a, tag, take_till, take_while, take_while1};
use nom::combinator::{fail, map_res};
use nom::multi::{many0, many1};
use nom::sequence::delimited;
use nom::{AsChar, IResult, Parser};

use sections::board::Board;
use sections::components::{Component, parse_components};
use sections::devices::{Device, parse_devices};
use sections::header::Header;
use sections::pads::{Pad, parse_pads};
use sections::padstacks::Padstacks;
use sections::shapes::{Shape, parse_shapes};
use sections::signals::Signals;
use sections::unknown::Unknown;

fn take_newlines(input: &[u8]) -> IResult<&[u8], &[u8]> {
    // Need to consume CR until first LF, then consume all following CRs and LFs
    let (remaining, _) = (
        take_while(|c| c == b'\r'),
        take_while1(AsChar::is_newline),
        take_while(|c| c == b'\r' || c == b'\n'),
    )
        .parse(input)?;
    Ok((remaining, &[]))
}

fn keyword(input: &[u8]) -> IResult<&[u8], &[u8]> {
    is_a("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

fn section_start(input: &[u8]) -> IResult<&[u8], &str> {
    delimited(
        tag(b"$".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
    .parse(input)
}

fn section_end(input: &[u8]) -> IResult<&[u8], &str> {
    delimited(
        tag(b"$END".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
    .parse(input)
}

/// A keyword/parameter pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct KeywordParam<'a> {
    /// The keyword that determines how to interpret the parameter.
    pub keyword: &'a str,
    /// The parameter associated with the keyword.
    pub parameter: &'a str,
}

impl<'a> KeywordParam<'a> {
    fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (remaining, (keyword, _, parameter, _)) = (
            map_res(keyword, str::from_utf8),
            tag(b" ".as_slice()),
            map_res(take_till(|c| c == b'\r' || c == b'\n'), str::from_utf8),
            take_newlines,
        )
            .parse(input)?;
        let kp = Self { keyword, parameter };
        Ok((remaining, kp))
    }
}

/// A section in the GenCAD file.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Section<'a> {
    /// The name of the section.
    name: &'a str,
    /// List of parameters in the section.
    parameters: Vec<KeywordParam<'a>>,
}

impl<'a> Section<'a> {
    fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (remaining, (start_tag, parameters, end_tag)) =
            (section_start, many0(KeywordParam::parse), section_end).parse(input)?;

        if start_tag != end_tag {
            return fail().parse(input);
        }

        let name = start_tag;

        Ok((remaining, Self { name, parameters }))
    }
}

fn sections<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<Section<'a>>> {
    many1(Section::parse).parse(input)
}

/// A section in the GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedSection {
    Header(Header),
    Board(Board),
    Pads(Vec<Pad>),
    Padstacks(Padstacks),
    Shapes(Vec<Shape>),
    Components(Vec<Component>),
    Devices(Vec<Device>),
    Signals(Signals),
    Unknown(Unknown),
}

/// A fully parsed GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedGencadFile {
    /// The parsed sections of the file.
    pub sections: Vec<ParsedSection>,
}

impl ParsedGencadFile {
    /// Parses a GenCAD file into a structured format.
    ///
    /// # Arguments
    ///
    /// * `reader` - A reader over the file data.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed file or an error.
    pub fn new<R: std::io::Read>(mut reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let (remaining, unparsed_sections) = sections(&buffer).map_err(|err| err.to_owned())?;

        if !remaining.is_empty() {
            return Err(format!("Unparsed data remaining in file: {:?}", remaining).into());
        }

        let mut sections = Vec::new();

        for section in unparsed_sections {
            match section.name {
                "HEADER" => sections.push(ParsedSection::Header(Header::new(&section.parameters)?)),
                "BOARD" => sections.push(ParsedSection::Board(Board::new(&section.parameters)?)),
                "PADS" => sections.push(ParsedSection::Pads(parse_pads(&section.parameters)?)),
                "PADSTACKS" => sections.push(ParsedSection::Padstacks(Padstacks::new(
                    &section.parameters,
                )?)),
                "SHAPES" => {
                    sections.push(ParsedSection::Shapes(parse_shapes(&section.parameters)?))
                }
                "COMPONENTS" => sections.push(ParsedSection::Components(parse_components(
                    &section.parameters,
                )?)),
                "DEVICES" => {
                    sections.push(ParsedSection::Devices(parse_devices(&section.parameters)?))
                }
                "SIGNALS" => {
                    sections.push(ParsedSection::Signals(Signals::new(&section.parameters)?))
                }
                unknown_name => sections.push(ParsedSection::Unknown(Unknown::new(
                    &unknown_name,
                    &section.parameters,
                )?)),
            }
        }

        Ok(Self { sections })
    }
}
