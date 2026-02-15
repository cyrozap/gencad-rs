// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD PADS section.
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

use super::super::pads::*;

use crate::parser::KeywordParam;
use crate::types::{ArcRef, CircleRef, CircularArcRef, LineRef, PadType, RectangleRef, XYRef};

#[test]
fn test_example_pads() {
    let params = vec![
        KeywordParam {
            keyword: "PAD",
            parameter: "p0101 FINGER 32",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "100 50 -100 50",
        },
        KeywordParam {
            keyword: "ARC",
            parameter: "-100 50 -100 -50 -100 0",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "-100 -50 100 -50",
        },
        KeywordParam {
            keyword: "ARC",
            parameter: "100 -50 100 50 100 0",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "p1053 ROUND 20",
        },
        KeywordParam {
            keyword: "CIRCLE",
            parameter: "0 0 30",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "p2034 BULLET 32",
        },
        KeywordParam {
            keyword: "ARC",
            parameter: "0 -50 0 50 0 0",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "0 50 -100 50",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "-100 50 -100 -50",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "-100 -50 0 -50",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "d_hole_50 ROUND 50",
        },
        KeywordParam {
            keyword: "CIRCLE",
            parameter: "0 0 25",
        },
        KeywordParam {
            keyword: "PAD",
            parameter: "3 RECTANGULAR 0",
        },
        KeywordParam {
            keyword: "RECTANGLE",
            parameter: "-5.2 -5.2 10.4 10.4",
        },
    ];

    let pads = parse_pads(&params).unwrap();

    assert_eq!(
        pads,
        vec![
            Pad {
                name: "p0101".to_string(),
                ptype: PadType::Finger,
                drill_size: 32.0,
                shapes: vec![
                    PadShape::Line(LineRef {
                        start: XYRef { x: 100.0, y: 50.0 },
                        end: XYRef { x: -100.0, y: 50.0 }
                    }),
                    PadShape::Arc(ArcRef::Circular(CircularArcRef {
                        start: XYRef { x: -100.0, y: 50.0 },
                        end: XYRef {
                            x: -100.0,
                            y: -50.0
                        },
                        center: XYRef { x: -100.0, y: 0.0 }
                    })),
                    PadShape::Line(LineRef {
                        start: XYRef {
                            x: -100.0,
                            y: -50.0
                        },
                        end: XYRef { x: 100.0, y: -50.0 }
                    }),
                    PadShape::Arc(ArcRef::Circular(CircularArcRef {
                        start: XYRef { x: 100.0, y: -50.0 },
                        end: XYRef { x: 100.0, y: 50.0 },
                        center: XYRef { x: 100.0, y: 0.0 }
                    }))
                ],
                attributes: vec![]
            },
            Pad {
                name: "p1053".to_string(),
                ptype: PadType::Round,
                drill_size: 20.0,
                shapes: vec![PadShape::Circle(CircleRef {
                    center: XYRef { x: 0.0, y: 0.0 },
                    radius: 30.0
                })],
                attributes: vec![]
            },
            Pad {
                name: "p2034".to_string(),
                ptype: PadType::Bullet,
                drill_size: 32.0,
                shapes: vec![
                    PadShape::Arc(ArcRef::Circular(CircularArcRef {
                        start: XYRef { x: 0.0, y: -50.0 },
                        end: XYRef { x: 0.0, y: 50.0 },
                        center: XYRef { x: 0.0, y: 0.0 }
                    })),
                    PadShape::Line(LineRef {
                        start: XYRef { x: 0.0, y: 50.0 },
                        end: XYRef { x: -100.0, y: 50.0 }
                    }),
                    PadShape::Line(LineRef {
                        start: XYRef { x: -100.0, y: 50.0 },
                        end: XYRef {
                            x: -100.0,
                            y: -50.0
                        }
                    }),
                    PadShape::Line(LineRef {
                        start: XYRef {
                            x: -100.0,
                            y: -50.0
                        },
                        end: XYRef { x: 0.0, y: -50.0 }
                    })
                ],
                attributes: vec![]
            },
            Pad {
                name: "d_hole_50".to_string(),
                ptype: PadType::Round,
                drill_size: 50.0,
                shapes: vec![PadShape::Circle(CircleRef {
                    center: XYRef { x: 0.0, y: 0.0 },
                    radius: 25.0
                })],
                attributes: vec![]
            },
            Pad {
                name: "3".to_string(),
                ptype: PadType::Rectangular,
                drill_size: 0.0,
                shapes: vec![PadShape::Rectangle(RectangleRef {
                    origin: XYRef { x: -5.2, y: -5.2 },
                    x: 10.4,
                    y: 10.4
                })],
                attributes: vec![]
            }
        ]
    );
}
