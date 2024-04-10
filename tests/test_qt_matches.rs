// these test cases are a transliteration of test cases in matches.xml
// part of the qt3 test suite

use regexml::{Error, Regex};

#[test]
fn test_matches2args_1() {
    let regex = Regex::xpath("This is a characte", "").unwrap();
    assert!(regex.is_match("This is a characte"));
}

#[test]
fn test_matches_err_1() {
    let regex = Regex::xpath("bra", "p");
    assert_eq!(
        regex.unwrap_err(),
        Error::InvalidFlags("Unrecognized flag 'p'".to_string())
    );
}

// back-reference illegal in square backets. See erratum FO.E24
#[test]
fn test_matches_err_2() {
    let regex = Regex::xpath("^(#)abc[\\1]1$", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Backreferences not allowed within character classes".to_string())
    );
}

// single-digit back-reference to non-existent group. See erratum FO.E24
#[test]
fn test_matches_err_3() {
    let regex = Regex::xpath("^(#)abc\\2$", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\2 (no such group)".to_string())
    );
}

// single-digit back-reference to group not yet closed. See erratum FO.E24
#[test]
fn test_matches_err_4() {
    let regex = Regex::xpath("^((#)abc\\1)$", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\1 (group not yet closed)".to_string())
    );
}

// double-digit back-reference to group not yet closed. See erratum FO.E24
#[test]
fn test_matches_err_5() {
    let regex = Regex::xpath(
        "^(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)((m)(n)(o)(p)(q)\\13)$",
        "",
    );
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\13 (group not yet closed)".to_string())
    );
}

