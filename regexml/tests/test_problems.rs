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

// #[test]
// fn test_re00036_match_failure() {
//     let matches_regex = Regex::xpath(
//         r"^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
//         "",
//     )
//     .unwrap();

//     let matches_regex = Regex::xpath(r"^((((girl)?)|(man|woman)[0-1]?[y|n])*)$", "").unwrap();

//     let matches_regex = Regex::xpath(r"^(?:(?:girl)?|man)*$", "").unwrap();

//     let matches_regex = Regex::xpath(r"^(?:(?:girl)?|man)*$", "").unwrap();

//     let matches_regex = Regex::xpath(r"^(?:(?:girl)?|man)*$", "").unwrap();

//     // let matches_regex = Regex::xpath(r"^(?:man|(?:girl)?)*$", "").unwrap();

//     dbg!(&matches_regex);
//     // the capturing group doesn't seem to be a problem

//     // let matches_regex = Regex::xpath(
//     //     r"^((((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
//     //     "",
//     // )
//     // .unwrap();
//     // assert!(matches_regex.is_match("many"));
//     // assert!(matches_regex.is_match("boy0xxwoman1ygirl1xygirl1xy"));
//     // assert!(matches_regex.is_match("girlgirl"));
//     assert!(matches_regex.is_match("mangirlman"));
//     // assert!(matches_regex.is_match("girlmangirl"));
//     // assert!(matches_regex.is_match("boy0woman1ygirl1many"));
// }
