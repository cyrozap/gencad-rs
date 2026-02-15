// SPDX-License-Identifier: GPL-3.0-or-later

/*
 *  Parser for the GenCAD string data type.
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

use nom::branch::alt;
use nom::bytes::complete::{take_while, take_while1};
use nom::character::complete::char;
use nom::combinator::{map, not, peek, value};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuotedStringFragment<'a> {
    Literal(&'a str),
    Char(char),
}

fn is_valid_char(c: char) -> bool {
    matches!(c, ' '..='~')
}

fn is_valid_char_but_not_backslash_or_quote(c: char) -> bool {
    c != '\\' && c != '"' && is_valid_char(c)
}

fn is_valid_char_but_not_space(c: char) -> bool {
    c != ' ' && is_valid_char(c)
}

fn backslash_sequence(s: &str) -> IResult<&str, char> {
    alt((
        // A backslash before a quote mark becomes a quote mark
        preceded(char('\\'), value('"', char('"'))),
        // Backslash before any other character is not an escape sequence--it's just a literal backslash
        value('\\', char('\\')),
    ))
    .parse(s)
}

fn quoted_string_inner_fragment<'a>(s: &'a str) -> IResult<&'a str, QuotedStringFragment<'a>> {
    alt((
        map(backslash_sequence, QuotedStringFragment::Char),
        map(
            take_while1(is_valid_char_but_not_backslash_or_quote),
            QuotedStringFragment::Literal,
        ),
    ))
    .parse(s)
}

fn quoted_string(s: &str) -> IResult<&str, String> {
    let build_string = many0(quoted_string_inner_fragment).map(|fragments| {
        fragments
            .into_iter()
            .fold(String::new(), |mut string, fragment| {
                match fragment {
                    QuotedStringFragment::Literal(s) => string.push_str(s),
                    QuotedStringFragment::Char(c) => string.push(c),
                }
                string
            })
    });
    delimited(char('"'), build_string, char('"')).parse(s)
}

fn unquoted_string(s: &str) -> IResult<&str, &str> {
    preceded(
        peek(not(char('"'))),
        take_while(is_valid_char_but_not_space),
    )
    .parse(s)
}

pub fn string(s: &str) -> IResult<&str, String> {
    alt((map(unquoted_string, |x| x.to_string()), quoted_string)).parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backslash_sequence() {
        assert_eq!(backslash_sequence(r#"\;"#), Ok((";", '\\')));
        assert_eq!(backslash_sequence(r#"\\;"#), Ok((r#"\;"#, '\\')));
        assert_eq!(backslash_sequence(r#"\";"#), Ok((";", '"')));
    }

    #[test]
    fn test_quoted_string_inner_fragment() {
        assert_eq!(
            quoted_string_inner_fragment(r#"ABCD EFGH"#),
            Ok(("", QuotedStringFragment::Literal("ABCD EFGH")))
        );
        assert_eq!(
            quoted_string_inner_fragment(r#"ABCD EFGH\"#),
            Ok((r#"\"#, QuotedStringFragment::Literal("ABCD EFGH")))
        );
        assert_eq!(
            quoted_string_inner_fragment(r#"ABCD \EFGH"#),
            Ok((r#"\EFGH"#, QuotedStringFragment::Literal("ABCD ")))
        );
        assert_eq!(
            quoted_string_inner_fragment(r#"\;"#),
            Ok((";", QuotedStringFragment::Char('\\')))
        );
        assert_eq!(
            quoted_string_inner_fragment(r#"\\;"#),
            Ok((r#"\;"#, QuotedStringFragment::Char('\\')))
        );
        assert_eq!(
            quoted_string_inner_fragment(r#"\";"#),
            Ok((";", QuotedStringFragment::Char('"')))
        );
    }

    #[test]
    fn test_quoted_string() {
        assert_eq!(quoted_string(r#""""#), Ok(("", "".to_string())));
        assert_eq!(quoted_string(r#""A""#), Ok(("", "A".to_string())));
        assert_eq!(
            quoted_string(r#""ABCD EFGH""#),
            Ok(("", "ABCD EFGH".to_string()))
        );
        assert_eq!(quoted_string(r#""A\"""#), Ok(("", "A\"".to_string())));
        assert_eq!(
            quoted_string(r#""ABCD EFGH \IJKL\ \"MNOP\" QRST WXYZ""#),
            Ok(("", "ABCD EFGH \\IJKL\\ \"MNOP\" QRST WXYZ".to_string()))
        );
    }

    #[test]
    fn test_unquoted_string() {
        assert_eq!(unquoted_string(""), Ok(("", "")));
        assert_eq!(unquoted_string("A"), Ok(("", "A")));

        // Unquoted strings may contain quotes, so long as they don't start with one
        assert_eq!(unquoted_string(r#"A""#), Ok(("", r#"A""#)));

        // Check all valid characters
        const VALID_CHARS: &str = r##"!"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"##;
        assert_eq!(unquoted_string(VALID_CHARS), Ok(("", VALID_CHARS)));

        // Some random valid strings
        assert_eq!(unquoted_string("HYGi_"), Ok(("", "HYGi_")));
        assert_eq!(unquoted_string("h&DB%Q1#u"), Ok(("", "h&DB%Q1#u")));
        assert_eq!(unquoted_string("RTc&n"), Ok(("", "RTc&n")));
        assert_eq!(unquoted_string("=B4=~._"), Ok(("", "=B4=~._")));
        assert_eq!(unquoted_string("u0dL$2|"), Ok(("", "u0dL$2|")));

        // Unquoted strings must not contain spaces
        assert_eq!(unquoted_string("o7a[xtE "), Ok((" ", "o7a[xtE")));
        assert_eq!(unquoted_string(")(WA Gnw"), Ok((" Gnw", ")(WA")));

        // Unquoted strings must not start with quote
        assert!(unquoted_string("\"qoY@M;").is_err());
        assert!(unquoted_string(r#""A""#).is_err());

        // No non-ASCII characters
        assert_eq!(unquoted_string("V3\"'😀"), Ok(("😀", "V3\"'")));
        assert_eq!(unquoted_string("A'😀A4%"), Ok(("😀A4%", "A'")));
    }
}
