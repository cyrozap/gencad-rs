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

/// Layer information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// The top of the board.
    Top,
    /// The bottom of the board.
    Bottom,
    /// The soldermask on the top side of the board.
    SoldermaskTop,
    /// The soldermask on the bottom side of the board.
    SoldermaskBottom,
    /// The silkscreen on the top side of the board.
    SilkscreenTop,
    /// The silkscreen on the bottom side of the board.
    SilkscreenBottom,
    /// The solder paste on the top side of the board.
    SolderpasteTop,
    /// The solder paste on the bottom side of the board.
    SolderpasteBottom,
    /// Specific power layer/plane.
    PowerX(u16),
    /// Specific ground layer/plane.
    GroundX(u16),
    /// All inner layers combined.
    Inner,
    /// A specific inner layer.
    InnerX(u16),
    /// All copper layers of the board.
    All,
    /// Specific layers that cannot be defined by the other parameters.
    LayerX(u16),
    /// Sets of layers.
    LayersetX(u16),
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_standard() {
        assert_eq!(layer("TOP"), Ok(("", Layer::Top)));
        assert_eq!(layer("BOTTOM"), Ok(("", Layer::Bottom)));
        assert_eq!(layer("SOLDERMASK_TOP"), Ok(("", Layer::SoldermaskTop)));
        assert_eq!(
            layer("SOLDERMASK_BOTTOM"),
            Ok(("", Layer::SoldermaskBottom))
        );
        assert_eq!(layer("SILKSCREEN_TOP"), Ok(("", Layer::SilkscreenTop)));
        assert_eq!(
            layer("SILKSCREEN_BOTTOM"),
            Ok(("", Layer::SilkscreenBottom))
        );
        assert_eq!(layer("SOLDERPASTE_TOP"), Ok(("", Layer::SolderpasteTop)));
        assert_eq!(
            layer("SOLDERPASTE_BOTTOM"),
            Ok(("", Layer::SolderpasteBottom))
        );
        assert_eq!(layer("INNER"), Ok(("", Layer::Inner)));
        assert_eq!(layer("ALL"), Ok(("", Layer::All)));

        assert_eq!(layer("POWER1"), Ok(("", Layer::PowerX(1))));
        assert_eq!(layer("GROUND1"), Ok(("", Layer::GroundX(1))));
        assert_eq!(layer("INNER1"), Ok(("", Layer::InnerX(1))));
        assert_eq!(layer("LAYER1"), Ok(("", Layer::LayerX(1))));
        assert_eq!(layer("LAYERSET1"), Ok(("", Layer::LayersetX(1))));
    }
}
