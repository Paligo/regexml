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
    let found: Vec<bool> = ["b", "ab", "aab", "aaab", "aaazab", "aaaaab"]
        .iter()
        .map(|s| regex.is_match(s))
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

// Simple call of matches() with "i" flag
#[test]
fn test_caselessmatch01() {
    let regex = Regex::xpath("ABC", "i").unwrap();
    assert!(regex.is_match("abc"));
}

// Call of matches() with "i" flag and a character range
#[test]
fn test_caselessmatch02() {
    let regex = Regex::xpath("[A-Z]*", "i").unwrap();
    assert!(regex.is_match("abZ"));
}

// Call of matches() with "i" flag and a character range
#[test]
fn test_caselessmatch03() {
    let regex = Regex::xpath("[a-z]*", "i").unwrap();
    assert!(regex.is_match("abZ"));
}

// Call of matches() with "i" flag and Kelvin sign Kelvin sign
#[test]
fn test_caselessmatch04() {
    let regex = Regex::xpath("[A-Z]", "i").unwrap();
    assert!(regex.is_match("\u{212A}"));
}

// Call of matches() with "i" flag and Kelvin sign Kelvin sign
#[test]
fn test_caselessmatch05() {
    let regex = Regex::xpath("[a-z]", "i").unwrap();
    assert!(regex.is_match("\u{212A}"));
}

// Call of matches() with "i" flag and Kelvin sign
#[test]
fn test_caselessmatch06() {
    let regex = Regex::xpath("K", "i").unwrap();
    assert!(regex.is_match("\u{212A}"));
}

// Call of matches() with "i" flag and Kelvin sign
#[test]
fn test_caselessmatch07() {
    let regex = Regex::xpath("k", "i").unwrap();
    assert!(regex.is_match("\u{212A}"));
}

// Call of matches() with "i" flag and range subtraction
#[test]
fn test_caselessmatch08() {
    let regex = Regex::xpath("[A-Z-[OI]]", "i").unwrap();
    assert!(regex.is_match("x"));
}

// Call of matches() with "i" flag and range subtraction
#[test]
fn test_caselessmatch09() {
    let regex = Regex::xpath("[A-Z-[OI]]", "i").unwrap();
    assert!(regex.is_match("X"));
}

// Call of matches() with "i" flag and range subtraction
#[test]
fn test_caselessmatch10() {
    let regex = Regex::xpath("[A-Z-[OI]]", "i").unwrap();
    assert!(!regex.is_match("O"));
}

// Call of matches() with "i" flag and range subtraction
#[test]
fn test_caselessmatch11() {
    let regex = Regex::xpath("[A-Z-[OI]]", "i").unwrap();
    assert!(!regex.is_match("i"));
}

// Call of matches() with "i" flag and negation
#[test]
fn test_caselessmatch12() {
    let regex = Regex::xpath("[^Q]", "i").unwrap();
    assert!(!regex.is_match("Q"));
}

// Call of matches() with "i" flag and negation
#[test]
fn test_caselessmatch13() {
    let regex = Regex::xpath("[^Q]", "i").unwrap();
    assert!(!regex.is_match("q"));
}

