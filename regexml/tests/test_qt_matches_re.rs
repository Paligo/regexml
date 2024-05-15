// I haven't translated all the tests from the qt3 test runner,
// only those which are failing to debug them.

use regexml::Regex;

// Test regex syntax
#[test]
fn test_re00036() {
    let regex = Regex::xpath(",", "").unwrap();
    let output = regex.tokenize(",boy0xx,woman1y,girl1xymany,boy0xxwoman1ygirl1xymany,boy0xxwoman1ygirl1xymanyboy0xxwoman1ygirl1xymany").unwrap();
    let matches_regex = Regex::xpath(
        r"^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
        "",
    )
    .unwrap();

    let not_matches_regex = Regex::xpath(
        r"^(?:(((((boy)|(girl))[0-1][x-z]{2})?)|(man|woman)[0-1]?[y|n])*)$",
        "",
    )
    .unwrap();

    for s in output {
        println!("s: {}", s);
        assert!(matches_regex.is_match(&s));
        let not_matches_tokenize = regex.tokenize("boy0xxwoman1ygirl1xyman,boyxx").unwrap();
        for sub in not_matches_tokenize {
            assert!(!not_matches_regex.is_match(&sub));
        }
    }
}
