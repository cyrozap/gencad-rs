// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser tests for the GenCAD DEVICES section.
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

use super::super::devices::*;

use crate::parser::KeywordParam;

#[test]
fn test_example_devices() {
    let params = vec![
        KeywordParam {
            keyword: "DEVICE",
            parameter: "89-1N4148",
        },
        KeywordParam {
            keyword: "PART",
            parameter: "1N4148",
        },
        KeywordParam {
            keyword: "TYPE",
            parameter: "DIODE",
        },
        KeywordParam {
            keyword: "PINDESC",
            parameter: "1 anode",
        },
        KeywordParam {
            keyword: "PINDESC",
            parameter: "2 cathode",
        },
        KeywordParam {
            keyword: "DESC",
            parameter: "\"Diode 1N4148 bandoleer reverse voltage 100V\"",
        },
    ];

    let devices = parse_devices(&params).unwrap();

    assert_eq!(
        devices,
        vec![Device {
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
        }]
    );
}

#[test]
fn test_device_with_improperly_formatted_strings() {
    // Seen in a real GenCAD file.

    let params = vec![
        KeywordParam {
            keyword: "DEVICE",
            parameter: "Device PANEL",
        },
        KeywordParam {
            keyword: "PART",
            parameter: "HEADER 2X10P G/F 2.54 BLK/C//LONG SHOUNG/1102014270",
        },
    ];

    let devices = parse_devices(&params).unwrap();

    assert_eq!(
        devices,
        vec![Device {
            name: "Device PANEL".to_string(),
            part: Some("HEADER 2X10P G/F 2.54 BLK/C//LONG SHOUNG/1102014270".to_string()),
            dtype: None,
            style: None,
            package: None,
            pin_descriptions: vec![],
            pin_functions: vec![],
            pincount: None,
            value: None,
            tol: None,
            ntol: None,
            ptol: None,
            volts: None,
            desc: None,
            attributes: vec![]
        }]
    );
}
