use regexml::{Error, Regex};

#[test]
fn test_matches2args_1() {
    let regex = Regex::xpath("This is a characte", "").unwrap();
    assert!(regex.is_match("This is a characte"));
}

#[test]
fn test_matches_err_1() {
    let regex = Regex::xpath("bra", "p");
    assert!(regex.is_err());
    assert_eq!(
        regex.unwrap_err(),
        Error::InvalidFlags("Unrecognized flag 'p'".to_string())
    );
}
