use regexml::{AnalyzeEntry, Error, MatchEntry, Regex};

// analyze-string with empty string
#[test]
fn test_analyze_string_001() {
    let regex = Regex::xpath("abc", "").unwrap();
    let result = regex.analyze("").unwrap().collect::<Vec<_>>();
    assert_eq!(result, vec![])
}

// analyze-string with a mix of matching and non-matching strings
#[test]
fn test_analyze_string_003() {
    let regex = Regex::xpath("a", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("a".to_string())]),
            AnalyzeEntry::NonMatch("n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("a".to_string())]),
            AnalyzeEntry::NonMatch("n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("a".to_string())]),
        ]
    )
}

// analyze-string with a single non-matching string
#[test]
fn test_analyze_string_004() {
    let regex = Regex::xpath("custard", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(result, vec![AnalyzeEntry::NonMatch("banana".to_string())])
}

// analyze-string with a single matching string
#[test]
fn test_analyze_string_005() {
    let regex = Regex::xpath(".+", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![AnalyzeEntry::Match(vec![MatchEntry::String(
            "banana".to_string()
        )])]
    )
}

// analyze-string with a adjacent matching strings
#[test]
fn test_analyze_string_006() {
    let regex = Regex::xpath("an", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("an".to_string())]),
            AnalyzeEntry::Match(vec![MatchEntry::String("an".to_string())]),
            AnalyzeEntry::NonMatch("a".to_string()),
        ]
    )
}

// analyze-string with a single captured group
#[test]
fn test_analyze_string_007() {
    let regex = Regex::xpath("a(n)", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![
                MatchEntry::String("a".to_string()),
                MatchEntry::Group {
                    nr: 1,
                    value: vec![MatchEntry::String("n".to_string())]
                }
            ]),
            AnalyzeEntry::Match(vec![
                MatchEntry::String("a".to_string()),
                MatchEntry::Group {
                    nr: 1,
                    value: vec![MatchEntry::String("n".to_string())]
                }
            ]),
            AnalyzeEntry::NonMatch("a".to_string()),
        ]
    )
}

// analyze-string with nested captured groups
#[test]
fn test_analyze_string_008() {
    let regex = Regex::xpath("(a(n?))", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();

    let match0 = MatchEntry::Group {
        nr: 1,
        value: vec![
            MatchEntry::String("a".to_string()),
            MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("n".to_string())],
            },
        ],
    };

    let match1 = MatchEntry::Group {
        nr: 1,
        value: vec![
            MatchEntry::String("a".to_string()),
            MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("n".to_string())],
            },
        ],
    };

    let match2 = MatchEntry::Group {
        nr: 1,
        value: vec![
            MatchEntry::String("a".to_string()),
            MatchEntry::Group {
                nr: 2,
                value: vec![],
            },
        ],
    };

    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![match0]),
            AnalyzeEntry::Match(vec![match1]),
            AnalyzeEntry::Match(vec![match2]),
        ]
    );
}

// analyze-string, groups in alternatives
#[test]
fn test_analyze_string_009() {
    let regex = Regex::xpath("(how)|(now)|(brown)|(cow)", "").unwrap();
    let result = regex
        .analyze("how now brown cow")
        .unwrap()
        .collect::<Vec<_>>();

    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 1,
                value: vec![MatchEntry::String("how".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("now".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 3,
                value: vec![MatchEntry::String("brown".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 4,
                value: vec![MatchEntry::String("cow".to_string())]
            }]),
        ]
    );
}

// analyze-string, with i flag
#[test]
fn test_analyze_string_010() {
    let regex = Regex::xpath("(HOW)|(NOW)|(BROWN)|(COW)", "i").unwrap();
    let result = regex
        .analyze("how now brown cow")
        .unwrap()
        .collect::<Vec<_>>();

    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 1,
                value: vec![MatchEntry::String("how".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("now".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 3,
                value: vec![MatchEntry::String("brown".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 4,
                value: vec![MatchEntry::String("cow".to_string())]
            }]),
        ]
    );
}

// analyze-string, with i and x flag
#[test]
fn test_analyze_string_011() {
    let regex = Regex::xpath("(HOW) | (NOW) \n| (BROWN) | (COW)", "ix").unwrap();
    let result = regex
        .analyze("how now brown cow")
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 1,
                value: vec![MatchEntry::String("how".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("now".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 3,
                value: vec![MatchEntry::String("brown".to_string())]
            }]),
            AnalyzeEntry::NonMatch(" ".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 4,
                value: vec![MatchEntry::String("cow".to_string())]
            }]),
        ]
    );
}

// analyze-string, with flags set to empty string
#[test]
fn test_analyze_string_012() {
    let regex = Regex::xpath("(.*?ow\\s+)+", "").unwrap();
    let result = regex
        .analyze("how now brown cow")
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![
                MatchEntry::String("how ".to_string()),
                MatchEntry::Group {
                    nr: 1,
                    value: vec![MatchEntry::String("now ".to_string())]
                }
            ]),
            AnalyzeEntry::NonMatch("brown cow".to_string()),
        ]
    );
}

// analyze-string, with "s" flag
#[test]
fn test_analyze_string_013() {
    let regex = Regex::xpath("Mary.*foot", "s").unwrap();
    let result = regex
        .analyze(
            r"Mary had a little lamb,
its fleece was black as soot,
and everywhere that Mary went,
it put its sooty foot.",
        )
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::String(
                r"Mary had a little lamb,
its fleece was black as soot,
and everywhere that Mary went,
it put its sooty foot"
                    .to_string(),
            )]),
            AnalyzeEntry::NonMatch(".".to_string()),
        ]
    );
}

