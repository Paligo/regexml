// A reluctant repetition of an operand that is not of fixed length, when a
// repetition fails before `min` repetitions are reached, has no match; it
// used to try the same repetition again for ever. The expected values agree
// with Python's `re`.

use std::sync::mpsc;
use std::time::Duration;

use regexml::{AnalyzeEntry, Regex};

// Runs `f` on its own thread and fails the test if it does not finish in
// time, so that a regression fails instead of hanging the test run.
fn within_seconds<T: Send + 'static>(seconds: u64, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = sender.send(f());
    });
    receiver
        .recv_timeout(Duration::from_secs(seconds))
        .expect("the regular expression did not finish matching")
}

fn is_match(re: &'static str, haystack: &'static str) -> bool {
    within_seconds(10, move || Regex::xpath(re, "").unwrap().is_match(haystack))
}

#[test]
fn test_reluctant_repeat_without_a_match() {
    assert!(!is_match("(a|bc)+?", "x"));
    assert!(!is_match("(a|bc)+?", ""));
    assert!(!is_match("(a|bc){2,3}?", "xa"));
    assert!(!is_match("(a|bc){2}?", "az"));
    assert!(!is_match("(?:a+?)+?", "xyz"));
    assert!(!is_match("((a|bc)+?d)+?", "abcx"));
}

#[test]
fn test_reluctant_repeat_after_a_failed_start() {
    assert!(is_match("(a|bc)+?", "xbcy"));
    assert!(is_match("(a|bc){2,3}?", "xabc"));
    assert!(is_match("(a|bc){2}?", "azabcz"));
    assert!(is_match("(?:a+?)+?", "xaay"));
    assert!(is_match("((a|bc)+?d)+?", "abcxabcd"));
}

#[test]
fn test_reluctant_repeat_with_zero_repetitions() {
    assert!(is_match("^(a|bc)*?$", ""));
    assert!(is_match("^x(a|bc)*?y$", "xy"));
    assert!(is_match("(a|bc)*?x", "abcx"));
    assert!(!is_match("^(a|bc)*?$", "x"));
}

#[test]
fn test_reluctant_repeat_replace_tokenize_analyze() {
    let (replaced, tokens, entries) = within_seconds(10, || {
        let regex = Regex::xpath("(a|bc)+?", "").unwrap();
        let replaced = regex.replace_all("xabcbcy", "-").unwrap();
        let tokens = regex.tokenize("xabcx").unwrap().collect::<Vec<_>>();
        let entries = regex.analyze("xbcy").unwrap().collect::<Vec<_>>();
        (replaced, tokens, entries)
    });
    assert_eq!(replaced, "x---y");
    assert_eq!(tokens, vec!["x", "", "x"]);
    assert_eq!(entries.len(), 3);
    assert!(matches!(&entries[0], AnalyzeEntry::NonMatch(s) if s == "x"));
    assert!(matches!(&entries[1], AnalyzeEntry::Match(_)));
    assert!(matches!(&entries[2], AnalyzeEntry::NonMatch(s) if s == "y"));
}