// Call of matches() with "i" flag and upper-case category
#[test]
fn test_caselessmatch14() {
    let regex = Regex::xpath(r#"\p{Lu}"#, "i").unwrap();
    assert!(!regex.is_match("m"));
}

// Call of matches() with "i" flag and upper-case category
#[test]
fn test_caselessmatch15() {
    let regex = Regex::xpath(r#"\P{Lu}"#, "i").unwrap();
    assert!(regex.is_match("m"));
}

// The flags argument cannot contain whitespace.
#[test]
fn test_k_matches_fun_5() {
    let regex = Regex::xpath("pattern", " ");
    assert_eq!(
        regex.unwrap_err(),
        Error::InvalidFlags("Unrecognized flag ' '".to_string())
    );
}

// The flags argument cannot contain 'X'.
#[test]
fn test_k_matches_fun_6() {
    let regex = Regex::xpath("pattern", "X");
    assert_eq!(
        regex.unwrap_err(),
        Error::InvalidFlags("Unrecognized flag 'X'".to_string())
    );
}

// Whitespace in the regexp is collapsed.
#[test]
fn test_k2_matches_func_1() {
    let regex = Regex::xpath(r#"hello\ sworld"#, "x").unwrap();
    assert!(regex.is_match("hello world"));
}

// Whitespace(before) in the regexp is collapsed, but not inside a character class.
#[test]
fn test_k2_matches_func_2() {
    let regex = Regex::xpath(" hello[ ]world", "x").unwrap();
    assert!(regex.is_match("hello world"));
}

// Whitespace(after) in the regexp is collapsed, but not inside a character class.
#[test]
fn test_k2_matches_func_3() {
    let regex = Regex::xpath("hello[ ]world ", "x").unwrap();
    assert!(regex.is_match("hello world"));
}

// Whitespace(in the middle) in the regexp is collapsed, but not inside a character class.
#[test]
fn test_k2_matches_func_4() {
    let regex = Regex::xpath("he ll o[ ]worl d", "x").unwrap();
    assert!(regex.is_match("hello world"));
}

// Whitespace in the regexp is collapsed, and should therefore compile.
#[test]
fn test_k2_matches_func_5() {
    let regex = Regex::xpath("\\p{ IsBasicLatin}+", "x").unwrap();
    assert!(regex.is_match("hello world"));
}

// Whitespace in the regexp is collapsed completely, and should therefore compile and match.
#[test]
fn test_k2_matches_func_6() {
    let regex = Regex::xpath("\\p{ I s B a s i c L a t i n }+", "x").unwrap();
    assert!(regex.is_match("hello world"));
}

// Whitespace in the regexp is not collapsed, and should therefore not compile.
#[test]
fn test_k2_matches_func_7() {
    let regex = Regex::xpath("\\p{ IsBasicLatin}+", "");
    let err = regex.unwrap_err();
    assert_eq!(
        err,
        Error::Syntax("Unknown character category:  IsBasicLatin".to_string())
    );
}

// Since no string is captured by the back-reference, the single character is matched.
#[test]
fn test_k2_matches_func_8() {
    let regex = Regex::xpath("(.)\\3", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\3 (no such group)".to_string())
    );
}

// Since no string is captured by the back-reference, the single character is matched.
#[test]
fn test_k2_matches_func_9() {
    let regex = Regex::xpath("(.)\\2", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\2 (no such group)".to_string())
    );
}

// A non-matching backwards-reference matches the empty string.
#[test]
fn test_k2_matches_func_10() {
    let regex = Regex::xpath("\\3", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\3 (no such group)".to_string())
    );
}

// Use a back reference inside a character class.
#[test]
fn test_k2_matches_func_11() {
    let regex = Regex::xpath("(asd)[\\1]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Backreferences not allowed within character classes".to_string())
    );
}

// Use a back reference inside a character class.
#[test]
fn test_k2_matches_func_12() {
    let regex = Regex::xpath("(asd)[asd\\1]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Backreferences not allowed within character classes".to_string())
    );
}

// Use a back reference inside a character class.
#[test]
fn test_k2_matches_func_13() {
    let regex = Regex::xpath("(asd)[asd\\0]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Octal escapes are not allowed".to_string())
    );
}

// Use a back reference inside a character class.
#[test]
fn test_k2_matches_func_14() {
    let regex = Regex::xpath("1[asd\\0]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Octal escapes are not allowed".to_string())
    );
}

// A negative character class never match a non-character
#[test]
fn test_k2_matches_func_15() {
    let regex = Regex::xpath("a[^b]", "").unwrap();
    assert!(!regex.is_match("a"));
    assert!(regex.is_match("a "));
}

// Use a pattern whose interpretation is unknown. See public report 4466 and 21425.
// xsd version 1.1
#[test]
fn test_k2_matches_func_16() {
    let regex = Regex::xpath("[0-9-.]*/", "").unwrap();
    assert!(!regex.is_match("input"));
}

// Caseless match with back-reference.
#[test]
fn test_k2_matches_func_17() {
    let regex = Regex::xpath("(a)\\1", "i").unwrap();
    assert!(regex.is_match("aA"));
}

// Test an invalid negative pos char group
#[test]
fn test_cbcl_matches_001() {
    let regex = Regex::xpath("[^]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Empty negative character group".to_string())
    );
}

// Test an invalid char range
#[test]
fn test_cbcl_matches_002() {
    let regex = Regex::xpath("[a-\\b]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Escape character 'b' not allowed".to_string())
    );
}

// Test a two-digit back reference
#[test]
fn test_cbcl_matches_003() {
    let regex = Regex::xpath(
        "(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)\\1\\2\\3\\4\\5\\6\\7\\8\\9\\10\\11",
        "",
    )
    .unwrap();
    assert!(regex.is_match("abcdefghijkabcdefghijk"));
}

