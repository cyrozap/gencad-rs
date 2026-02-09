// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Rust definition of the GenCAD layer data type.
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

use crate::serialization::ToGencadString;

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

impl ToGencadString for Layer {
    fn to_gencad_string(&self) -> String {
        match self {
            Self::Top => "TOP".to_string(),
            Self::Bottom => "BOTTOM".to_string(),
            Self::SoldermaskTop => "SOLDERMASK_TOP".to_string(),
            Self::SoldermaskBottom => "SOLDERMASK_BOTTOM".to_string(),
            Self::SilkscreenTop => "SILKSCREEN_TOP".to_string(),
            Self::SilkscreenBottom => "SILKSCREEN_BOTTOM".to_string(),
            Self::SolderpasteTop => "SOLDERPASTE_TOP".to_string(),
            Self::SolderpasteBottom => "SOLDERPASTE_BOTTOM".to_string(),
            Self::PowerX(n) => format!("POWER{}", n),
            Self::GroundX(n) => format!("GROUND{}", n),
            Self::Inner => "INNER".to_string(),
            Self::InnerX(n) => format!("INNER{}", n),
            Self::All => "ALL".to_string(),
            Self::LayerX(n) => format!("LAYER{}", n),
            Self::LayersetX(n) => format!("LAYERSET{}", n),
        }
    }
}
