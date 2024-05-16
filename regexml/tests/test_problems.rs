use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_properties::{sets, GeneralCategoryGroup};
use regexml::{Error, Regex};

#[test]
fn test_infinite_loop() {
    // {0,0} is really a no-op, but could lead to an infinite loop
    // in the FixedGreedy operation. Now we create a noop in such a situation.
    let regex = Regex::xpath(r#"(( a | ( bc ) ) {0,0} )+ xyz"#, "x");
    let regex = regex.unwrap();
    assert!(regex.is_match(r#"xyz"#));
}

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

#[test]
fn test_re00036_match_failure() {
    let matches_regex = Regex::xpath(r"^(?:(?:foo)?|bar)*$", "").unwrap();

    assert!(matches_regex.is_match("barfoobar"));
}

#[test]
fn test_low_surrogates() {
    let matches_regex = Regex::xpath(r"^(?:\p{IsLowSurrogates}?)$", "").unwrap();

    assert!(matches_regex.is_match(""));
}

#[test]
fn test_syntax() {
    let err = Regex::xpath(r"[^-[bc]]$", "").unwrap_err();
    assert_eq!(
        err,
        Error::Syntax("Nothing before subtraction operator".to_string())
    )
}

// this mimics matches.re.xml re00984 to discover why it is
// failing with regexml. The reason is that two characters,
// 8968 and 8969 are inaccurately considered as in \w by the test,
// but they cannot be as they're punctuation characters.
// https://github.com/w3c/qt3tests/issues/62
#[test]
fn test_word_characters() {
    // load characters from file
    let characters = include_str!("characters.txt");
    let entries = characters.split(',');
    let characters = entries.map(|entry| {
        let entry = entry.trim();
        if entry.starts_with("&#") {
            let value = entry[2..entry.len() - 1].parse::<u32>().unwrap();
            // now the value is a character code
            std::char::from_u32(value).unwrap()
        } else if entry.len() == 1 {
            entry.chars().next().unwrap()
        } else {
            panic!("weird entry")
        }
    });
    let regex = Regex::xpath(r"^(?:[\w])$", "").unwrap();
    let mut failed = false;
    for c in characters {
        // skip these so the test will pass;
        if c as u32 == 8968 || c as u32 == 8969 {
            continue;
        }
        if !regex.is_match(&c.to_string()) {
            failed = true;
            println!("failed for {} (integer {})", c, c as u32);
        }
    }
    assert!(!failed);
}
