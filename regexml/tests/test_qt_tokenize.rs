// these test cases are a transliteration of test cases in tokenize.xml
// part of the qt3 test suite

use regexml::{Error, Regex};

// Evaluation of tokenize function where pattern matches the zero length
// string. Given on example 3 for this function in the Func and Ops specs.
#[test]
fn test_fn_tokenize_1() {
    let regex = Regex::xpath(".?", "").unwrap();
    let err = regex.tokenize("abba").unwrap_err();
    assert_eq!(err, Error::MatchesEmptyString);
}

// Evaluation of tokenize function whith an invalid value for the flags
#[test]
fn test_fn_tokenize_2() {
    let err = Regex::xpath(r"\s+", "t").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidFlags("Unrecognized flag 't'".to_string())
    );
}

// Evaluation of tokenize function with pattern set to "\s+" as per example 1
// for this functions from the Func and Ops specs.
#[test]
fn test_fn_tokenize_3() {
    let regex = Regex::xpath(r"\s+", "").unwrap();
    assert_eq!(
        regex
            .tokenize("The cat sat on the mat")
            .unwrap()
            .collect::<Vec<_>>(),
        vec![
            "The".to_string(),
            "cat".to_string(),
            "sat".to_string(),
            "on".to_string(),
            "the".to_string(),
            "mat".to_string(),
        ]
    );
}

// Evaluation of tokenize function with pattern set to "\s*" as per example 2
// for this functions from the Func and Ops specs.
#[test]
fn test_fn_tokenize_4() {
    let regex = Regex::xpath(r",\s*", "").unwrap();
    assert_eq!(
        regex.tokenize("1, 15, 24, 50").unwrap().collect::<Vec<_>>(),
        vec![
            "1".to_string(),
            "15".to_string(),
            "24".to_string(),
            "50".to_string(),
        ]
    );
}

// Evaluation of tokenize function with pattern set to "\s*<br>\s*" and flag
// set to "i" as per example 4 for this functions from the Func and Ops specs.
#[test]
fn test_fn_tokenize_5() {
    let regex = Regex::xpath(r"\s*<br>\s*", "i").unwrap();
    assert_eq!(
        regex
            .tokenize("Some unparsed <br> HTML <BR> text")
            .unwrap()
            .collect::<Vec<_>>(),
        vec![
            "Some unparsed".to_string(),
            "HTML".to_string(),
            "text".to_string(),
        ]
    );
}

// Evaluation of tokenize function with pattern with flags arguments set to
// empty string.
#[test]
fn test_fn_tokenize_6() {
    let regex = Regex::xpath(r"\s*<br>\s*", "").unwrap();
    assert_eq!(
        regex
            .tokenize("Some unparsed <br> HTML <BR> text")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["Some unparsed".to_string(), "HTML <BR> text".to_string(),]
    );
}

// Evaluation of tokenize function with $input set to zero length string. Uses
// fn:count to avoid empty file.
#[test]
fn test_fn_tokenize_8() {
    let regex = Regex::xpath(r"\s+", "").unwrap();
    assert_eq!(
        regex.tokenize("").unwrap().collect::<Vec<_>>(),
        Vec::<String>::new()
    );
}

// Evaluation of tokenize function with two patterms matching the input string.
#[test]
fn test_fn_tokenize_9() {
    let regex = Regex::xpath(r"(ab)|(a)", "").unwrap();
    assert_eq!(
        regex.tokenize("abracadabra").unwrap().collect::<Vec<_>>(),
        vec![
            "".to_string(),
            "r".to_string(),
            "c".to_string(),
            "d".to_string(),
            "r".to_string(),
            "".to_string(),
        ]
    );
}

