// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  src/sections/board.rs - Parser for the GenCAD BOARD section.
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

use nom::Parser;
use nom::sequence::preceded;

use crate::parser::KeywordParam;
use crate::types::TextPar;
use crate::types::{
    ArcRef, Attribute, CircleRef, Layer, LineRef, Number, RectangleRef, XYRef, arc_ref, attrib_ref,
    circle_ref, filled_ref, layer, line_ref, number, rectangle_ref, string, text_par, track_name,
    util::spaces, x_y_ref,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardShape {
    Line(LineRef),
    Arc(ArcRef),
    Circle(CircleRef),
    Rectangle(RectangleRef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cutout {
    pub name: String,
    pub shapes: Vec<BoardShape>,
    pub attributes: Vec<Attribute>,
}

impl Cutout {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let shapes = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            shapes,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mask {
    pub name: String,
    pub layer: Layer,
    pub shapes: Vec<BoardShape>,
    pub attributes: Vec<Attribute>,
}

impl Mask {
    fn new(name: &str, layer: Layer) -> Self {
        let name = name.to_string();
        let shapes = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            layer,
            shapes,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub origin: XYRef,
    pub text: TextPar,
}

impl Text {
    fn new(origin: XYRef, text: TextPar) -> Self {
        Self { origin, text }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArtworkComponent {
    Line(LineRef),
    Arc(ArcRef),
    Circle(CircleRef),
    Rectangle(RectangleRef),
    Track(String),
    Filled(bool),
    Text(Text),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Artwork {
    pub name: String,
    pub layer: Layer,
    pub components: Vec<ArtworkComponent>,
    pub attributes: Vec<Attribute>,
}

impl Artwork {
    fn new(name: &str, layer: Layer) -> Self {
        let name = name.to_string();
        let components = Vec::new();
        let attributes = Vec::new();
        Self {
            name,
            layer,
            components,
            attributes,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Subsection {
    Cutout(Cutout),
    Mask(Mask),
    Artwork(Artwork),
}

#[derive(Debug, Clone, PartialEq)]
enum BoardParserState {
    Board,
    Subsection(Subsection),
}

/// Represents the `BOARD` section of a GenCAD file.
#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    /// The board thickness, if specified.
    pub thickness: Option<Number>,
    /// The shapes that make up the board outline.
    pub outline_shapes: Vec<BoardShape>,
    /// A list of attributes associated with the board.
    pub attributes: Vec<Attribute>,
    /// A list of subsections in the board.
    pub subsections: Vec<Subsection>,
}

impl Board {
    pub fn new(params: &[KeywordParam]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut thickness = None;
        let mut outline_shapes = Vec::new();
        let mut attributes = Vec::new();
        let mut subsections = Vec::new();

        let mut parser_state = BoardParserState::Board;

        for param in params {
            match param.keyword.as_str() {
                "THICKNESS" => {
                    if parser_state == BoardParserState::Board && thickness.is_none() {
                        let (_, value) =
                            number(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                        thickness = Some(value);
                    }
                }
                "LINE" => {
                    let (_, line) =
                        line_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Line(line))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Line(line))
                            }
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Line(line))
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Line(line));
                    }
                }
                "ARC" => {
                    let (_, arc) =
                        arc_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Arc(arc))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Arc(arc))
                            }
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Arc(arc))
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Arc(arc));
                    }
                }
                "CIRCLE" => {
                    let (_, circle) =
                        circle_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Circle(circle))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Circle(circle))
                            }
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Circle(circle))
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Circle(circle));
                    }
                }
                "RECTANGLE" => {
                    let (_, rectangle) =
                        rectangle_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => {
                                cutout.shapes.push(BoardShape::Rectangle(rectangle))
                            }
                            Subsection::Mask(ref mut mask) => {
                                mask.shapes.push(BoardShape::Rectangle(rectangle))
                            }
                            Subsection::Artwork(ref mut artwork) => artwork
                                .components
                                .push(ArtworkComponent::Rectangle(rectangle)),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        outline_shapes.push(BoardShape::Rectangle(rectangle));
                    }
                }
                "ATTRIBUTE" => {
                    let (_, attribute) =
                        attrib_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Cutout(ref mut cutout) => cutout.attributes.push(attribute),
                            Subsection::Mask(ref mut mask) => mask.attributes.push(attribute),
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.attributes.push(attribute)
                            }
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        attributes.push(attribute);
                    }
                }

                // Artwork-only keywords
                "TRACK" => {
                    let (_, track) =
                        track_name(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Track(track))
                            }
                            _ => return Err("TRACK is only supported in ARTWORK section!".into()),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        return Err("TRACK is only supported in ARTWORK section!".into());
                    }
                }
                "FILLED" => {
                    let (_, filled) =
                        filled_ref(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Artwork(ref mut artwork) => {
                                artwork.components.push(ArtworkComponent::Filled(filled))
                            }
                            _ => return Err("FILLED is only supported in ARTWORK section!".into()),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        return Err("FILLED is only supported in ARTWORK section!".into());
                    }
                }
                "TEXT" => {
                    let (_, (origin, text)) = (x_y_ref, preceded(spaces, text_par))
                        .parse(param.parameter.as_str())
                        .map_err(|err| err.to_owned())?;
                    if let BoardParserState::Subsection(mut subsection) = parser_state {
                        match subsection {
                            Subsection::Artwork(ref mut artwork) => artwork
                                .components
                                .push(ArtworkComponent::Text(Text::new(origin, text))),
                            _ => return Err("TEXT is only supported in ARTWORK section!".into()),
                        }
                        parser_state = BoardParserState::Subsection(subsection);
                    } else {
                        return Err("TEXT is only supported in ARTWORK section!".into());
                    }
                }

                // Subsections
                "CUTOUT" => {
                    if let BoardParserState::Subsection(subsection) = parser_state {
                        subsections.push(subsection);
                    }
                    let (_, cutout_name) =
                        string(param.parameter.as_str()).map_err(|err| err.to_owned())?;
                    parser_state = BoardParserState::Subsection(Subsection::Cutout(Cutout::new(
                        cutout_name.as_str(),
                    )))
                }
                "MASK" => {
                    if let BoardParserState::Subsection(subsection) = parser_state {
                        subsections.push(subsection);
                    }
                    let (_, (mask_name, mask_layer)) = (string, preceded(spaces, layer))
                        .parse(param.parameter.as_str())
                        .map_err(|err| err.to_owned())?;
                    parser_state = BoardParserState::Subsection(Subsection::Mask(Mask::new(
                        mask_name.as_str(),
                        mask_layer,
                    )))
                }
                "ARTWORK" => {
                    if let BoardParserState::Subsection(subsection) = parser_state {
                        subsections.push(subsection);
                    }
                    let (_, (mask_name, mask_layer)) = (string, preceded(spaces, layer))
                        .parse(param.parameter.as_str())
                        .map_err(|err| err.to_owned())?;
                    parser_state = BoardParserState::Subsection(Subsection::Artwork(Artwork::new(
                        mask_name.as_str(),
                        mask_layer,
                    )))
                }
                _ => {}
            }
        }

        if let BoardParserState::Subsection(subsection) = parser_state {
            subsections.push(subsection);
        }

        Ok(Self {
            thickness,
            outline_shapes,
            attributes,
            subsections,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CircularArcRef;

    #[test]
    fn test_example_board() {
        let params = vec![
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "1000 2000 1200 2000".to_string(),
            },
            KeywordParam {
                keyword: "ARC".to_string(),
                parameter: "1200 2000 1200 3000 1180 2500".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "1200 3000 1000 3000".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "1000 3000 1000 2000".to_string(),
            },
            KeywordParam {
                keyword: "CUTOUT".to_string(),
                parameter: "TRANSFORMER_HOLE".to_string(),
            },
            KeywordParam {
                keyword: "CIRCLE".to_string(),
                parameter: "1180 2500 20".to_string(),
            },
            KeywordParam {
                keyword: "ATTRIBUTE".to_string(),
                parameter: "board mill \"tool 255\"".to_string(),
            },
            KeywordParam {
                keyword: "MASK".to_string(),
                parameter: "Fixture_1 TOP".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "1005 2005 1195 2005".to_string(),
            },
            KeywordParam {
                keyword: "ARC".to_string(),
                parameter: "1195 2005 1195 2995 1195 2500".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "1195 2995 1005 2995".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "1005 2995 1005 2005".to_string(),
            },
            KeywordParam {
                keyword: "ARTWORK".to_string(),
                parameter: "ORIGIN_MARKER TOP".to_string(),
            },
            KeywordParam {
                keyword: "TRACK".to_string(),
                parameter: "10".to_string(),
            },
            KeywordParam {
                keyword: "FILLED".to_string(),
                parameter: "YES".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "-100 0 100 0".to_string(),
            },
            KeywordParam {
                keyword: "LINE".to_string(),
                parameter: "0 -100 0 100".to_string(),
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
}
