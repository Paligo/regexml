use regexml::Regex;

#[test]
fn test_is_match_simple() {
    let regex = Regex::new("hello").unwrap();
    assert!(regex.is_match("hello"));
    assert!(!regex.is_match("world"));
}

// #[test]
// fn test_is_match_optional() {
//     let regex = Regex::new("a?").unwrap();
//     assert!(regex.is_match(""));
//     assert!(regex.is_match("a"));
//     assert!(regex.is_match("aa"));
//     assert!(!regex.is_match("b"));
// }

// #[test]
// fn test_is_match_star() {
//     let regex = Regex::new("a*").unwrap();
//     assert!(regex.is_match(""));
//     assert!(regex.is_match("a"));
//     assert!(regex.is_match("aa"));
//     assert!(regex.is_match("aaa"));
//     assert!(!regex.is_match("b"));
// }
