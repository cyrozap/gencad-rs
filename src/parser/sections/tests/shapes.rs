// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD SHAPES section.
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

use super::super::shapes::*;

use crate::parser::KeywordParam;
use crate::types::{ArcRef, CircularArcRef, Layer, LineRef, Mirror, XYRef};

#[test]
fn test_example_shape() {
    let params = vec![
        KeywordParam {
            keyword: "SHAPE",
            parameter: "CAP_SUPPRESS_TYPE_____24",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "-1000 200 -1000 -200",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "-1000 -200 1000 -200",
        },
        KeywordParam {
            keyword: "ARC",
            parameter: "1000 -200 1000 200 1000 0",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "1000 200 -1000 200",
        },
        KeywordParam {
            keyword: "PIN",
            parameter: "1 p102_4 -100 100 TOP 315 0",
        },
        KeywordParam {
            keyword: "PIN",
            parameter: "1 s106_6 -100 100 BOTTOM 315 MIRRORX",
        },
        KeywordParam {
            keyword: "PIN",
            parameter: "2 p102_4 100 -100 TOP 135 0",
        },
        KeywordParam {
            keyword: "PIN",
            parameter: "2 s106_6 100 -100 BOTTOM 135 MIRRORX",
        },
        KeywordParam {
            keyword: "ARTWORK",
            parameter: "PIN1_MARKER 0 400 0 0",
        },
        KeywordParam {
            keyword: "FID",
            parameter: "PRIMARY OPTICAL1 0 0 TOP 0 0",
        },
    ];

    let shapes = parse_shapes(&params).unwrap();

    assert_eq!(
        shapes,
        vec![Shape {
            name: "CAP_SUPPRESS_TYPE_____24".to_string(),
            elements: vec![
                ShapeElement::Line(LineRef {
                    start: XYRef {
                        x: -1000.0,
                        y: 200.0
                    },
                    end: XYRef {
                        x: -1000.0,
                        y: -200.0
                    }
                }),
                ShapeElement::Line(LineRef {
                    start: XYRef {
                        x: -1000.0,
                        y: -200.0
                    },
                    end: XYRef {
                        x: 1000.0,
                        y: -200.0
                    }
                }),
                ShapeElement::Arc(ArcRef::Circular(CircularArcRef {
                    start: XYRef {
                        x: 1000.0,
                        y: -200.0
                    },
                    end: XYRef {
                        x: 1000.0,
                        y: 200.0
                    },
                    center: XYRef { x: 1000.0, y: 0.0 },
                })),
                ShapeElement::Line(LineRef {
                    start: XYRef {
                        x: 1000.0,
                        y: 200.0
                    },
                    end: XYRef {
                        x: -1000.0,
                        y: 200.0
                    }
                }),
            ],
            insert: None,
            height: None,
            subshapes: vec![
                SubShape::Pin(Pin {
                    name: "1".to_string(),
                    pad_name: "p102_4".to_string(),
                    xy: XYRef {
                        x: -100.0,
                        y: 100.0
                    },
                    layer: Layer::Top,
                    rotation: 315.0,
                    mirror: Mirror::Not,
                    attributes: vec![]
                }),
                SubShape::Pin(Pin {
                    name: "1".to_string(),
                    pad_name: "s106_6".to_string(),
                    xy: XYRef {
                        x: -100.0,
                        y: 100.0
                    },
                    layer: Layer::Bottom,
                    rotation: 315.0,
                    mirror: Mirror::MirrorX,
                    attributes: vec![]
                }),
                SubShape::Pin(Pin {
                    name: "2".to_string(),
                    pad_name: "p102_4".to_string(),
                    xy: XYRef {
                        x: 100.0,
                        y: -100.0
                    },
                    layer: Layer::Top,
                    rotation: 135.0,
                    mirror: Mirror::Not,
                    attributes: vec![]
                }),
                SubShape::Pin(Pin {
                    name: "2".to_string(),
                    pad_name: "s106_6".to_string(),
                    xy: XYRef {
                        x: 100.0,
                        y: -100.0
                    },
                    layer: Layer::Bottom,
                    rotation: 135.0,
                    mirror: Mirror::MirrorX,
                    attributes: vec![]
                }),
                SubShape::Artwork(Artwork {
                    name: "PIN1_MARKER".to_string(),
                    xy: XYRef { x: 0.0, y: 400.0 },
                    rotation: 0.0,
                    mirror: Mirror::Not,
                    attributes: vec![]
                }),
                SubShape::Fid(Fid {
                    name: "PRIMARY".to_string(),
                    pad_name: "OPTICAL1".to_string(),
                    xy: XYRef { x: 0.0, y: 0.0 },
                    layer: Layer::Top,
                    rotation: 0.0,
                    mirror: Mirror::Not,
                    attributes: vec![]
                })
            ],
            attributes: vec![]
        }]
    );
}
