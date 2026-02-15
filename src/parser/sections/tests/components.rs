// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD COMPONENTS section.
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

use super::super::components::*;

use crate::parser::KeywordParam;
use crate::types::{Layer, Mirror, RectangleRef, TextPar, XYRef};

#[test]
fn test_example_components() {
    let params = vec![
        KeywordParam {
            keyword: "COMPONENT",
            parameter: "D102",
        },
        KeywordParam {
            keyword: "DEVICE",
            parameter: "1N4148",
        },
        KeywordParam {
            keyword: "PLACE",
            parameter: "1200 1800",
        },
        KeywordParam {
            keyword: "LAYER",
            parameter: "TOP",
        },
        KeywordParam {
            keyword: "ROTATION",
            parameter: "90",
        },
        KeywordParam {
            keyword: "SHAPE",
            parameter: "DO35_a MIRRORX 0",
        },
        KeywordParam {
            keyword: "ARTWORK",
            parameter: "ORIGIN_MARKER 0 0 0 MIRRORX 0",
        },
        KeywordParam {
            keyword: "TEXT",
            parameter: "50 -50 100 90 0 TOP D102 42 -50 500 200",
        },
        KeywordParam {
            keyword: "SHEET",
            parameter: "12_B3",
        },
        KeywordParam {
            keyword: "COMPONENT",
            parameter: "U7",
        },
        KeywordParam {
            keyword: "DEVICE",
            parameter: "74LS04",
        },
        KeywordParam {
            keyword: "PLACE",
            parameter: "0.003 9.52527",
        },
        KeywordParam {
            keyword: "LAYER",
            parameter: "BOTTOM",
        },
        KeywordParam {
            keyword: "ROTATION",
            parameter: "12.25",
        },
        KeywordParam {
            keyword: "SHAPE",
            parameter: "DIL14 0 FLIP",
        },
        KeywordParam {
            keyword: "ARTWORK",
            parameter: "PIN1_MARKER 6500 2400 0 MIRRORX FLIP",
        },
    ];

    let components = parse_components(&params).unwrap();

    assert_eq!(
        components,
        vec![
            Component {
                name: "D102".to_string(),
                device: "1N4148".to_string(),
                place: XYRef {
                    x: 1200.0,
                    y: 1800.0
                },
                layer: Layer::Top,
                rotation: 90.0,
                shape: Shape {
                    name: "DO35_a".to_string(),
                    mirror: Mirror::MirrorX,
                    flip: false
                },
                subcomponents: vec![SubComponent::Artwork(Artwork {
                    name: "ORIGIN_MARKER".to_string(),
                    xy: XYRef { x: 0.0, y: 0.0 },
                    rotation: 0.0,
                    mirror: Mirror::MirrorX,
                    flip: false,
                    attributes: vec![]
                })],
                texts: vec![Text {
                    origin: XYRef { x: 50.0, y: -50.0 },
                    text: TextPar {
                        text_size: 100.0,
                        rotation: 90.0,
                        mirror: Mirror::Not,
                        layer: Layer::Top,
                        text: "D102".to_string(),
                        area: RectangleRef {
                            origin: XYRef { x: 42.0, y: -50.0 },
                            x: 500.0,
                            y: 200.0
                        }
                    }
                }],
                sheet: Some("12_B3".to_string()),
                attributes: vec![]
            },
            Component {
                name: "U7".to_string(),
                device: "74LS04".to_string(),
                place: XYRef {
                    x: 0.003,
                    y: 9.52527
                },
                layer: Layer::Bottom,
                rotation: 12.25,
                shape: Shape {
                    name: "DIL14".to_string(),
                    mirror: Mirror::Not,
                    flip: true
                },
                subcomponents: vec![SubComponent::Artwork(Artwork {
                    name: "PIN1_MARKER".to_string(),
                    xy: XYRef {
                        x: 6500.0,
                        y: 2400.0
                    },
                    rotation: 0.0,
                    mirror: Mirror::MirrorX,
                    flip: true,
                    attributes: vec![]
                })],
                texts: vec![],
                sheet: None,
                attributes: vec![]
            }
        ]
    );
}

#[test]
fn test_component_with_improperly_formatted_strings() {
    // Seen in a real GenCAD file.

    let params = vec![
        KeywordParam {
            keyword: "COMPONENT",
            parameter: "LGA1718",
        },
        KeywordParam {
            keyword: "PLACE",
            parameter: "5073.52 8389.53",
        },
        KeywordParam {
            keyword: "LAYER",
            parameter: "TOP",
        },
        KeywordParam {
            keyword: "ROTATION",
            parameter: "0",
        },
        KeywordParam {
            keyword: "SHAPE",
            parameter: "LGA1718 0 0",
        },
        KeywordParam {
            keyword: "DEVICE",
            parameter: "Device LGA1718",
        },
    ];

    let components = parse_components(&params).unwrap();

    assert_eq!(
        components,
        vec![Component {
            name: "LGA1718".to_string(),
            device: "Device LGA1718".to_string(),
            place: XYRef {
                x: 5073.52,
                y: 8389.53
            },
            layer: Layer::Top,
            rotation: 0.0,
            shape: Shape {
                name: "LGA1718".to_string(),
                mirror: Mirror::Not,
                flip: false
            },
            subcomponents: vec![],
            texts: vec![],
            sheet: None,
            attributes: vec![]
        },]
    );
}
