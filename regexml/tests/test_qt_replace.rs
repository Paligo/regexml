// these test cases are a transliteration of test cases in replace.xml
// part of the qt3 test suite

use regexml::{Error, Regex};

// Evaluation of replace function with replacement = "*" as an example 1 for
// this function.
#[test]
fn test_fn_replace_1() {
    let regex = Regex::xpath("bra", "").unwrap();
    let result = regex.replace_all("abracadabra", "*").unwrap();
    assert_eq!(result, "a*cada*");
}

// Evaluation of replace function with pattern = "a.*a" and replacement = "*"
// as an example 2 for this function.
#[test]
fn test_fn_replace_2() {
    let regex = Regex::xpath("a.*a", "").unwrap();
    let result = regex.replace_all("abracadabra", "*").unwrap();
    assert_eq!(result, "*");
}

// Evaluation of replace function with pattern = "a.*?a" and replacement = "*"
// as an example 3 for this function.
#[test]
fn test_fn_replace_3() {
    let regex = Regex::xpath("a.*?a", "").unwrap();
    let result = regex.replace_all("abracadabra", "*").unwrap();
    assert_eq!(result, "*c*bra");
}

// Evaluation of replace function with pattern = "a" and replacement = "" as an
// example 4 for this function.
#[test]
fn test_fn_replace_4() {
    let regex = Regex::xpath("a", "").unwrap();
    let result = regex.replace_all("abracadabra", "").unwrap();
    assert_eq!(result, "brcdbr");
}

// Evaluation of replace function with pattern = "a(.)" and replacement = "a$1$1"
// as an example 5 for this function.
#[test]
fn test_fn_replace_5() {
    let regex = Regex::xpath("a(.)", "").unwrap();
    let result = regex.replace_all("abracadabra", "a$1$1").unwrap();
    assert_eq!(result, "abbraccaddabbra");
}

// fn-replace-6 is not translated, as it checks that a replace of an empty
// string is an error. We can do this in the integration layer with xpath.

// Evaluation of replace function with pattern = "A+" and replacement = "b" as
// an example 7 for this function.
#[test]
fn test_fn_replace_7() {
    let regex = Regex::xpath("A+", "").unwrap();
    let result = regex.replace_all("AAAA", "b").unwrap();
    assert_eq!(result, "b");
}

// Evaluation of replace function with pattern = "A+?" and replacement = "b" as
// an example 8 for this function.
#[test]
fn test_fn_replace_8() {
    let regex = Regex::xpath("A+?", "").unwrap();
    let result = regex.replace_all("AAAA", "b").unwrap();
    assert_eq!(result, "bbbb");
}

// Evaluation of replace function with pattern = "^(.*?)d(.*)" and replacement =
// "$1c$2" as an example 9 for this function.
#[test]
fn test_fn_replace_9() {
    let regex = Regex::xpath("^(.*?)d(.*)$", "").unwrap();
    let result = regex.replace_all("darted", "$1c$2").unwrap();
    assert_eq!(result, "carted");
}

// Evaluation of replace function with pattern = "(ab)|(a)" and replacement =
// "[1=$1][2=$2]" as an example 10 for this function.
#[test]
fn test_fn_replace_10() {
    let regex = Regex::xpath("(ab)|(a)", "").unwrap();
    let result = regex.replace_all("abcd", "[1=$1][2=$2]").unwrap();
    assert_eq!(result, "[1=ab][2=]cd");
}

// Evaluation of replace function with input set to empty sequence.
#[test]
fn test_fn_replace_11() {
    let regex = Regex::xpath("bra", "").unwrap();
    let result = regex.replace_all("", "*").unwrap();
    assert_eq!(result, "");
}

