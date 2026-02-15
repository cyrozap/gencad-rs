// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD layer data type.
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

use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, value};
use nom::{IResult, Parser};

use crate::types::Layer;

impl Layer {
    fn from_pair(user: (&str, &str)) -> Result<Self, &'static str> {
        let (k, v) = user;
        let n: u16 = u16::from_str(v).map_err(|_| "Failed to parse u16")?;
        match k {
            "POWER" => Ok(Self::PowerX(n)),
            "GROUND" => Ok(Self::GroundX(n)),
            "INNER" => Ok(Self::InnerX(n)),
            "LAYER" => Ok(Self::LayerX(n)),
            "LAYERSET" => Ok(Self::LayersetX(n)),
            _ => panic!("This should never happen!"),
        }
    }
}

pub fn layer(s: &str) -> IResult<&str, Layer> {
    alt((
        map_res(
            alt((
                (tag("POWER"), digit1),
                (tag("GROUND"), digit1),
                (tag("INNER"), digit1),
                (tag("LAYER"), digit1),
                (tag("LAYERSET"), digit1),
            )),
            Layer::from_pair,
        ),
        alt((
            value(Layer::Top, tag("TOP")),
            value(Layer::Bottom, tag("BOTTOM")),
            value(Layer::SoldermaskTop, tag("SOLDERMASK_TOP")),
            value(Layer::SoldermaskBottom, tag("SOLDERMASK_BOTTOM")),
            value(Layer::SilkscreenTop, tag("SILKSCREEN_TOP")),
            value(Layer::SilkscreenBottom, tag("SILKSCREEN_BOTTOM")),
            value(Layer::SolderpasteTop, tag("SOLDERPASTE_TOP")),
            value(Layer::SolderpasteBottom, tag("SOLDERPASTE_BOTTOM")),
            value(Layer::Inner, tag("INNER")),
            value(Layer::All, tag("ALL")),
        )),
    ))
    .parse(s)
}
