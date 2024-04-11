use regexml::Regex;

#[test]
fn test_infinite_loop() {
    // {0,0} is really a no-op, but could lead to an infinite loop
    // in the FixedGreedy operation. Now we create a noop in such a situation.
    let regex = Regex::xpath(r#"(( a | ( bc ) ) {0,0} )+ xyz"#, "x");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"xyz"#));
}

// this won't trigger a backtrack, so choice is okay
#[test]
fn test_choice_without_postfix() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)+"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

// This should trigger a backtrack, but it seems to work
#[test]
fn test_backtrack_attempt() {
    let regex = Regex::xpath(r#"(WORDS)?(WORD)?S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

// This triggers a backtrack (as WORDSS won't match) but doesn't
// work yet

// #[test]
// fn test_choice_with_postfix() {
//     let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)+S"#, "");
//     let regex = regex.unwrap();
//     assert!(regex.is_match(r#"WORDS"#));
// }
