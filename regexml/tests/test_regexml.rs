use regexml::Regex;

#[test]
fn test_is_match_simple() {
    let regex = Regex::xpath("hello", "").unwrap();
    assert!(regex.is_match("hello"));
    assert!(!regex.is_match("world"));
}

#[test]
fn test_is_match_question_mark() {
    let regex = Regex::xpath("a?", "").unwrap();
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
}

#[test]
fn test_is_match_question_mark_complete() {
    let regex = Regex::xpath("^a?$", "").unwrap();
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(!regex.is_match("aa"));
    assert!(!regex.is_match("b"));
}

#[test]
fn test_is_match_star() {
    let regex = Regex::xpath("a*", "").unwrap();
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
    assert!(regex.is_match("aaa"));
}

#[test]
fn test_is_match_star_complete() {
    let regex = Regex::xpath("^a*$", "").unwrap();
    assert!(!regex.is_match("b"));
    assert!(regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
    assert!(regex.is_match("aaa"));
}

#[test]
fn test_is_match_plus() {
    let regex = Regex::xpath("a+", "").unwrap();
    assert!(!regex.is_match(""));
    assert!(regex.is_match("a"));
    assert!(regex.is_match("aa"));
    assert!(regex.is_match("aaa"));
}

#[test]
fn test_mixed() {
    let regex = Regex::xpath("^a?b+c*$", "").unwrap();
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
    let regex = Regex::xpath("^a?b+c*$", "").unwrap();
    assert!(regex.is_match("ab"));
}

// these tests are a sampling from qt3/fn/matches.xml to ensure
// that at least some basics work, and to help debug it when not.

#[test]
fn test_matches_1() {
    let regex = Regex::xpath("bra", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_matches_2() {
    let regex = Regex::xpath("^a.*a$", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_matches_3() {
    let regex = Regex::xpath("^bra", "").unwrap();
    assert!(!regex.is_match("abracadabra"));
}

#[test]
fn test_matches_6() {
    let regex = Regex::xpath("\\^", "").unwrap();
    assert!(regex.is_match("abracadabra^abracadabra"));
}

#[test]
fn test_matches_20() {
    let regex = Regex::xpath("\t", "").unwrap();
    assert!(regex.is_match("abracadbra\tabracadabra"));
}

#[test]
fn test_matches_22() {
    let regex = Regex::xpath("aa{1}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabra"));
}

#[test]
fn test_matches_23() {
    let regex = Regex::xpath("aa{1,}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabraabracadabra"));
}

#[test]
fn test_matches_30() {
    let regex = Regex::xpath("(?:abra(?:cad)?)*", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_matches_52() {
    let regex = Regex::xpath("^(a*b?a*){3,3}$", "").unwrap();
    assert!(regex.is_match("aaababaaabaa"))
}

#[test]
fn test_matches_character_range() {
    let regex = Regex::xpath("[A-Z]", "").unwrap();
    assert!(regex.is_match("A"))
}

#[test]
fn test_matches_53() {
    let regex = Regex::xpath("([A-Z])\\1*", "").unwrap();
    assert!(regex.is_match("A"))
}

#[test]
fn test_simple_replace() {
    let regex = Regex::xpath("hello", "").unwrap();

    assert_eq!(
        regex.replace_all("hello world", "bye").unwrap(),
        "bye world"
    )
}

#[test]
fn test_replace_48() {
    let regex = Regex::xpath("^a(.).$|^a...$", "").unwrap();

    assert_eq!(regex.replace_all("abcd", "$1").unwrap(), "")
}

#[test]
fn test_tokenize_9() {
    let regex = Regex::xpath("(ab)|(a)", "").unwrap();
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

#[test]
fn test_tokenize_10() {
    let regex = Regex::xpath("ww", "").unwrap();
    assert_eq!(
        regex.tokenize("abracadabra").unwrap().collect::<Vec<_>>(),
        vec!["abracadabra".to_string(),]
    );
}

#[test]
fn test_tokenize_11() {
    let regex = Regex::xpath("^a", "").unwrap();
    assert_eq!(
        regex.tokenize("abracadabra").unwrap().collect::<Vec<_>>(),
        vec!["".to_string(), "bracadabra".to_string()]
    );
}

#[test]
fn test_caselessmatch01() {
    let regex = Regex::xpath("ABC", "i").unwrap();
    assert!(regex.is_match("abc"));
}

#[test]
fn test_caselessmatch01_inverted() {
    let regex = Regex::xpath("abc", "i").unwrap();
    assert!(regex.is_match("ABC"));
}

#[test]
fn test_caseless_match_character_classes() {
    let regex = Regex::xpath("[A-Z]", "i").unwrap();
    assert!(regex.is_match("a"));
}

#[test]
fn test_matches_multiline() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(!regex.is_match("abcd\ndefg\n"));
}

#[test]
fn test_position_assumption() {
    let a = [0, 1, 2, 3, 4, 5];
    // find the index of first matching element while skipping the first two
    a.iter().enumerate().skip(2).find(|(_, &x)| x == 3);
}

#[test]
fn test_matches_31() {
    let regex = Regex::xpath("(?:abra(?:cad)?)*", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

#[test]
fn test_regex_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Regex>();
}

#[test]
fn test_category_escapes_after_their_sets_are_cached() {
    // the first compilation of each escape builds its set, the later ones
    // start from the cached set; complementing or subtracting it in one
    // expression must not affect another
    for _ in 0..2 {
        let regex = Regex::xpath(r"^\P{Lu}+$", "").unwrap();
        assert!(regex.is_match("abc1"));
        assert!(!regex.is_match("aBc"));

        let regex = Regex::xpath(r"^[\p{L}-[\p{Lu}]]+$", "").unwrap();
        assert!(regex.is_match("abcé"));
        assert!(!regex.is_match("abÉ"));

        let regex = Regex::xpath(r"^\p{Lu}+$", "").unwrap();
        assert!(regex.is_match("ABÉ"));
        assert!(!regex.is_match("Ab"));

        let regex = Regex::xsd(r"\d{3}-\w+", "").unwrap();
        assert!(regex.is_match("123-abc"));
        assert!(regex.is_match("١٢٣-abc"));
        assert!(!regex.is_match("123-!"));

        let regex = Regex::xpath(r"^[\W\D]+$", "").unwrap();
        assert!(regex.is_match("a!"));

        let regex = Regex::xpath(r"^\W+$", "").unwrap();
        assert!(regex.is_match("!? "));
        assert!(!regex.is_match("!a"));
    }
}

#[test]
fn test_category_escapes_compiled_on_several_threads() {
    let threads: Vec<_> = (0..8)
        .map(|i| {
            std::thread::spawn(move || {
                let pattern = if i % 2 == 0 {
                    r"^\p{Lu}\w*\d$"
                } else {
                    r"^[\p{L}-[\p{Ll}]]\w*\d$"
                };
                let regex = Regex::xpath(pattern, "").unwrap();
                regex.is_match("Abc1") && !regex.is_match("abc1")
            })
        })
        .collect();
    for thread in threads {
        assert!(thread.join().unwrap());
    }
}