// Evaluation of matches function as per example 1 (for this function)
#[test]
fn test_matches_1() {
    let regex = Regex::xpath("bra", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

// Evaluation of matches function as per example 2 (for this function). Pattern
// set to "^a.*a$".
#[test]
fn test_matches_2() {
    let regex = Regex::xpath("^a.*a$", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

// Evaluation of matches function as per example 3 (for this function). Pattern
// set to "^bra"
#[test]
fn test_matches_3() {
    let regex = Regex::xpath("^bra", "").unwrap();
    assert!(!regex.is_match("abracadabra"));
}

// Evaluate the fn:matches function with the input string set to the empty
// sequence.
#[test]
fn test_matches_5() {
    let regex = Regex::xpath("^bra", "").unwrap();
    assert!(!regex.is_match("()"));
}

// Evaluation of matches function with pattern set to "\^".
#[test]
fn test_matches_6() {
    let regex = Regex::xpath("\\^", "").unwrap();
    assert!(regex.is_match("abracadabra^abracadabra"));
}

// Evaluation of matches function with pattern set to "\?" for an input string
// that contains "?".
#[test]
fn test_matches_7() {
    let regex = Regex::xpath("\\?", "").unwrap();
    assert!(regex.is_match("abracadabra?abracadabra"));
}

// Evaluation of matches function with pattern set to "\*" for an input string
// that contains "*".
#[test]
fn test_matches_8() {
    let regex = Regex::xpath("\\*", "").unwrap();
    assert!(regex.is_match("abracadabra*abracadabra"));
}

// Evaluation of matches function with pattern set to "\+" for an input string
// that contains "+".
#[test]
fn test_matches_9() {
    let regex = Regex::xpath("\\+", "").unwrap();
    assert!(regex.is_match("abracadabra+abracadabra"));
}

// Evaluation of matches function with pattern set to "\{" for an input string
// that contains "}".
#[test]
fn test_matches_10() {
    let regex = Regex::xpath("\\{", "").unwrap();
    assert!(regex.is_match("abracadabra{abracadabra"));
}

// Evaluation of matches function with pattern set to "\}" for an input string
// that contains "}".
#[test]
fn test_matches_11() {
    let regex = Regex::xpath("\\}", "").unwrap();
    assert!(regex.is_match("abracadabra}abracadabra"));
}

// Evaluation of matches function with pattern set to "\(" for an input string
// that contains "(".
#[test]
fn test_matches_12() {
    let regex = Regex::xpath("\\(", "").unwrap();
    assert!(regex.is_match("abracadabra(abracadabra"));
}

// Evaluation of matches function with pattern set to "\)" for an input string
// that contains ")".
#[test]
fn test_matches_13() {
    let regex = Regex::xpath("\\)", "").unwrap();
    assert!(regex.is_match("abracadabra)abracadabra"));
}

// Evaluation of matches function with pattern set to "\[" for an input string
// that contains "[".
#[test]
fn test_matches_14() {
    let regex = Regex::xpath("\\[", "").unwrap();
    assert!(regex.is_match("abracadabra[abracadabra"));
}

// Evaluation of matches function with pattern set to "\]" for an input string
// that contains "]".
#[test]
fn test_matches_15() {
    let regex = Regex::xpath("\\]", "").unwrap();
    assert!(regex.is_match("abracadabra]abracadabra"));
}

// Evaluation of matches function with pattern set to "\-" for an input string
// that contains "-".
#[test]
fn test_matches_16() {
    let regex = Regex::xpath("\\-", "").unwrap();
    assert!(regex.is_match("abracadabra-abracadabra"));
}

// Evaluation of matches function with pattern set to "\." for an input string
// that contains ".".
#[test]
fn test_matches_17() {
    let regex = Regex::xpath("\\.", "").unwrap();
    assert!(regex.is_match("abracadabra.abracadabra"));
}

// Evaluation of matches function with pattern set to "\|" for an input string
// that contains "|".
#[test]
fn test_matches_18() {
    let regex = Regex::xpath("\\|", "").unwrap();
    assert!(regex.is_match("abracadabra|abracadabra"));
}

// Evaluation of matches function with pattern set to "\\" for an input string
// that contains "\".
#[test]
fn test_matches_19() {
    let regex = Regex::xpath("\\\\", "").unwrap();
    assert!(regex.is_match("abracadabra\\abracadabra"));
}

// Evaluation of matches function with pattern set to "\t" for an input string
// that contains the tab character.
#[test]
fn test_matches_20() {
    let regex = Regex::xpath("\\t", "").unwrap();
    assert!(regex.is_match("abracadabra\tabracadabra"));
}

// Evaluation of matches function with pattern set to "\n" for an input string
// that contains the newline character.
#[test]
fn test_matches_21() {
    let regex = Regex::xpath("\\n", "").unwrap();
    assert!(regex.is_match("abracadabra\nabracadabra"));
}

// Evaluation of matches function with pattern set to "aa{1}" (exact quantity)
// for an input string that contains the "aa" string.
#[test]
fn test_matches_22() {
    let regex = Regex::xpath("aa{1}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabra"));
}

// Evaluation of matches function with pattern set to "aa{1,}" (min quantity)
// for an input string that contains the "aa" string twice.
#[test]
fn test_matches_23() {
    let regex = Regex::xpath("aa{1,}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabraabracadabra"));
}

// Evaluation of matches function with pattern set to "aa{1,2}" (range
// quantity) for an input string that contains the "aa" string twice.
#[test]
fn test_matches_24() {
    let regex = Regex::xpath("aa{1,2}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabraabracadabra"));
}

// Evaluation of matches function with invalid regular expression
#[test]
fn test_matches_err_25() {
    let regex = Regex::xpath("**%%", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No expression before quantifier".to_string())
    );
}

// Check for the correct behavior of ^ and $ in multi-line mode This test case
// was motivated by the resolution of Bug Report 4543. Note that '^' matches
// the position after any newline other than a newline that is the last
// character in the input string.
#[test]
fn test_matches_26() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(!regex.is_match("abcd\ndefg\n"));
}

// Check for the correct behavior of ^ and $ in multi-line mode This test case
// was motivated by the resolution of Bug Report 4543.
#[test]
fn test_matches_27() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(regex.is_match("\nabcd\ndefg\n"));
}

// Check for the correct behavior of ^ and $ in multi-line mode This test case
// was motivated by the resolution of Bug Report 4543
#[test]
fn test_matches_28() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(regex.is_match("abcd\n\ndefg\n"));
}

// 2-digits not treated as a back-reference See erratum FO.E24
#[test]
fn test_matches_29() {
    let regex = Regex::xpath("^(#)abc\\11$", "").unwrap();
    assert!(regex.is_match("#abc#1"));
}

// 2-digits treated as a back-reference See erratum FO.E24
#[test]
fn test_matches_30() {
    let regex = Regex::xpath(
        "^(#)(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)(m)(n)(o)(p)(q)\\11$",
        "",
    )
    .unwrap();
    assert!(!regex.is_match("#abcdefghijklmnopq#1"));
}

