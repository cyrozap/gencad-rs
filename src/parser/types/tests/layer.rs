// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD layer data type.
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

use super::super::*;

use crate::types::Layer;

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