// analyze-string, multiple lines without "s" flag
#[test]
fn test_analyze_string_014() {
    let regex = Regex::xpath(".+", "").unwrap();
    let result = regex
        .analyze(
            r"Mary had a little lamb,
its fleece was black as soot,
and everywhere that Mary went,
it put its sooty foot.",
        )
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "Mary had a little lamb,".to_string()
            )]),
            AnalyzeEntry::NonMatch("\n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "its fleece was black as soot,".to_string()
            )]),
            AnalyzeEntry::NonMatch("\n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "and everywhere that Mary went,".to_string()
            )]),
            AnalyzeEntry::NonMatch("\n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "it put its sooty foot.".to_string()
            )]),
        ]
    );
}

// analyze-string, multiple lines with "m" flag
#[test]
fn test_analyze_string_015() {
    let regex = Regex::xpath(".+$", "m").unwrap();
    let result = regex
        .analyze(
            r"Mary had a little lamb,
its fleece was black as soot,
and everywhere that Mary went,
it put its sooty foot.",
        )
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "Mary had a little lamb,".to_string()
            )]),
            AnalyzeEntry::NonMatch("\n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "its fleece was black as soot,".to_string()
            )]),
            AnalyzeEntry::NonMatch("\n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "and everywhere that Mary went,".to_string()
            )]),
            AnalyzeEntry::NonMatch("\n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String(
                "it put its sooty foot.".to_string()
            )]),
        ]
    );
}

#[test]
fn test_analyze_string_016() {
    let regex = Regex::xpath("^.+$", "").unwrap();
    let result = regex
        .analyze(
            r"Mary had a little lamb,
its fleece was black as soot,
and everywhere that Mary went,
it put its sooty foot.",
        )
        .unwrap()
        .collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![AnalyzeEntry::NonMatch(
            r"Mary had a little lamb,
its fleece was black as soot,
and everywhere that Mary went,
it put its sooty foot."
                .to_string(),
        ),]
    );
}

// analyze-string, subtle distinction in the positioning of an empty captured group
#[test]
fn test_analyze_string_017() {
    let regex = Regex::xpath("(b)(x?)", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![
                MatchEntry::Group {
                    nr: 1,
                    value: vec![MatchEntry::String("b".to_string())]
                },
                MatchEntry::Group {
                    nr: 2,
                    value: vec![],
                },
            ]),
            AnalyzeEntry::NonMatch("anana".to_string()),
        ]
    );
}

// analyze-string, subtle distinction in the positioning of an empty captured group
#[test]
fn test_analyze_string_017a() {
    let regex = Regex::xpath("(b(x?))", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::Match(vec![MatchEntry::Group {
                nr: 1,
                value: vec![
                    MatchEntry::String("b".to_string()),
                    MatchEntry::Group {
                        nr: 2,
                        value: vec![],
                    },
                ],
            }]),
            AnalyzeEntry::NonMatch("anana".to_string()),
        ]
    );
}

// non-capturing group indicated by "(?:...)"
#[test]
fn test_analyze_string_018() {
    let regex = Regex::xpath("(?:b(an)*a)", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![AnalyzeEntry::Match(vec![
            MatchEntry::String("ban".to_string()),
            MatchEntry::Group {
                nr: 1,
                value: vec![MatchEntry::String("an".to_string())]
            },
            MatchEntry::String("a".to_string()),
        ])]
    );
}

// "q" flag suppresses grouping
#[test]
fn test_analyze_string_019() {
    let regex = Regex::xpath("(banana)", "q").unwrap();
    let result = regex.analyze("((banana))").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("(".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("(banana)".to_string())]),
            AnalyzeEntry::NonMatch(")".to_string()),
        ]
    );
}

// "." does NOT match CR in default mode
#[test]
fn test_analyze_string_026() {
    let regex = Regex::xpath("y.J", "").unwrap();
    let result = regex.analyze("Mary\rJones").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![AnalyzeEntry::NonMatch("Mary\rJones".to_string())]
    );
}

// "." does NOT match CR in default mode
#[test]
fn test_analyze_string_027() {
    let regex = Regex::xpath("y.J", "s").unwrap();
    let result = &regex.analyze("Mary\rJones").unwrap().collect::<Vec<_>>()[1];
    assert_eq!(
        result,
        &AnalyzeEntry::Match(vec![MatchEntry::String("y\rJ".to_string())])
    );
}

// test 29 is so complex it's hard to decipher what it's really testing,
// so skipped it

// error, bad regex pattern
#[test]
fn test_analyze_string_901() {
    let err = Regex::xpath(")-(", "").unwrap_err();
    assert_eq!(err, Error::Syntax("Unmatched close paren".to_string()))
}

// error, bad flags
#[test]
fn test_analyze_string_902() {
    let err = Regex::xpath("abc", "w").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidFlags("Unrecognized flag 'w'".to_string())
    )
}

// analyze-string, error, pattern matches a zero-length string
#[test]
fn test_analyze_string_903() {
    let regex = Regex::xpath("a|b|c?", "").unwrap();
    let result = regex.analyze("abc").unwrap_err();
    assert_eq!(result, Error::MatchesEmptyString);
}
