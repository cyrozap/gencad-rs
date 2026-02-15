// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD text_par data type.
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

use crate::types::{Layer, Mirror, RectangleRef, TextPar, XYRef};

#[test]
fn test_ok() {
    assert_eq!(
        text_par("1 2 0 SILKSCREEN_TOP \"Test String\" -10 -20 20 40"),
        Ok((
            "",
            TextPar {
                text_size: 1.0,
                rotation: 2.0,
                mirror: Mirror::Not,
                layer: Layer::SilkscreenTop,
                text: "Test String".to_string(),
                area: RectangleRef {
                    origin: XYRef { x: -10.0, y: -20.0 },
                    x: 20.0,
                    y: 40.0
                },
            }
        ))
    );
}
