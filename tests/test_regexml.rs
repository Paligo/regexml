use regexml::Regex;

#[test]
fn test_is_match_simple() {
    let regex = Regex::new("hello").unwrap();
    assert!(regex.is_match("hello"));
    assert!(!regex.is_match("world"));
}

#[test]
fn test_is_match_question_mark() {
    let regex = Regex::new("a?").unwrap();
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
}

#[test]
fn test_is_match_question_mark_complete() {
    let regex = Regex::new("^a?$").unwrap();
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(!regex.is_match("aa"));
    assert!(!regex.is_match("b"));
}

#[test]
fn test_is_match_star() {
    let regex = Regex::new("a*").unwrap();
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
    assert!(regex.is_match("aaa"));
}

#[test]
fn test_is_match_star_complete() {
    let regex = Regex::new("^a*$").unwrap();
    assert!(!regex.is_match("b"));
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
    assert!(regex.is_match("aaa"));
}

#[test]
fn test_is_match_plus() {
    let regex = Regex::new("a+").unwrap();
    assert!(!regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
    assert!(regex.is_match("aaa"));
}

#[test]
fn test_mixed() {
    let regex = Regex::new("^a?b+c*$").unwrap();
    assert!(regex.is_match("bc"));
    assert!(regex.is_match("abc"));
    assert!(regex.is_match("abbc"));
    assert!(regex.is_match("abbbcc"));
    assert!(regex.is_match("abbb"));
    assert!(regex.is_match("ab"));
    assert!(regex.is_match("b"));
    assert!(!regex.is_match("a"));
    assert!(!regex.is_match("c"));
    assert!(!regex.is_match("ac"));
}

#[test]
fn test_mixed_problem() {
    let regex = Regex::new("^a?b+c*$").unwrap();
    assert!(regex.is_match("ab"));
}

// these tests are a sampling from qt3/fn/matches.xml to ensure
// that at least some basics work, and to help debug it when not.

#[test]
fn test_matches_1() {
    let regex = Regex::new("bra").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_matches_2() {
    let regex = Regex::new("^a.*a$").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_matches_3() {
    let regex = Regex::new("^bra").unwrap();
    assert!(!regex.is_match("abracadabra"));
}

#[test]
fn test_matches_6() {
    let regex = Regex::new("\\^").unwrap();
    assert!(regex.is_match("abracadabra^abracadabra"));
}

#[test]
fn test_matches_20() {
    let regex = Regex::new("\t").unwrap();
    assert!(regex.is_match("abracadbra\tabracadabra"));
}

#[test]
fn test_matches_22() {
    let regex = Regex::new("aa{1}").unwrap();
    assert!(regex.is_match("abracadabraabracadabra"));
}

#[test]
fn test_matches_23() {
    let regex = Regex::new("aa{1,}").unwrap();
    assert!(regex.is_match("abracadabraabracadabraabracadabra"));
}

#[test]
fn test_matches_30() {
    let regex = Regex::new("(?:abra(?:cad)?)*").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_matches_52() {
    let regex = Regex::new("^(a*b?a*){3,3}$").unwrap();
    assert!(regex.is_match("aaababaaabaa"))
}

#[test]
fn test_matches_character_range() {
    let regex = Regex::new("[A-Z]").unwrap();
    assert!(regex.is_match("A"))
}

// #[test]
// fn test_matches_53() {
//     let regex = Regex::new("([A-Z]\\1*").unwrap();
//     assert!(regex.is_match("A"))
// }
