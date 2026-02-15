// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD BOARD section.
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

use super::super::board::*;

use crate::parser::KeywordParam;
use crate::types::{ArcRef, Attribute, CircleRef, CircularArcRef, Layer, LineRef, XYRef};

#[test]
fn test_example_board() {
    let params = vec![
        KeywordParam {
            keyword: "LINE",
            parameter: "1000 2000 1200 2000",
        },
        KeywordParam {
            keyword: "ARC",
            parameter: "1200 2000 1200 3000 1180 2500",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "1200 3000 1000 3000",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "1000 3000 1000 2000",
        },
        KeywordParam {
            keyword: "CUTOUT",
            parameter: "TRANSFORMER_HOLE",
        },
        KeywordParam {
            keyword: "CIRCLE",
            parameter: "1180 2500 20",
        },
        KeywordParam {
            keyword: "ATTRIBUTE",
            parameter: "board mill \"tool 255\"",
        },
        KeywordParam {
            keyword: "MASK",
            parameter: "Fixture_1 TOP",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "1005 2005 1195 2005",
        },
        KeywordParam {
            keyword: "ARC",
            parameter: "1195 2005 1195 2995 1195 2500",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "1195 2995 1005 2995",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "1005 2995 1005 2005",
        },
        KeywordParam {
            keyword: "ARTWORK",
            parameter: "ORIGIN_MARKER TOP",
        },
        KeywordParam {
            keyword: "TRACK",
            parameter: "10",
        },
        KeywordParam {
            keyword: "FILLED",
            parameter: "YES",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "-100 0 100 0",
        },
        KeywordParam {
            keyword: "LINE",
            parameter: "0 -100 0 100",
        },
    ];

    let board = Board::new(&params).unwrap();

    assert_eq!(board.thickness, None);
    assert_eq!(
        board.outline_shapes,
        vec![
            BoardShape::Line(LineRef {
                start: XYRef {
                    x: 1000.0,
                    y: 2000.0
                },
                end: XYRef {
                    x: 1200.0,
                    y: 2000.0
                }
            }),
            BoardShape::Arc(ArcRef::Circular(CircularArcRef {
                start: XYRef {
                    x: 1200.0,
                    y: 2000.0
                },
                end: XYRef {
                    x: 1200.0,
                    y: 3000.0
                },
                center: XYRef {
                    x: 1180.0,
                    y: 2500.0
                }
            })),
            BoardShape::Line(LineRef {
                start: XYRef {
                    x: 1200.0,
                    y: 3000.0
                },
                end: XYRef {
                    x: 1000.0,
                    y: 3000.0
                }
            }),
            BoardShape::Line(LineRef {
                start: XYRef {
                    x: 1000.0,
                    y: 3000.0
                },
                end: XYRef {
                    x: 1000.0,
                    y: 2000.0
                }
            })
        ]
    );
    assert_eq!(board.attributes, Vec::new());
    assert_eq!(
        board.subsections,
        vec![
            Subsection::Cutout(Cutout {
                name: "TRANSFORMER_HOLE".to_string(),
                shapes: vec![BoardShape::Circle(CircleRef {
                    center: XYRef {
                        x: 1180.0,
                        y: 2500.0
                    },
                    radius: 20.0
                })],
                attributes: vec![Attribute {
                    category: "board".to_string(),
                    name: "mill".to_string(),
                    data: "tool 255".to_string()
                }]
            }),
            Subsection::Mask(Mask {
                name: "Fixture_1".to_string(),
                layer: Layer::Top,
                shapes: vec![
                    BoardShape::Line(LineRef {
                        start: XYRef {
                            x: 1005.0,
                            y: 2005.0
                        },
                        end: XYRef {
                            x: 1195.0,
                            y: 2005.0
                        }
                    }),
                    BoardShape::Arc(ArcRef::Circular(CircularArcRef {
                        start: XYRef {
                            x: 1195.0,
                            y: 2005.0
                        },
                        end: XYRef {
                            x: 1195.0,
                            y: 2995.0
                        },
                        center: XYRef {
                            x: 1195.0,
                            y: 2500.0
                        }
                    })),
                    BoardShape::Line(LineRef {
                        start: XYRef {
                            x: 1195.0,
                            y: 2995.0
                        },
                        end: XYRef {
                            x: 1005.0,
                            y: 2995.0
                        }
                    }),
                    BoardShape::Line(LineRef {
                        start: XYRef {
                            x: 1005.0,
                            y: 2995.0
                        },
                        end: XYRef {
                            x: 1005.0,
                            y: 2005.0
                        }
                    })
                ],
                attributes: Vec::new(),
            }),
            Subsection::Artwork(Artwork {
                name: "ORIGIN_MARKER".to_string(),
                layer: Layer::Top,
                components: vec![
                    ArtworkComponent::Track("10".to_string()),
                    ArtworkComponent::Filled(true),
                    ArtworkComponent::Line(LineRef {
                        start: XYRef { x: -100.0, y: 0.0 },
                        end: XYRef { x: 100.0, y: 0.0 }
                    }),
                    ArtworkComponent::Line(LineRef {
                        start: XYRef { x: 0.0, y: -100.0 },
                        end: XYRef { x: 0.0, y: 100.0 }
                    })
                ],
                attributes: Vec::new(),
            })
        ]
    );
}
