// Captured groups after backtracking.
//
// XPath F&O 3.1 §5.6.1: "When a capturing sub-expression is matched more than
// once (because it is within a construct that allows repetition), then only
// the last substring that it matched will be captured." A substring matched
// by an attempt that was later backtracked is not part of the match, so it
// must neither be reported nor overwrite (or erase) what the group captured on
// the path that did match. The expected values below agree with Python's
// `re`, Perl and Java, except where noted.

use regexml::{AnalyzeEntry, MatchEntry, Regex};

/// A match span and the `(group number, start, end)` of every group reported
/// for it, in character offsets.
type Captures = ((usize, usize), Vec<(usize, usize, usize)>);

/// The first match of `pattern` in `input` and the groups `analyze` reports
/// for it, sorted by group number.
fn first_match(pattern: &str, input: &str) -> Option<Captures> {
    fn walk(entries: &[MatchEntry], pos: &mut usize, groups: &mut Vec<(usize, usize, usize)>) {
        for entry in entries {
            match entry {
                MatchEntry::String(s) => *pos += s.chars().count(),
                MatchEntry::Group { nr, value } => {
                    let start = *pos;
                    walk(value, pos, groups);
                    groups.push((*nr, start, *pos));
                }
            }
        }
    }
    let regex = Regex::xpath(pattern, "").unwrap();
    let mut pos = 0;
    for entry in regex.analyze(input).unwrap() {
        match entry {
            AnalyzeEntry::NonMatch(s) => pos += s.chars().count(),
            AnalyzeEntry::Match(entries) => {
                let start = pos;
                let mut groups = Vec::new();
                walk(&entries, &mut pos, &mut groups);
                groups.sort();
                return Some(((start, pos), groups));
            }
        }
    }
    None
}

fn replace(pattern: &str, input: &str, replacement: &str) -> String {
    Regex::xpath(pattern, "")
        .unwrap()
        .replace_all(input, replacement)
        .unwrap()
}

// The second repetition first matches `(a)` at offset 1, which fails later
// on, and then `(ab)`: group 2 keeps the `a` of the first repetition.
#[test]
fn a_backtracked_alternative_keeps_the_earlier_capture() {
    assert_eq!(
        first_match("((a)|(ab))+c", "aabc"),
        Some(((0, 4), vec![(1, 1, 3), (2, 0, 1), (3, 1, 3)]))
    );
    assert_eq!(replace("((a)|(ab))+c", "aabc", "[$1|$2|$3]"), "[ab|a|ab]");
    let regex = Regex::xpath("((a)|(ab))+c", "").unwrap();
    assert_eq!(
        regex.analyze("aabc").unwrap().collect::<Vec<_>>(),
        vec![AnalyzeEntry::Match(vec![
            MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("a".to_string())]
            },
            MatchEntry::Group {
                nr: 1,
                value: vec![MatchEntry::Group {
                    nr: 3,
                    value: vec![MatchEntry::String("ab".to_string())]
                }]
            },
            MatchEntry::String("c".to_string()),
        ])]
    );
}

#[test]
fn the_last_successful_capture_is_kept_over_several_repetitions() {
    assert_eq!(
        first_match("((a)|(ab))+c", "aaabc"),
        Some(((0, 5), vec![(1, 2, 4), (2, 1, 2), (3, 2, 4)]))
    );
    assert_eq!(replace("((a)|(ab))+c", "aaabc", "[$2]"), "[a]");
}

// A group whose only attempt was backtracked has captured nothing: it is not
// reported at all (it used to be reported as an empty group).
#[test]
fn a_group_whose_attempts_all_failed_is_not_captured() {
    assert_eq!(
        first_match("((a)|(ab))+c", "abc"),
        Some(((0, 3), vec![(1, 0, 2), (3, 0, 2)]))
    );
    assert_eq!(replace("((a)|(ab))+c", "abc", "[$2]"), "[]");
}

#[test]
fn a_capture_of_an_earlier_repetition_survives_a_later_alternative() {
    assert_eq!(
        first_match("((a)|(ab))+c", "abac"),
        Some(((0, 4), vec![(1, 2, 3), (2, 2, 3), (3, 0, 2)]))
    );
}

