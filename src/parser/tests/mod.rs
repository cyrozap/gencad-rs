// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Tests for the parser module.
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

use super::sections::board;
use super::sections::board::{ArtworkComponent, BoardShape, Cutout, Mask, Subsection};
use super::sections::components::{self, SubComponent};
use super::sections::devices::PinDesc;
use super::sections::pads;
use super::sections::pads::PadShape;
use super::sections::padstacks;
use super::sections::padstacks::Padstack;
use super::sections::shapes;
use super::sections::shapes::{Pin, ShapeElement, SubShape};
use super::sections::signals::{NailLoc, Node, Signal};
use super::sections::unknown::{Statement, Unknown};
use super::types::{
    ArcRef, Attribute, CircleRef, CircularArcRef, Dimension, Layer, LineRef, Mirror, PadType,
    RectangleRef, TextPar, XYRef,
};
use super::*;

const EXAMPLE: &[u8; 2610] = include_bytes!("fixtures/example.cad");

#[test]
fn test_example() {
    let parsed = ParsedGencadFile::new(EXAMPLE.as_slice()).unwrap();

    assert_eq!(
        parsed,
        ParsedGencadFile {
            sections: vec![
                ParsedSection::Header(Header {
                    gencad_version: 1.4,
                    user: "Mitron Europe Ltd. Serial Number 00001".to_string(),
                    drawing: "Modem C100 motherboard 1234-5678".to_string(),
                    revision: "Rev 566g 20th September 1990".to_string(),
                    units: Dimension::User(1200),
                    origin: XYRef { x: 0.0, y: 0.0 },
                    intertrack: 0.0,
                    attributes: vec![
                        Attribute {
                            category: "alpha".to_string(),
                            name: "m_part".to_string(),
                            data: "BIS 9600".to_string()
                        },
                        Attribute {
                            category: "alpha".to_string(),
                            name: "m_desc".to_string(),
                            data: "Issue 2".to_string()
                        }
                    ]
                }),
                ParsedSection::Board(Board {
                    thickness: None,
                    outline_shapes: vec![
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
                    ],
                    attributes: vec![],
                    subsections: vec![
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
                            attributes: vec![]
                        }),
                        Subsection::Artwork(board::Artwork {
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
                            attributes: vec![]
                        })
                    ]
                }),
                ParsedSection::Pads(vec![
                    pads::Pad {
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
                    pads::Pad {
                        name: "p1053".to_string(),
                        ptype: PadType::Round,
                        drill_size: 20.0,
                        shapes: vec![PadShape::Circle(CircleRef {
                            center: XYRef { x: 0.0, y: 0.0 },
                            radius: 30.0
                        })],
                        attributes: vec![]
                    },
                    pads::Pad {
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
                    pads::Pad {
                        name: "d_hole_50".to_string(),
                        ptype: PadType::Round,
                        drill_size: 50.0,
                        shapes: vec![PadShape::Circle(CircleRef {
                            center: XYRef { x: 0.0, y: 0.0 },
                            radius: 25.0
                        })],
                        attributes: vec![]
                    },
                    pads::Pad {
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
                ]),
                ParsedSection::Padstacks(Padstacks {
                    padstacks: vec![
                        Padstack {
                            name: "p_stack1".to_string(),
                            drill_size: -1.0,
                            pads: vec![
                                padstacks::Pad {
                                    name: "p102_4".to_string(),
                                    layer: Layer::Top,
                                    rotation: 180.0,
                                    mirror: Mirror::Not
                                },
                                padstacks::Pad {
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
                                padstacks::Pad {
                                    name: "r_r3".to_string(),
                                    layer: Layer::Top,
                                    rotation: 180.0,
                                    mirror: Mirror::MirrorX
                                },
                                padstacks::Pad {
                                    name: "r_r0".to_string(),
                                    layer: Layer::InnerX(1),
                                    rotation: 180.0,
                                    mirror: Mirror::MirrorX
                                },
                                padstacks::Pad {
                                    name: "r_r0".to_string(),
                                    layer: Layer::InnerX(2),
                                    rotation: 180.0,
                                    mirror: Mirror::MirrorX
                                },
                                padstacks::Pad {
                                    name: "r_r3".to_string(),
                                    layer: Layer::Bottom,
                                    rotation: 180.0,
                                    mirror: Mirror::MirrorY
                                }
                            ]
                        }
                    ],
                    attributes: vec![]
                }),
                ParsedSection::Shapes(vec![Shape {
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
                            center: XYRef { x: 1000.0, y: 0.0 }
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
                        })
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
                        SubShape::Artwork(shapes::Artwork {
                            name: "PIN1_MARKER".to_string(),
                            xy: XYRef { x: 0.0, y: 400.0 },
                            rotation: 0.0,
                            mirror: Mirror::Not,
                            attributes: vec![]
                        }),
                        SubShape::Fid(shapes::Fid {
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
                }]),
                ParsedSection::Components(vec![
                    Component {
                        name: "D102".to_string(),
                        device: "1N4148".to_string(),
                        place: XYRef {
                            x: 1200.0,
                            y: 1800.0
                        },
                        layer: Layer::Top,
                        rotation: 90.0,
                        shape: components::Shape {
                            name: "DO35_a".to_string(),
                            mirror: Mirror::MirrorX,
                            flip: false
                        },
                        subcomponents: vec![SubComponent::Artwork(components::Artwork {
                            name: "ORIGIN_MARKER".to_string(),
                            xy: XYRef { x: 0.0, y: 0.0 },
                            rotation: 0.0,
                            mirror: Mirror::MirrorX,
                            flip: false,
                            attributes: vec![]
                        })],
                        texts: vec![components::Text {
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
                        shape: components::Shape {
                            name: "DIL14".to_string(),
                            mirror: Mirror::Not,
                            flip: true
                        },
                        subcomponents: vec![SubComponent::Artwork(components::Artwork {
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
                ]),
                ParsedSection::Devices(vec![Device {
                    name: "89-1N4148".to_string(),
                    part: Some("1N4148".to_string()),
                    dtype: Some("DIODE".to_string()),
                    style: None,
                    package: None,
                    pin_descriptions: vec![
                        PinDesc {
                            pin_name: "1".to_string(),
                            text: "anode".to_string()
                        },
                        PinDesc {
                            pin_name: "2".to_string(),
                            text: "cathode".to_string()
                        }
                    ],
                    pin_functions: vec![],
                    pincount: None,
                    value: None,
                    tol: None,
                    ntol: None,
                    ptol: None,
                    volts: None,
                    desc: Some("Diode 1N4148 bandoleer reverse voltage 100V".to_string()),
                    attributes: vec![]
                }]),
                ParsedSection::Signals(Signals {
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
                }),
                ParsedSection::Unknown(Unknown {
                    name: "UNKNOWN".to_string(),
                    statements: vec![
                        Statement {
                            keyword: "KEYWORDA".to_string(),
                            parameter: "parameter1 parameter2 parameter3".to_string()
                        },
                        Statement {
                            keyword: "KEYWORDB".to_string(),
                            parameter: "parameter1 parameter2 parameter3".to_string()
                        },
                    ]
                }),
            ]
        }
    );
}