// Evaluation of replace function with pattern set to "\?" for an input string
// that contains "?".
#[test]
fn test_fn_replace_13() {
    let regex = Regex::xpath("\\?", "").unwrap();
    let result = regex
        .replace_all("abracadabra?abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\*" for an input string
// that contains "*".
#[test]
fn test_fn_replace_14() {
    let regex = Regex::xpath("\\*", "").unwrap();
    let result = regex
        .replace_all("abracadabra*abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\+" for an input string
// that contains "+".
#[test]
fn test_fn_replace_15() {
    let regex = Regex::xpath("\\+", "").unwrap();
    let result = regex
        .replace_all("abracadabra+abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\{" for an input string
// that contains "}".
#[test]
fn test_fn_replace_16() {
    let regex = Regex::xpath("\\{", "").unwrap();
    let result = regex
        .replace_all("abracadabra{abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\}" for an input string
// that contains "}".
#[test]
fn test_fn_replace_17() {
    let regex = Regex::xpath("\\}", "").unwrap();
    let result = regex
        .replace_all("abracadabra}abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\(" for an input string
// that contains "(".
#[test]
fn test_fn_replace_18() {
    let regex = Regex::xpath("\\(", "").unwrap();
    let result = regex
        .replace_all("abracadabra(abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\)" for an input string
// that contains ")".
#[test]
fn test_fn_replace_19() {
    let regex = Regex::xpath("\\)", "").unwrap();
    let result = regex
        .replace_all("abracadabra)abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\[" for an input string
// that contains "[".
#[test]
fn test_fn_replace_20() {
    let regex = Regex::xpath("\\[", "").unwrap();
    let result = regex
        .replace_all("abracadabra[abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\]" for an input string
// that contains "]".
#[test]
fn test_fn_replace_21() {
    let regex = Regex::xpath("\\]", "").unwrap();
    let result = regex
        .replace_all("abracadabra]abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\-" for an input string
// that contains "-".
#[test]
fn test_fn_replace_22() {
    let regex = Regex::xpath("\\-", "").unwrap();
    let result = regex
        .replace_all("abracadabra-abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\." for an input string
// that contains ".".
#[test]
fn test_fn_replace_23() {
    let regex = Regex::xpath("\\.", "").unwrap();
    let result = regex
        .replace_all("abracadabra.abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\|" for an input string
// that contains "|".
#[test]
fn test_fn_replace_24() {
    let regex = Regex::xpath("\\|", "").unwrap();
    let result = regex
        .replace_all("abracadabra|abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\\" for an input string
// that contains "\".
#[test]
fn test_fn_replace_25() {
    let regex = Regex::xpath("\\\\", "").unwrap();
    let result = regex
        .replace_all("abracadabra\\abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\t" for an input string
// that contains the tab character.
#[test]
fn test_fn_replace_26() {
    let regex = Regex::xpath("\\t", "").unwrap();
    let result = regex
        .replace_all("abracadabra\tabracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "\n" for an input string
// that contains the newline character.
#[test]
fn test_fn_replace_27() {
    let regex = Regex::xpath("\\n", "").unwrap();
    let result = regex
        .replace_all("abracadabra\nabracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "aa{1}" (exact quantity)
// for an input string that contains the "aa" string.
#[test]
fn test_fn_replace_28() {
    let regex = Regex::xpath("aa{1}", "").unwrap();
    let result = regex.replace_all("abracadabraabracadabra", "with").unwrap();
    assert_eq!(result, "abracadabrwithbracadabra");
}

// Evaluation of replace function with pattern set to "aa{1,}" (min quantity)
// for an input string that contains the "aa" string twice.
#[test]
fn test_fn_replace_29() {
    let regex = Regex::xpath("aa{1,}", "").unwrap();
    let result = regex
        .replace_all("abracadabraabracadabraabracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrwithbracadabrwithbracadabra");
}

// Evaluation of replace function with pattern set to "aa{1,2}" (range quantity)
// for an input string that contains the "aa" string twice.
#[test]
fn test_fn_replace_30() {
    let regex = Regex::xpath("aa{1,2}", "").unwrap();
    let result = regex
        .replace_all("abracadabraabracadabraabracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrwithbracadabrwithbracadabra");
}

// Evaluation of replace function with pattern set to "\^".
#[test]
fn test_fn_replace_31() {
    let regex = Regex::xpath("\\^", "").unwrap();
    let result = regex
        .replace_all("abracadabra^abracadabra", "with")
        .unwrap();
    assert_eq!(result, "abracadabrawithabracadabra");
}

// Evaluation of replace function with pattern set to "^a".
#[test]
fn test_fn_replace_32() {
    let regex = Regex::xpath("^a", "").unwrap();
    let result = regex.replace_all("abracadabra", "with").unwrap();
    assert_eq!(result, "withbracadabra");
}

// Evaluation of replace function with pattern that does not match the input
// string.
#[test]
fn test_fn_replace_33() {
    let regex = Regex::xpath("ww", "").unwrap();
    let result = regex.replace_all("abracadabra", "with").unwrap();
    assert_eq!(result, "abracadabra");
}

// Evaluation of replace function with q flag.
#[test]
fn test_fn_replace_34() {
    let regex = Regex::xpath("a", "q").unwrap();
    let result = regex.replace_all("abracadabra", "$1").unwrap();
    assert_eq!(result, "$1br$1c$1d$1br$1");
}

// Evaluation of replace function with q flag.
#[test]
fn test_fn_replace_35() {
    let regex = Regex::xpath("a?", "q").unwrap();
    let result = regex.replace_all("a?bracadabra?", "\\$1").unwrap();
    assert_eq!(result, "\\$1bracadabr\\$1");
}

// Evaluation of replace function with non-capturing groups.
#[test]
fn test_fn_replace_36() {
    let regex = Regex::xpath("([aA])(?:br)([aA])", "").unwrap();
    let result = regex.replace_all("abracadabra", "$1**$2").unwrap();
    assert_eq!(result, "a**acada**a");
}

// Evaluation of replace function with escaped $ sign in replacement string.
#[test]
fn test_fn_replace_37() {
    let regex = Regex::xpath("a", "").unwrap();
    let result = regex.replace_all("abracadabra", "\\$").unwrap();
    assert_eq!(result, "$br$c$d$br$");
}

// Evaluation of replace function with escaped $ sign in replacement string.
#[test]
fn test_fn_replace_38() {
    let regex = Regex::xpath("(a)", "").unwrap();
    let result = regex.replace_all("abracadabra", "\\$$1").unwrap();
    assert_eq!(result, "$abr$ac$ad$abr$a");
}

// Evaluation of replace function with escaped \ sign in replacement string.
#[test]
fn test_fn_replace_39() {
    let regex = Regex::xpath("a", "").unwrap();
    let result = regex.replace_all("abracadabra", "\\\\").unwrap();
    assert_eq!(result, "\\br\\c\\d\\br\\");
}

// Evaluation of replace with double-digit capture
#[test]
fn test_fn_replace_40() {
    let regex = Regex::xpath("((((( ((((( (((((a))))) ))))) )))))", "x").unwrap();
    let result = regex.replace_all("abracadabra", "|$1$15|").unwrap();
    assert_eq!(result, "|aa|br|aa|c|aa|d|aa|br|aa|");
}

// Evaluation of replace with double-digit capture
#[test]
fn test_fn_replace_41() {
    let regex = Regex::xpath("((((( ((((( (((((a))))) ))))) )))))", "x").unwrap();
    let result = regex.replace_all("abracadabra", "$1520").unwrap();
    assert_eq!(result, "a20bra20ca20da20bra20");
}

// Evaluation of replace with double-digit capture beyond max capture value
#[test]
fn test_fn_replace_42() {
    let regex = Regex::xpath("((((( ((((( (((((a)(b))))) ))))) )))))", "x").unwrap();
    let result = regex
        .replace_all("abracadabra", "($14.$15.$16.$17)")
        .unwrap();
    assert_eq!(result, "(ab.a.b.ab7)racad(ab.a.b.ab7)ra");
}
// "." does NOT match CR in default mode
#[test]
fn test_fn_replace_43() {
    let regex = Regex::xpath("Mary.Jones", "").unwrap();
    let result = regex.replace_all("Mary\rJones", "Jacob Jones").unwrap();
    assert_eq!(result, "Mary\rJones");
}

// "." does match CR in dot-all mode
#[test]
fn test_fn_replace_44() {
    let regex = Regex::xpath("Mary.Jones", "s").unwrap();
    let result = regex.replace_all("Mary\rJones", "Jacob Jones").unwrap();
    assert_eq!(result, "Jacob Jones");
}

// Evaluation of replace, using $0
#[test]
fn test_fn_replace_45() {
    let regex = Regex::xpath("[A-Z][A-Z]+", "").unwrap();
    let result = regex
        .replace_all("Now, let's SEND OUT for QUICHE!!", "$0$0")
        .unwrap();
    assert_eq!(result, "Now, let's SENDSEND OUTOUT for QUICHEQUICHE!!");
}

// Evaluation of replace, Saxon bug 2166
#[test]
fn test_fn_replace_46() {
    let regex = Regex::xpath(r"^\d+(-(\d+))?$", "").unwrap();
    let result = regex.replace_all("12-34", "$2").unwrap();
    assert_eq!(result, "34");
}

// Evaluation of replace, Saxon bug 2390 (reluctant quantifiers, backtracking, and captured groups)
#[test]
fn test_fn_replace_47() {
    let regex = Regex::xpath(r"^.+?(b+)?$", "").unwrap();
    let result = regex.replace_all("abc", "$1").unwrap();
    assert_eq!(result, "");
}

// Evaluation of replace, Saxon bug 2390 (choice, backtracking, and captured groups)
#[test]
fn test_fn_replace_48() {
    let regex = Regex::xpath(r"^a(.).$|^a...$", "").unwrap();
    let result = regex.replace_all("abcd", "$1").unwrap();
    assert_eq!(result, "");
}

// Evaluation of replace function with q flag, with pattern set to "\".
#[test]
fn test_fn_replace_49() {
    let regex = Regex::xpath("/", "q").unwrap();
    let result = regex.replace_all("a/b/c", "\\").unwrap();
    assert_eq!(result, "a\\b\\c");
}

// Evaluation of replace function with q flag, with pattern set to "\\", example
// from spec. (See bug #29522)
#[test]
fn test_fn_replace_50() {
    let regex = Regex::xpath("\\", "q").unwrap();
    let result = regex.replace_all("a\\b\\c", "\\\\").unwrap();
    assert_eq!(result, "a\\\\b\\\\c");
}

// Evaluation of replace function with q flag, with pattern set to "$", example from
// spec. (See bug #29522)
#[test]
fn test_fn_replace_51() {
    let regex = Regex::xpath("/", "q").unwrap();
    let result = regex.replace_all("a/b/c", "$").unwrap();
    assert_eq!(result, "a$b$c");
}

// Evaluation of replace function with q flag, with pattern set to "$'".
#[test]
fn test_fn_replace_52() {
    let regex = Regex::xpath("/", "q").unwrap();
    let result = regex.replace_all("a/b/c", "$'").unwrap();
    assert_eq!(result, "a$'b$'c");
}

// Evaluation of replace function with q flag, with pattern set to "$`". (See bug #29522)
#[test]
fn test_fn_replace_53() {
    let regex = Regex::xpath("/", "q").unwrap();
    let result = regex.replace_all("a/b/c", "$`").unwrap();
    assert_eq!(result, "a$`b$`c");
}

// Evaluation of replace with single-digit capture at end of replacement string. Saxon bug 2735.
#[test]
fn test_fn_replace_54() {
    let regex = Regex::xpath("((((( ((((( (((((a))))) ))))) )))))", "x").unwrap();
    let result = regex.replace_all("abracadabra", "$1$1").unwrap();
    assert_eq!(result, "aabraacaadaabraa");
}

// Test case with undefined captures. Saxon bug 2865.
#[test]
fn test_fn_replace_55() {
    let regex = Regex::xpath("(a)|(b)|(c)|(d)|(e)|(f)|(g)|(h)|(i)|(j)", "").unwrap();
    let result = regex.replace_all("abcdefghijk", "$1").unwrap();
    assert_eq!(result, "ak");
}

// Test case based on Saxon bug 3429.
#[test]
fn test_fn_replace_56() {
    let regex = Regex::xpath(r"^\d*\.?\d+", "").unwrap();
    let result = regex.replace_all("10%", "").unwrap();
    assert_eq!(result, "%");
}

// Evaluation of replace with unmatched double-digit capturing group. Saxon bug 4076.
#[test]
fn test_fn_replace_57() {
    let regex = Regex::xpath("^(9)(8)(7)(6)(5)(4)(3)(2)(1)((A*?)|(.+))$", "").unwrap();
    let result = regex.replace_all("987654321A", "$9$11$12").unwrap();
    assert_eq!(result, "1A");
}

// Evaluation of replace with emoji. Saxon bug 5174.
#[test]
fn test_fn_replace_58() {
    let regex = Regex::xpath(r"\p{IsEmoticons}+", "").unwrap();
    let result = regex.replace_all("Hello 😀😃😄🙆 üäö$", "").unwrap();
    assert_eq!(result, "Hello  üäö$");
}

#[test]
fn test_fn_replace_single_backslash() {
    let regex = Regex::xpath("a", "").unwrap();
    let err = regex.replace_all("abracadabra", "\\").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString("Invalid escape at end of replacement string".to_string())
    );
}

// The flags argument cannot contain whitespace.
#[test]
fn test_k_replacefunc_1() {
    let err = Regex::xpath("input", " ").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidFlags("Unrecognized flag ' '".to_string())
    );
}

// The flags argument cannot contain 'X'.
#[test]
fn test_k_replacefunc_2() {
    let err = Regex::xpath("input", "X").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidFlags("Unrecognized flag 'X'".to_string())
    );
}

// A '\' cannot occur at the end of the line.
#[test]
fn test_k_replacefunc_6() {
    let regex = Regex::xpath("in", "").unwrap();
    let err = regex.replace_all("input", "thisIsInvalid\\").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString("Invalid escape at end of replacement string".to_string())
    );
}

// A '$' cannot occur at the end of the line.
#[test]
fn test_k_replacefunc_7() {
    let regex = Regex::xpath("(input)", "").unwrap();
    let err = regex.replace_all("input", "thisIsInvalid$").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString("Invalid escape at end of replacement string".to_string())
    );
}

// A '\' cannot be used to escape whitespace.
#[test]
fn test_k_replacefunc_8() {
    let regex = Regex::xpath("in", "").unwrap();
    let err = regex.replace_all("input", "thisIsInvalid\\ ").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString("Invalid escape ' ' in replacement string".to_string())
    );
}

// A '$' cannot be followed by whitespace.
#[test]
fn test_k_replacefunc_9() {
    let regex = Regex::xpath("in", "").unwrap();
    let err = regex.replace_all("input", "thisIsInvalid$ ").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString(
            "$ in replacement string must be followed by a digit".to_string()
        )
    );
}

// Unexpectedly ending escape.
#[test]
fn test_k_replacefunc_10() {
    let regex = Regex::xpath("(a )", "").unwrap();
    let err = regex.replace_all("a a a ", "replacment: \\1").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString("Invalid escape '1' in replacement string".to_string())
    );
}

// Unexpected ending escape.
#[test]
fn test_k2_replacefunc_1() {
    let regex = Regex::xpath("(a )", "").unwrap();
    let err = regex.replace_all("a a a ", "replacment: \\1").unwrap_err();
    assert_eq!(
        err,
        Error::InvalidReplacementString("Invalid escape '1' in replacement string".to_string())
    );
}

// Use a back reference inside a character class.
#[test]
fn test_k2_replacefunc_4() {
    let err = Regex::xpath("(asd)[\\1]", "").unwrap_err();
    assert_eq!(
        err,
        Error::Syntax("Backreferences not allowed within character classes".to_string())
    );
}

// Use a back reference inside a character class(#2).
#[test]
fn test_k2_replacefunc_5() {
    let err = Regex::xpath("(asd)[asd\\1]", "").unwrap_err();
    assert_eq!(
        err,
        Error::Syntax("Backreferences not allowed within character classes".to_string())
    );
}

// Use a back reference inside a character class(#3).
#[test]
fn test_k2_replacefunc_6() {
    let err = Regex::xpath("(asd)[asd\\0]", "").unwrap_err();
    assert_eq!(
        err,
        Error::Syntax("Octal escapes are not allowed".to_string())
    );
}

// Use a back reference inside a character class(#3).
#[test]
fn test_k2_replacefunc_7() {
    let err = Regex::xpath("1[asd\\0]", "").unwrap_err();
    assert_eq!(
        err,
        Error::Syntax("Octal escapes are not allowed".to_string())
    );
}
