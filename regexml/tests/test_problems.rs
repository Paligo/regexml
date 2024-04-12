use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_properties::{sets, GeneralCategoryGroup};
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
//     dbg!(&regex);
//     assert!(regex.is_match(r#"WORDS"#));
// }

#[test]
fn test_choice_without_plus() {
    let regex = Regex::xpath(r#"(WORDS|WORLD|WORD)S"#, "");
    let regex = regex.unwrap();
    dbg!(&regex);
    assert!(regex.is_match(r#"WORDS"#));
}

// #[test]
// fn test_another_backtrack() {
//     let regex = Regex::xpath(r#"^(.+)?B"#, "");
//     let regex = regex.unwrap();
//     dbg!(&regex);
//     assert!(regex.is_match(r#"AB"#));
// }

// #[test]
// fn test_backtrack_no_nesting() {
//     let regex = Regex::xpath(r#"^([AB]+)?B"#, "");
//     let regex = regex.unwrap();
//     dbg!(&regex);
//     assert!(regex.is_match(r#"AB"#));
// }

// #[test]
// fn test_simple_capture() {
//     let regex = Regex::xpath(r#"^(.+)?B"#, "");
//     let regex = regex.unwrap();
//     dbg!(&regex);
//     assert!(regex.is_match(r#"AB"#));
// }

// #[test]
// fn test_another_backtrack_without_question() {
//     let regex = Regex::xpath(r#"^(.+)B"#, "");
//     let regex = regex.unwrap();
//     assert!(regex.is_match(r#"AB"#));
// }

#[test]
fn test_l_category_group() {
    let c = '㐀';

    let set = sets::for_general_category_group(GeneralCategoryGroup::Letter);
    let inv_list = set.to_code_point_inversion_list();
    let mut builder = CodePointInversionListBuilder::new();
    builder.add_set(&inv_list);
    let b = builder.build();
    assert!(b.contains(c));
}

#[test]
fn test_l_category_membership() {
    let regex = Regex::xpath(r#"^\p{L}"#, "");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"㐀"#));
}
