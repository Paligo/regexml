// #[test]
// fn test_choice_with_postfix() {
//     let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)+S"#, "");
//     let regex = regex.unwrap();
//     assert!(regex.is_match(r#"WORDS"#));
// }

use regexml::Regex;

#[test]
fn test_infinite_loop() {
    let regex = Regex::xpath(r#"(( a | ( bc ) ) {0,0} )+ xyz"#, "x");
    // y
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"xyz"#));
}

#[test]
fn test_choice_without_postfix() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)+"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

#[test]
fn test_backtrack_attempt() {
    let regex = Regex::xpath(r#"(WORDS)?(WORD)?S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}
