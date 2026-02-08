// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/parser.rs - Parser library for GenCAD files.
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
 *         println!("{}", section.name);
 *     }
 *
 *     Ok(())
 * }
 * ```
 */

use nom::bytes::complete::{is_a, tag, take_till, take_while, take_while1};
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::{AsChar, IResult, Parser};

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
    let (remaining, output) = (
        tag(b"$".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
        .parse(input)?;
    let keyword = output.1;
    Ok((remaining, keyword))
}

fn section_end(input: &[u8]) -> IResult<&[u8], &str> {
    let (remaining, output) = (
        tag(b"$END".as_slice()),
        map_res(keyword, str::from_utf8),
        take_newlines,
    )
        .parse(input)?;
    let keyword = output.1;
    Ok((remaining, keyword))
}

/// A keyword/parameter pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordParam {
    /// The keyword that determines how to interpret the parameter.
    pub keyword: String,
    /// The parameter associated with the keyword.
    pub parameter: String,
}

impl KeywordParam {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, output) = (
            map_res(keyword, str::from_utf8),
            tag(b" ".as_slice()),
            map_res(take_till(|c| c == b'\r' || c == b'\n'), str::from_utf8),
            take_newlines,
        )
            .parse(input)?;
        let keyword = output.0.to_string();
        let parameter = output.2.to_string();
        let kp = Self { keyword, parameter };
        Ok((remaining, kp))
    }
}

/// A section in the GenCAD file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// The name of the section.
    pub name: String,
    /// List of parameters in the section.
    pub parameters: Vec<KeywordParam>,
}

impl Section {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, output) =
            (section_start, many0(KeywordParam::parse), section_end).parse(input)?;
        let name = output.0.to_string();
        let parameters = output.1;

        // TODO: Match section end name with section start name?

        Ok((remaining, Self { name, parameters }))
    }
}

/// A fully parsed GenCAD file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedGencadFile {
    /// The parsed sections of the file.
    pub sections: Vec<Section>,
}

impl ParsedGencadFile {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (remaining, sections) = many1(Section::parse).parse(input)?;

        Ok((remaining, Self { sections }))
    }

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

        let (_, parsed) = Self::parse(&buffer).map_err(|err| err.to_owned())?;

        Ok(parsed)
    }
}
