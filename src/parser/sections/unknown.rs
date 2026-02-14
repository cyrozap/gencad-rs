// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for unknown GenCAD sections.
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

use crate::parser::KeywordParam;

/// A keyword/parameter pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    /// The keyword that determines how to interpret the parameter.
    pub keyword: String,
    /// The parameter associated with the keyword.
    pub parameter: String,
}

/// Represents an unknown section in a GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Unknown {
    /// The section name.
    pub name: String,
    /// The list of statements in this section.
    pub statements: Vec<Statement>,
}

impl Unknown {
    pub(crate) fn new(
        name: &str,
        params: &[KeywordParam],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let name = name.to_string();
        let statements = params
            .iter()
            .map(|kp| Statement {
                keyword: kp.keyword.to_string(),
                parameter: kp.parameter.to_string(),
            })
            .collect();
        Ok(Unknown { name, statements })
    }
}
