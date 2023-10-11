use regexml::Regex;

#[test]
fn test_is_match_simple() {
    let regex = Regex::new("hello").unwrap();
    assert!(regex.is_match("hello"));
    assert!(!regex.is_match("world"));
}