// Evaluation of matches function with non-capturing groups (allowed in XPath
// 3.0)
#[test]
fn test_matches_31() {
    let regex = Regex::xpath("(?:abra(?:cad)?)*", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

// Evaluation of matches function with "q" flag (allowed in XQuery 3.0)
#[test]
fn test_matches_32() {
    let regex = Regex::xpath("(?:abra(?:cad)?)*", "q").unwrap();
    assert!(!regex.is_match("abracadabra"));
}

// Evaluation of matches function with "q" flag (allowed in XQuery 3.0)
#[test]
fn test_matches_33() {
    let regex = Regex::xpath("x[y-z]", "q").unwrap();
    assert!(regex.is_match("x[y-z]"));
}

// Evaluation of matches function with "q" and "i" flags (allowed in XQuery 3.0)
#[test]
fn test_matches_34() {
    let regex = Regex::xpath("x[y-z]", "qi").unwrap();
    assert!(regex.is_match("X[y-Z]"));
}

// Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \99
// is an invalid reference as 99th subexpression does not exist
#[test]
fn test_matches_35() {
    let regex = Regex::xpath("(a)\\99", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\9 (no such group)".to_string())
    );
}

// Test for bug fix of 5348 in Errata for F+O. ok match here
#[test]
fn test_matches_36() {
    let regex = Regex::xpath("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)\\10", "").unwrap();
    assert!(regex.is_match("abcdefghijj"));
}

// Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \11
// reference is made before the closing right parenthesis of 11th reference
#[test]
fn test_matches_37() {
    let regex = Regex::xpath("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k\\11)", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\11 (group not yet closed)".to_string())
    );
}

// Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \10
// reference is made before the closing right parenthesis of 10th reference
#[test]
fn test_matches_38() {
    let regex = Regex::xpath("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j\\10)", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\10 (group not yet closed)".to_string())
    );
}

// Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \9
// reference is made before the closing right parenthesis of 9th reference
#[test]
fn test_matches_39() {
    let regex = Regex::xpath("(a)(b)(c)(d)(e)(f)(g)(h)(i\\9)", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\9 (group not yet closed)".to_string())
    );
}

// Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \1
// reference is made before the closing right parenthesis of 1st reference
#[test]
fn test_matches_40() {
    let regex = Regex::xpath("(a\\1)", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\1 (group not yet closed)".to_string())
    );
}

// Handling of final newline with non-multiline mode
#[test]
fn test_matches_41() {
    let regex = Regex::xpath("Mary$", "").unwrap();
    assert!(!regex.is_match("Mary\n"));
}

// Handling of final newline with $ in dot-all mode
#[test]
fn test_matches_42() {
    let regex = Regex::xpath("Mary$", "s").unwrap();
    assert!(!regex.is_match("Mary\n"));
}

// "." doesn't normally match newline
#[test]
fn test_matches_43() {
    let regex = Regex::xpath("Mary.Jones", "").unwrap();
    assert!(!regex.is_match("Mary\nJones"));
}

// "." does match newline in dot-all mode
#[test]
fn test_matches_44() {
    let regex = Regex::xpath("Mary.Jones", "s").unwrap();
    assert!(regex.is_match("Mary\nJones"));
}

// "." does NOT match CR in default mode
#[test]
fn test_matches_45() {
    let regex = Regex::xpath("Mary.Jones", "").unwrap();
    assert!(!regex.is_match("Mary\rJones"));
}

// "." does match CR in dot-all mode
#[test]
fn test_matches_46() {
    let regex = Regex::xpath("Mary.Jones", "s").unwrap();
    assert!(regex.is_match("Mary\rJones"));
}

// Check for the correct behavior of $ when not in multi-line mode. The correct
// answer according to the spec is false; though some regex engines are known to
// report true.
#[test]
fn test_matches_47() {
    let regex = Regex::xpath("g$", "").unwrap();
    assert!(!regex.is_match("abcd\ndefg\n"));
}

// Edge condition: match occurs at last character.
#[test]
fn test_matches_48() {
    let regex = Regex::xpath("\\.", "").unwrap();
    assert!(regex.is_match("abracadabra-abracadabra."));
}

// Edge condition: match occurs at last character.
#[test]
fn test_matches_49() {
    let regex = Regex::xpath("(124|864|377|3)", "").unwrap();
    assert!(regex.is_match("abracadabra-abracadabra-3"));
}

// Skip fn-matches-50 for now, which runs regexes in matches/perl-tests.xml for now

// Unescaped left parens inside a charClass are allowed and don't affect the
// subexpression count
#[test]
fn test_matches_51() {
    let regex = Regex::xpath("^(ab)([()]*)(cd)([)(]*)ef\\4gh$", "").unwrap();
    assert!(regex.is_match("ab()cd()ef()gh"));
}

// A use case involving backtracking and ambiguity
#[test]
fn test_matches_52() {
    let regex = Regex::xpath("^(a*b?a*){3,3}$", "").unwrap();
    assert!(regex.is_match("aaababaaabaa"));
}

// A use case involving repetition of a back-reference. Saxon bug 3712.
#[test]
fn test_matches_53() {
    let regex = Regex::xpath("([A-Z])\\1*", "").unwrap();
    assert!(regex.is_match("A"));
}

// A use case involving optional matching of start-of-string. Saxon bug 3782.
#[test]
fn test_matches_54() {
    let regex = Regex::xpath("(^|:)?Z", "").unwrap();
    assert!(regex.is_match("kZ"));
}

// Matching reluctant quantifier with min cardinality. See Saxon bug 3902
#[test]
fn test_matches_55() {
    let regex = Regex::xpath("^(a{3,}?)b", "").unwrap();
    let found: Vec<bool> = ["b", "ab", "aab", "aaab", "aaaab", "aaaaab"]
        .iter()
        .map(|s| regex.is_match(s))
        .collect();
    assert_eq!(found, [false, false, false, true, true, true]);
}

// Matching reluctant quantifier with max cardinality. See Saxon bug 3902
#[test]
fn test_matches_56() {
    let regex = Regex::xpath("^(a{0,3}?)b", "").unwrap();
    let found: Vec<bool> = ["b", "ab", "aab", "aaab", "aaaab", "aaaaab"]
        .iter()
        .map(|s| regex.is_match(s))
        .collect();
    assert_eq!(found, [true, true, true, true, false, false]);
}

// Matching reluctant quantifier with min and max cardinality. See Saxon bug 3902
#[test]
fn test_matches_57() {
    let regex = Regex::xpath("^(a{2,3}?)b", "").unwrap();
    let found: Vec<bool> = ["b", "ab", "aab", "aaab", "aaaab", "aaaaab"]
        .iter()
        .map(|s| regex.is_match(s))
        .collect();
    assert_eq!(found, [false, false, true, true, false, false]);
}

// Matching reluctant quantifier with min cardinality, variable length item
// that repeats. See Saxon bug 3902
#[test]
fn test_matches_58() {
    let regex = Regex::xpath("^((az?){3,}?)b", "").unwrap();
    dbg!(&regex);
    let found: Vec<bool> = ["b", "ab", "aab", "aaab", "aaazab", "aaaaab"]
        .iter()
        .map(|s| {
            dbg!(s);
            regex.is_match(s)
        })
        .collect();
    assert_eq!(found, [false, false, false, true, true, true]);
}

// Matching reluctant quantifier with max cardinality, variable length item
// that repeats. See Saxon bug 3902
#[test]
fn test_matches_59() {
    let regex = Regex::xpath("^((az?){0,3}?)b", "").unwrap();
    let found: Vec<bool> = ["b", "ab", "aazb", "aaab", "aaaab", "aaaaab"]
        .iter()
        .map(|s| regex.is_match(s))
        .collect();
    assert_eq!(found, [true, true, true, true, false, false]);
}

// Matching reluctant quantifier with min and max cardinality, variable length
#[test]
fn test_matches_60() {
    let regex = Regex::xpath("^((az?){2,3}?)b", "").unwrap();
    let found: Vec<bool> = ["b", "ab", "aazb", "aaab", "aaaab", "aaaaab"]
        .iter()
        .map(|s| regex.is_match(s))
        .collect();
    assert_eq!(found, [false, false, true, true, false, false]);
}

#[test]
fn test_matches_60a() {
    let regex = Regex::xpath("^((az?){2,3}?)b", "").unwrap();
    assert!(regex.is_match("aaab"));
}

// Matching reluctant quantifier with min and max cardinality, backtracking required. See Saxon bug 3902
#[test]
fn test_matches_61() {
    let regex = Regex::xpath("^((az?){2,3}?)a$", "").unwrap();
    let found: Vec<bool> = ["b", "aa", "aaza", "aaaa", "aaaaa", "aaaaaa"]
        .iter()
        .map(|s| regex.is_match(s))
        .collect();
    assert_eq!(found, [false, false, true, true, false, false]);
}
