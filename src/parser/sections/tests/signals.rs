// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD SIGNALS section.
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

use super::super::signals::*;

use crate::parser::KeywordParam;
use crate::types::{Layer, XYRef};

#[test]
fn test_example_signals() {
    let params = vec![
        KeywordParam {
            keyword: "SIGNAL",
            parameter: "data_bus_7",
        },
        KeywordParam {
            keyword: "NODE",
            parameter: "IC3 2",
        },
        KeywordParam {
            keyword: "NODE",
            parameter: "R2 2",
        },
        KeywordParam {
            keyword: "NAILLOC",
            parameter: "R2 2 -1 500 2500 -1 -1 100T BOTTOM",
        },
        KeywordParam {
            keyword: "NODE",
            parameter: "IC4 2",
        },
        KeywordParam {
            keyword: "NODE",
            parameter: "6Ic2 p34A",
        },
        KeywordParam {
            keyword: "NAILLOC",
            parameter: "6Ic2 p34A -1 800 3000 -1 -1 75T BOTTOM",
        },
        KeywordParam {
            keyword: "SIGNAL",
            parameter: "ADDRESS_BUS_4",
        },
        KeywordParam {
            keyword: "NODE",
            parameter: "U1 2",
        },
        KeywordParam {
            keyword: "NODE",
            parameter: "PL12 132",
        },
        KeywordParam {
            keyword: "NAILLOC",
            parameter: "PL12 132 -1 200 200 -1 -1 100T BOTTOM",
        },
    ];

    let signals = Signals::new(&params).unwrap();

    assert_eq!(
        signals,
        Signals {
            signals: vec![
                Signal {
                    name: "data_bus_7".to_string(),
                    nodes: vec![
                        Node {
                            component_name: "IC3".to_string(),
                            pin_name: "2".to_string()
                        },
                        Node {
                            component_name: "R2".to_string(),
                            pin_name: "2".to_string()
                        },
                        Node {
                            component_name: "IC4".to_string(),
                            pin_name: "2".to_string()
                        },
                        Node {
                            component_name: "6Ic2".to_string(),
                            pin_name: "p34A".to_string()
                        }
                    ],
                    nail_locations: vec![
                        NailLoc {
                            component_name: "R2".to_string(),
                            pin_name: "2".to_string(),
                            tp_name: "-1".to_string(),
                            xy: XYRef {
                                x: 500.0,
                                y: 2500.0
                            },
                            tan: "-1".to_string(),
                            tin: "-1".to_string(),
                            probe: "100T".to_string(),
                            layer: Layer::Bottom
                        },
                        NailLoc {
                            component_name: "6Ic2".to_string(),
                            pin_name: "p34A".to_string(),
                            tp_name: "-1".to_string(),
                            xy: XYRef {
                                x: 800.0,
                                y: 3000.0
                            },
                            tan: "-1".to_string(),
                            tin: "-1".to_string(),
                            probe: "75T".to_string(),
                            layer: Layer::Bottom
                        }
                    ]
                },
                Signal {
                    name: "ADDRESS_BUS_4".to_string(),
                    nodes: vec![
                        Node {
                            component_name: "U1".to_string(),
                            pin_name: "2".to_string()
                        },
                        Node {
                            component_name: "PL12".to_string(),
                            pin_name: "132".to_string()
                        }
                    ],
                    nail_locations: vec![NailLoc {
                        component_name: "PL12".to_string(),
                        pin_name: "132".to_string(),
                        tp_name: "-1".to_string(),
                        xy: XYRef { x: 200.0, y: 200.0 },
                        tan: "-1".to_string(),
                        tin: "-1".to_string(),
                        probe: "100T".to_string(),
                        layer: Layer::Bottom
                    }]
                }
            ],
            attributes: vec![]
        }
    );
}
