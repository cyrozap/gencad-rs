// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD PADSTACKS section.
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

use super::super::padstacks::*;

use crate::parser::KeywordParam;
use crate::types::{Layer, Mirror};

#[test]
fn test_example_padstacks() {
    let params = vec![
        KeywordParam {
            keyword: "PADSTACK",
            parameter: "p_stack1 -1",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "p102_4 TOP 180 0",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "s102_4 BOTTOM 0 0",
        },
        KeywordParam {
            keyword: "PADSTACK",
            parameter: "p_stack2 -1",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "r_r3 TOP 180 MIRRORX",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "r_r0 INNER1 180 MIRRORX",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "r_r0 INNER2 180 MIRRORX",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "r_r3 BOTTOM 180 MIRRORY",
        },
    ];

    let padstacks = Padstacks::new(&params).unwrap();

    assert_eq!(
        padstacks,
        Padstacks {
            padstacks: vec![
                Padstack {
                    name: "p_stack1".to_string(),
                    drill_size: -1.0,
                    pads: vec![
                        Pad {
                            name: "p102_4".to_string(),
                            layer: Layer::Top,
                            rotation: 180.0,
                            mirror: Mirror::Not
                        },
                        Pad {
                            name: "s102_4".to_string(),
                            layer: Layer::Bottom,
                            rotation: 0.0,
                            mirror: Mirror::Not
                        }
                    ]
                },
                Padstack {
                    name: "p_stack2".to_string(),
                    drill_size: -1.0,
                    pads: vec![
                        Pad {
                            name: "r_r3".to_string(),
                            layer: Layer::Top,
                            rotation: 180.0,
                            mirror: Mirror::MirrorX
                        },
                        Pad {
                            name: "r_r0".to_string(),
                            layer: Layer::InnerX(1),
                            rotation: 180.0,
                            mirror: Mirror::MirrorX
                        },
                        Pad {
                            name: "r_r0".to_string(),
                            layer: Layer::InnerX(2),
                            rotation: 180.0,
                            mirror: Mirror::MirrorX
                        },
                        Pad {
                            name: "r_r3".to_string(),
                            layer: Layer::Bottom,
                            rotation: 180.0,
                            mirror: Mirror::MirrorY
                        }
                    ]
                }
            ],
            attributes: vec![]
        }
    );
}