// Test a very large exact quantifier
#[test]
fn test_cbcl_matches_004() {
    let err = Regex::xpath("a{99999999999999999999999999}", "").unwrap_err();
    assert_eq!(err, Error::Syntax("Expected valid number".to_string()));
}

// Test with an invalid character range
#[test]
fn test_cbcl_matches_005() {
    let regex = Regex::xpath("[a--]", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Unescaped hyphen cannot act as end of range".to_string())
    );
}

// Test with a character class containing an escaped character
#[test]
fn test_cbcl_matches_006() {
    let regex = Regex::xpath("[\t]", "").unwrap();
    assert!(regex.is_match("\t"));
}

// Test with a character class beginning with a '-'
#[test]
fn test_cbcl_matches_007() {
    let regex = Regex::xpath("[-ab]+", "").unwrap();
    assert!(regex.is_match("-abba-"));
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_008() {
    let regex = Regex::xpath("\\P{L", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_009() {
    let regex = Regex::xpath("\\P{M", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_010() {
    let regex = Regex::xpath("\\P{N", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_011() {
    let regex = Regex::xpath("\\P{P", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_012() {
    let regex = Regex::xpath("\\P{Z", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_013() {
    let regex = Regex::xpath("\\P{S", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test a badly formed category name
#[test]
fn test_cbcl_matches_014() {
    let regex = Regex::xpath("\\P{C", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No closing '}' after \\P".to_string())
    );
}

// Test category name L
#[test]
fn test_cbcl_matches_015() {
    let regex = Regex::xpath("\\P{L}", "").unwrap();
    assert!(!regex.is_match("foo"));
}

// Test category name M
#[test]
fn test_cbcl_matches_016() {
    let regex = Regex::xpath("\\P{M}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name N
#[test]
fn test_cbcl_matches_017() {
    let regex = Regex::xpath("\\P{N}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name P
#[test]
fn test_cbcl_matches_018() {
    let regex = Regex::xpath("\\P{P}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Z
#[test]
fn test_cbcl_matches_019() {
    let regex = Regex::xpath("\\P{Z}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name S
#[test]
fn test_cbcl_matches_020() {
    let regex = Regex::xpath("\\P{S}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name C
#[test]
fn test_cbcl_matches_021() {
    let regex = Regex::xpath("\\P{C}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Lu
#[test]
fn test_cbcl_matches_022() {
    let regex = Regex::xpath("\\P{Lu}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Me
#[test]
fn test_cbcl_matches_023() {
    let regex = Regex::xpath("\\P{Me}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name No
#[test]
fn test_cbcl_matches_024() {
    let regex = Regex::xpath("\\P{No}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Pf
#[test]
fn test_cbcl_matches_025() {
    let regex = Regex::xpath("\\P{Pf}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Zs
#[test]
fn test_cbcl_matches_026() {
    let regex = Regex::xpath("\\P{Zs}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Sk
#[test]
fn test_cbcl_matches_027() {
    let regex = Regex::xpath("\\P{Sk}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test category name Cc
#[test]
fn test_cbcl_matches_028() {
    let regex = Regex::xpath("\\P{Cc}", "").unwrap();
    assert!(regex.is_match("foo"));
}

// Test invalid category name La
#[test]
fn test_cbcl_matches_029() {
    let regex = Regex::xpath("\\P{La}", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Unknown unicode general category La".to_string())
    );
}

// Test invalid category name Ma
#[test]
fn test_cbcl_matches_030() {
    let regex = Regex::xpath("\\P{Ma}", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Unknown unicode general category Ma".to_string())
    );
}

// Test invalid category name Na
#[test]
fn test_cbcl_matches_031() {
    let regex = Regex::xpath("\\P{Na}", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Unknown unicode general category Na".to_string())
    );
}

// Test invalid category name Pa
#[test]
fn test_cbcl_matches_032() {
    let regex = Regex::xpath("\\P{Pa}", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Unknown unicode general category Pa".to_string())
    );
}

// Test invalid category name Za
#[test]
fn test_cbcl_matches_033() {
    let regex = Regex::xpath("\\P{Za}", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Unknown unicode general category Za".to_string())
    );
}

// Test an empty branch
#[test]
fn test_cbcl_matches_036() {
    let regex = Regex::xpath("a()b", "").unwrap();
    assert!(!regex.is_match("foo"));
}
// Test a multibyte Unicode character
#[test]
fn test_cbcl_matches_037() {
    let regex = Regex::xpath("\u{10000}", "").unwrap();
    assert!(regex.is_match("\u{10000}"));
}

// Test a large exact quantifier
#[test]
fn test_cbcl_matches_038() {
    let regex = Regex::xpath("a{2147483647}", "").unwrap();
    assert!(!regex.is_match("aaa"));
}

// Test a two-digit back reference
#[test]
fn test_cbcl_matches_039() {
    let regex = Regex::xpath(
        "(a)(b)(c)(d)(e)(f)(g)(h)(i)\\1\\2\\3\\4\\5\\6\\7\\8\\9\\10\\11",
        "",
    )
    .unwrap();
    assert!(regex.is_match("abcdefghiabcdefghia0a1"));
}

// Test the multi-character escape \S
#[test]
fn test_cbcl_matches_040() {
    let regex = Regex::xpath("\\S+", "").unwrap();
    assert!(regex.is_match("abc"));
}

// Test the multi-character escape \S
#[test]
fn test_cbcl_matches_041() {
    let regex = Regex::xpath("\\S+", "").unwrap();
    assert!(!regex.is_match("\r \t"));
}

// Test the multi-character escape \i
#[test]
fn test_cbcl_matches_042() {
    let regex = Regex::xpath("\\i+", "").unwrap();
    assert!(regex.is_match("a_:"));
}

// Test the multi-character escape \i
#[test]
fn test_cbcl_matches_043() {
    let regex = Regex::xpath("\\i+", "").unwrap();
    assert!(!regex.is_match("1.0"));
}

// Test the multi-character escape \I
#[test]
fn test_cbcl_matches_044() {
    let regex = Regex::xpath("\\I+", "").unwrap();
    assert!(regex.is_match("1.0"));
}

// Test the multi-character escape \I
#[test]
fn test_cbcl_matches_045() {
    let regex = Regex::xpath("\\I+", "").unwrap();
    assert!(!regex.is_match("a_:"));
}

// Test the multi-character escape \c
#[test]
fn test_cbcl_matches_046() {
    let regex = Regex::xpath("\\c+", "").unwrap();
    assert!(regex.is_match("abc"));
}

// Test the multi-character escape \c
#[test]
fn test_cbcl_matches_047() {
    let regex = Regex::xpath("\\c+", "").unwrap();
    assert!(!regex.is_match(" \t\r"));
}

// Test the multi-character escape \C
#[test]
fn test_cbcl_matches_048() {
    let regex = Regex::xpath("\\C+", "").unwrap();
    assert!(regex.is_match(" \t\r"));
}

// Test the multi-character escape \C
#[test]
fn test_cbcl_matches_049() {
    let regex = Regex::xpath("\\C+", "").unwrap();
    assert!(!regex.is_match("abc"));
}

// A back-reference is compared using case-blind comparison: that is, each
// character must either be the same as the corresponding character of the
// previously matched string, or must be a case-variant of that character. the
// back reference. For example, the strings "Mum", "mom", "Dad", and "DUD" all
// match the regular expression "([md])[aeiou]\1" when the "i" flag is used.
#[test]
fn test_cbcl_matches_050() {
    let regex = Regex::xpath("([md])[aeiou]\\1", "i").unwrap();
    assert!(regex.is_match("Mum"));
}

// Test back-reference to character above &#xFFFF;
#[test]
fn test_cbcl_matches_051() {
    let regex = Regex::xpath("(\u{10000})\\1", "").unwrap();
    assert!(regex.is_match("\u{10000}\u{10000}"));
}

// Test back-reference to character above &#xFFFF;
#[test]
fn test_cbcl_matches_052() {
    let regex = Regex::xpath("(\u{10000})\\1", "").unwrap();
    assert!(!regex.is_match("\u{10000}\u{10001}"));
}

// A back-reference is compared using case-blind comparison: that is, each
// character must either be the same as the corresponding character of the
// previously matched string, or must be a case-variant of that character. the
// back reference. For example, the strings "Mum", "mom", "Dad", and "DUD" all
// match the regular expression "([md])[aeiou]\1" when the "i" flag is used.
#[test]
fn test_cbcl_matches_053() {
    let regex = Regex::xpath("([md])[aeiou]\\1", "i").unwrap();
    assert!(!regex.is_match("Mud"));
}
