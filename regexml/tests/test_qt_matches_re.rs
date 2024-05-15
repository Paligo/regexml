// I haven't translated all the tests from the qt3 test runner,
// only those which are failing.

//  <test-case name="re00036">
// <description>Test regex syntax</description>
// <created by="Michael Kay" on="2011-07-04"/>
// <test>(every $s in
// tokenize(',boy0xx,woman1y,girl1xymany,boy0xxwoman1ygirl1xymany,boy0xxwoman1ygirl1xymanyboy0xxwoman1ygirl1xymany', ',')
// satisfies matches($s, '^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$'))
// and (every $s in tokenize('boy0xxwoman1ygirl1xyman,boyxx', ',') satisfies not(matches($s, '^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$')))</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>

use regexml::Regex;

// // Test regex syntax
// #[test]
// fn test_re00036() {
//     let regex = Regex::xpath(",", "").unwrap();
//     let output = regex.tokenize(",boy0xx,woman1y,girl1xymany,boy0xxwoman1ygirl1xymany,boy0xxwoman1ygirl1xymanyboy0xxwoman1ygirl1xymany").unwrap();
//     let matches_regex = Regex::xpath(
//         r"^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
//         "",
//     )
//     .unwrap();

//     let not_matches_regex = Regex::xpath(
//         r"^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
//         "",
//     )
//     .unwrap();

//     for s in output {
//         println!("s: {}", s);
//         assert!(matches_regex.is_match(&s));
//         let not_matches_tokenize = regex.tokenize("boy0xxwoman1ygirl1xyman,boyxx").unwrap();
//         for sub in not_matches_tokenize {
//             assert!(!not_matches_regex.is_match(&sub));
//         }
//     }
// }

// #[test]
// fn test_re00036_matches() {
//     let matches_regex = Regex::xpath(
//         r"^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
//         "",
//     )
//     .unwrap();

//     let matches_regex = Regex::xpath(r"^((((girl)?)|(man|woman)[0-1]?[y|n])*)$", "").unwrap();

//     let matches_regex = Regex::xpath(r"^((((girl)?)|man)*)$", "").unwrap();

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

//     // assert!(matches_regex.is_match("boy0woman1ygirl1many"));
// }