// Back-references see the restored capture.
#[test]
fn a_back_reference_matches_the_restored_capture() {
    assert_eq!(
        first_match(r"((a)|(ab))+c\2", "aabca"),
        Some(((0, 5), vec![(1, 1, 3), (2, 0, 1), (3, 1, 3)]))
    );
    let anchored = Regex::xpath(r"^((a)|(ab))+c\2$", "").unwrap();
    assert!(anchored.is_match("aabca"));
    // `\2` is "a", not the zero-length string
    assert!(!anchored.is_match("aabc"));
}

// A greedy repetition of a fixed-length group that backs off keeps the
// capture of the last repetition it still reports.
#[test]
fn backing_off_a_fixed_length_repetition_restores_the_capture() {
    assert_eq!(
        first_match("(a)*ab", "aab"),
        Some(((0, 3), vec![(1, 0, 1)]))
    );
    let regex = Regex::xpath(r"^([ab])*b\1$", "").unwrap();
    // `([ab])*` backs off to "ab", so `\1` is "b"
    assert!(regex.is_match("abbb"));
    assert_eq!(replace(r"^([ab])*b\1$", "abbb", "[$1]"), "[b]");
}

// The same for a repetition of variable length.
#[test]
fn backing_off_a_variable_length_repetition_restores_the_capture() {
    assert_eq!(
        first_match("(a|bc)*bcd", "abcbcd"),
        Some(((0, 6), vec![(1, 1, 3)]))
    );
}

// A reluctant repetition that has to take one more repetition keeps the
// captures of the earlier ones.
#[test]
fn a_reluctant_repetition_keeps_the_captures_of_its_earlier_repetitions() {
    assert_eq!(
        first_match("((a)|(b))+?c", "abc"),
        Some(((0, 3), vec![(1, 1, 2), (2, 0, 1), (3, 1, 2)]))
    );
}

// XPath F&O 3.1 §5.6.1: "If no string has been matched by the Nth capturing
// sub-expression, the back-reference is interpreted as matching a zero-length
// string" (Perl, Python and Java fail such a back-reference instead).
#[test]
fn a_back_reference_to_a_group_that_has_not_matched_matches_the_empty_string() {
    assert_eq!(first_match(r"(a)|b\1", "b"), Some(((0, 1), vec![])));
    assert_eq!(first_match(r"(?:x|(a))b\1", "xb"), Some(((0, 2), vec![])));
}

// A repetition that matches its operand zero times leaves the groups in the
// operand unmatched, also when an earlier repetition of an enclosing
// repetition had captured them. This is what the W3C XQuery/XPath 3 test
// suite expects (fn-matches-50, tests p303 and p908 of perl-tests.xml, W3C bug
// 29253); F&O 3.1 §5.6.1 leaves the substring captured in nested repeating
// constructs implementation-dependent. (Perl, Python and Java keep the earlier
// capture and do not match p303.)
#[test]
fn zero_repetitions_leave_the_groups_of_the_operand_unmatched() {
    // p303: in the second repetition `(.)?` matches nothing, so `\2` matches
    // the zero-length string
    assert_eq!(
        first_match(r"^((.)?a\2)+$", "babadad"),
        Some(((0, 7), vec![(1, 4, 7), (2, 4, 5)]))
    );
    // p908
    assert!(!Regex::xpath(r"(\w)?(abc)\1b", "")
        .unwrap()
        .is_match("abcab"));
    // the same with a group of variable length and with a reluctant `??`
    assert!(Regex::xpath(r"^((a|bc)?x\2)+$", "")
        .unwrap()
        .is_match("bcxbcx"));
    assert!(Regex::xpath(r"^((.)??a\2)+$", "")
        .unwrap()
        .is_match("babadad"));
    // zero repetitions with nothing captured before: the group is unmatched
    assert_eq!(first_match("(a)?b", "b"), Some(((0, 1), vec![])));
}

// Captures of a failed match attempt at an earlier start position are gone
// when a later one succeeds.
#[test]
fn a_failed_attempt_leaves_no_captures() {
    assert_eq!(
        first_match("(a)(b)c|(a)d", "abad"),
        Some(((2, 4), vec![(3, 2, 3)]))
    );
    assert_eq!(replace("(a)(b)c|(a)d", "abad", "[$1|$2|$3]"), "ab[||a]");
}

// Patterns without groups take none of the capture bookkeeping; still
// correct.
#[test]
fn patterns_without_groups() {
    assert_eq!(first_match(r"\p{L}+", "12ab3"), Some(((2, 4), vec![])));
    assert_eq!(first_match("(?:a|ab)+c", "aabc"), Some(((0, 4), vec![])));
}
