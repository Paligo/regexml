// XSD 1.1 Part 2 §G.4.2.4: "If a string "IsX" matches the non-terminal IsBlock
// but X is not a recognized block name, then the expressions "\p{IsX}" and
// "\P{IsX}" each denote the set of all characters." The `;k` flag asks for
// that; by default an unknown block name is an error, as F&O 3.1 §5.6.1.5
// requires for XPath.

use regexml::{Error, Regex};

// single characters, so that a match anywhere is a match of the whole string
const SAMPLES: &[&str] = &["a", "Z", "0", " ", "\n", "-", "é", "€", "中", "\u{10FFFF}"];

fn unknown_block_error() -> Error {
    Error::Syntax("Unknown Unicode block: UnknownBlock".to_string())
}

// the expression compiled in both dialects
fn both(re: &str, flags: &str) -> [Regex; 2] {
    [
        Regex::xsd(re, flags).unwrap(),
        Regex::xpath(re, flags).unwrap(),
    ]
}

#[test]
fn test_unknown_block_is_an_error_by_default() {
    for re in [
        r"\p{IsUnknownBlock}",
        r"\P{IsUnknownBlock}",
        r"[\p{IsUnknownBlock}]",
        r"[a-z-[\P{IsUnknownBlock}]]",
    ] {
        for flags in ["", "i", ";K", ";kK"] {
            assert_eq!(Regex::xsd(re, flags).unwrap_err(), unknown_block_error());
            assert_eq!(Regex::xpath(re, flags).unwrap_err(), unknown_block_error());
        }
    }
}

#[test]
fn test_unknown_block_matches_every_character() {
    for flags in [";k", "i;k", "x;k", ";Kk"] {
        for regex in both(r"\p{IsUnknownBlock}", flags) {
            for s in SAMPLES {
                assert!(regex.is_match(s), "{:?} {}", s, flags);
            }
            assert!(!regex.is_match(""));
        }
    }
    let regex = Regex::xpath(r"^(?:\p{IsUnknownBlock}+)$", ";k").unwrap();
    assert!(regex.is_match("any thing at all\n中"));
    assert!(!regex.is_match(""));
}

#[test]
fn test_complement_of_unknown_block_matches_every_character_too() {
    for regex in both(r"\P{IsUnknownBlock}", ";k") {
        for s in SAMPLES {
            assert!(regex.is_match(s), "{:?}", s);
        }
        assert!(!regex.is_match(""));
    }
}

#[test]
fn test_unknown_block_in_character_classes() {
    for re in [
        r"[\p{IsUnknownBlock}]",
        r"[\P{IsUnknownBlock}]",
        r"[a\p{IsUnknownBlock}]",
        r"[\p{IsUnknownBlock}\p{IsBasicLatin}]",
    ] {
        for regex in both(re, ";k") {
            for s in SAMPLES {
                assert!(regex.is_match(s), "{} {:?}", re, s);
            }
        }
    }

    // the negation of a class is the complement of the set it denotes
    for regex in both(r"[^\p{IsUnknownBlock}]", ";k") {
        for s in SAMPLES {
            assert!(!regex.is_match(s), "{:?}", s);
        }
    }

    // subtracting it leaves nothing, subtracting from it leaves the rest
    for re in [r"[a-z-[\p{IsUnknownBlock}]]", r"[a-z-[\P{IsUnknownBlock}]]"] {
        for regex in both(re, ";k") {
            for s in SAMPLES {
                assert!(!regex.is_match(s), "{} {:?}", re, s);
            }
        }
    }
    for regex in both(r"[\p{IsUnknownBlock}-[a]]", ";k") {
        assert!(!regex.is_match("a"));
        assert!(regex.is_match("b"));
        assert!(regex.is_match("€"));
    }
}

#[test]
fn test_known_blocks_are_unchanged() {
    for regex in both(r"\p{IsBasicLatin}", ";k") {
        assert!(regex.is_match("a"));
        assert!(!regex.is_match("é"));
    }
    for regex in both(r"\P{IsBasicLatin}", ";k") {
        assert!(regex.is_match("é"));
        assert!(!regex.is_match("a"));
    }
}

#[test]
fn test_name_that_is_not_a_block_name_is_still_an_error() {
    // IsBlock ::= 'Is' [a-zA-Z0-9#x2D]+
    for re in [
        r"\p{IsUnknown_Block}",
        r"\p{IsUnknown Block}",
        r"\P{IsUnknown.Block}",
    ] {
        assert!(
            matches!(Regex::xsd(re, ";k"), Err(Error::Syntax(_))),
            "{}",
            re
        );
        assert!(
            matches!(Regex::xpath(re, ";k"), Err(Error::Syntax(_))),
            "{}",
            re
        );
    }
    // a hyphen is allowed
    assert!(Regex::xsd(r"\p{IsUnknown-Block}", ";k")
        .unwrap()
        .is_match("a"));
}