// Evaluation of tokenize function with pattern that does not match the input string.
#[test]
fn test_fn_tokenize_10() {
    let regex = Regex::xpath("ww", "").unwrap();
    assert_eq!(
        regex.tokenize("abracadabra").unwrap().collect::<Vec<_>>(),
        vec!["abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "^a".
#[test]
fn test_fn_tokenize_11() {
    let regex = Regex::xpath("^a", "").unwrap();
    assert_eq!(
        regex.tokenize("abracadabra").unwrap().collect::<Vec<_>>(),
        vec!["".to_string(), "bracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\^".
#[test]
fn test_fn_tokenize_12() {
    let regex = Regex::xpath(r"\^", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra^abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\?" for an input string
// that contains "?".
#[test]
fn test_fn_tokenize_13() {
    let regex = Regex::xpath(r"\?", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra?abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\*" for an input string
#[test]
fn test_fn_tokenize_14() {
    let regex = Regex::xpath(r"\*", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra*abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\+" for an input string
// that contains "+".
#[test]
fn test_fn_tokenize_15() {
    let regex = Regex::xpath(r"\+", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra+abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\{" for an input string
// that contains "{".
#[test]
fn test_fn_tokenize_16() {
    let regex = Regex::xpath(r"\{", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra{abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\}" for an input string
// that contains "}".
#[test]
fn test_fn_tokenize_17() {
    let regex = Regex::xpath(r"\}", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra}abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}
// Evaluation of tokenize function with pattern set to "\(" for an input string
// that contains "(".
#[test]
fn test_fn_tokenize_18() {
    let regex = Regex::xpath(r"\(", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra(abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\)" for an input string
// that contains ")".
#[test]
fn test_fn_tokenize_19() {
    let regex = Regex::xpath(r"\)", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra)abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\[" for an input string
// that contains "[".
#[test]
fn test_fn_tokenize_20() {
    let regex = Regex::xpath(r"\[", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra[abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\]" for an input string
// that contains "]".
#[test]
fn test_fn_tokenize_21() {
    let regex = Regex::xpath(r"\]", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra]abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\-" for an input string
// that contains "-".
#[test]
fn test_fn_tokenize_22() {
    let regex = Regex::xpath(r"\-", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra-abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\." for an input string
// that contains ".".
#[test]
fn test_fn_tokenize_23() {
    let regex = Regex::xpath(r"\.", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra.abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\|" for an input string
// that contains "|".
#[test]
fn test_fn_tokenize_24() {
    let regex = Regex::xpath(r"\|", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra|abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\\" for an input string
// that contains "\".
#[test]
fn test_fn_tokenize_25() {
    let regex = Regex::xpath(r"\\", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra\\abracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\t" for an input string
// that contains the tab character.
#[test]
fn test_fn_tokenize_26() {
    let regex = Regex::xpath(r"\t", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra\tabracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "\n" for an input string
// that contains the newline character.
#[test]
fn test_fn_tokenize_27() {
    let regex = Regex::xpath(r"\n", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabra\nabracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabra".to_string(), "abracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "aa{1}" (exact quantity)
// for an input string that contains the "aa" string.
#[test]
fn test_fn_tokenize_28() {
    let regex = Regex::xpath("aa{1}", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabraabracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec!["abracadabr".to_string(), "bracadabra".to_string()]
    );
}

// Evaluation of tokenize function with pattern set to "aa{1,}" (min quantity)
// for an input string that contains the "aa" string twice.
#[test]
fn test_fn_tokenize_29() {
    let regex = Regex::xpath("aa{1,}", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabraabracadabraabracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec![
            "abracadabr".to_string(),
            "bracadabr".to_string(),
            "bracadabra".to_string(),
        ]
    );
}

// Evaluation of tokenize function with pattern set to "aa{1,2}" (range quantity)
// for an input string that contains the "aa" string twice.
#[test]
fn test_fn_tokenize_30() {
    let regex = Regex::xpath("aa{1,2}", "").unwrap();
    assert_eq!(
        regex
            .tokenize("abracadabraabracadabraabracadabra")
            .unwrap()
            .collect::<Vec<_>>(),
        vec![
            "abracadabr".to_string(),
            "bracadabr".to_string(),
            "bracadabra".to_string(),
        ]
    );
}

// Evaluation of tokenize function with regex 'q' flag.
#[test]
fn test_fn_tokenize_31() {
    let regex = Regex::xpath(".", "q").unwrap();
    assert_eq!(
        regex
            .tokenize("abc.def.gh.ijk")
            .unwrap()
            .collect::<Vec<_>>(),
        vec![
            "abc".to_string(),
            "def".to_string(),
            "gh".to_string(),
            "ijk".to_string(),
        ]
    );
}

// Evaluation of tokenize function with regex 'q' and "i" flags.
#[test]
fn test_fn_tokenize_32() {
    let regex = Regex::xpath("a.", "qi").unwrap();
    assert_eq!(
        regex
            .tokenize("A.BRA.CADA.BRA")
            .unwrap()
            .collect::<Vec<_>>(),
        vec![
            "".to_string(),
            "BR".to_string(),
            "CAD".to_string(),
            "BRA".to_string(),
        ]
    );
}

// Evaluation of tokenize function with non-capturing group in the regex.
#[test]
fn test_fn_tokenize_33() {
    let regex = Regex::xpath("A(?:B)", "").unwrap();
    assert_eq!(
        regex.tokenize("ABRACADABRA").unwrap().collect::<Vec<_>>(),
        vec!["".to_string(), "RACAD".to_string(), "RA".to_string()]
    );
}

// "." does NOT match CR in default mode
#[test]
fn test_fn_tokenize_34() {
    let regex = Regex::xpath("y.J", "").unwrap();
    assert_eq!(
        regex.tokenize("Mary\rJones").unwrap().collect::<Vec<_>>(),
        vec!["Mary\rJones".to_string()]
    );
}

// "." does match CR in dot-all mode
#[test]
fn test_fn_tokenize_35() {
    let regex = Regex::xpath("y.J", "s").unwrap();
    assert_eq!(
        regex.tokenize("Mary\rJones").unwrap().collect::<Vec<_>>(),
        vec!["Mar".to_string(), "ones".to_string()]
    );
}

// Regex must be one that does not match a zero-length string
#[test]
fn test_fn_tokenize_36() {
    let regex = Regex::xpath("^", "m").unwrap();
    let err = regex.tokenize("Mary\nJones").unwrap_err();
    assert_eq!(err, Error::MatchesEmptyString);
}

// Regex must be one that does not match a zero-length string
#[test]
fn test_fn_tokenize_37() {
    let regex = Regex::xpath("$", "m").unwrap();
    let err = regex.tokenize("Mary\nJones").unwrap_err();
    assert_eq!(err, Error::MatchesEmptyString);
}

// Regex must be one that does not match a zero-length string
#[test]
fn test_fn_tokenize_38() {
    let regex = Regex::xpath(r"^[\s]*$", "m").unwrap();
    let err = regex.tokenize("Mary\nJones").unwrap_err();
    assert_eq!(err, Error::MatchesEmptyString);
}
