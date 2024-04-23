use regexml::Regex;

// these contains some problem cases related to backtracking

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

// This should trigger a backtrack, but isn't a problem
#[test]
fn test_choice_no_repeat() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

// this also works, even though it should trigger a backtrack and has a capture
// group
#[test]
fn test_choice_star() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)*S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

#[test]
fn test_choice_question() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)?S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

#[test]
fn test_plus_inside_capture_group() {
    let regex = Regex::xpath(r#"^(.+)B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_star_inside_capture_group() {
    let regex = Regex::xpath(r#"^(.*)B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_star_outside_capture_group() {
    let regex = Regex::xpath(r#"^(.)*B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_star_inside_and_outside_capture_group() {
    let regex = Regex::xpath(r#"^(.?)*B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_whatever() {
    let regex = Regex::xpath(r#"(AS|A)+S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AS"#));
}

#[test]
fn test_plus_inside_and_star_inside_capture_group() {
    let regex = Regex::xpath(r#"^(.*)+B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_question_mark_outside_capture_group() {
    let regex = Regex::xpath(r#"^(.*)?B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_choice_plus() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)+S"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"WORDS"#));
}

#[test]
fn test_another_backtrack() {
    let regex = Regex::xpath(r#"^(.+)?B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_backtrack_no_nesting() {
    let regex = Regex::xpath(r#"^([AB]+)?B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}

#[test]
fn test_simple_capture() {
    let regex = Regex::xpath(r#"^(.+)?B"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"AB"#));
}
