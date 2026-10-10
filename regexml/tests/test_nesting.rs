use regexml::{Error, Regex};

// The deepest nesting of groups and character classes the compiler accepts.
const LIMIT: usize = 64;

// A spawned thread gets 2 MB of stack by default; matching and compiling at
// the limit must fit into it with room to spare, also in a debug build.
fn on_a_2mb_thread<T: Send + 'static>(f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(2 * 1024 * 1024)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap()
}

fn groups(n: usize) -> String {
    format!("{}a{}", "(".repeat(n), ")".repeat(n))
}

fn repeated_groups(n: usize) -> String {
    format!("{}a{}", "(".repeat(n), ")+".repeat(n))
}

fn non_capturing_groups(n: usize) -> String {
    format!("{}a{}", "(?:".repeat(n), ")".repeat(n))
}

fn alternatives(n: usize) -> String {
    format!("{}a{}", "(b|".repeat(n), ")".repeat(n))
}

// n character classes: [a-z-[a-z-[...[a]...]]]
fn subtractions(n: usize) -> String {
    format!("{}[a]{}", "[a-z-".repeat(n - 1), "]".repeat(n - 1))
}

// n - 1 groups around a character class
fn groups_around_a_class(n: usize) -> String {
    format!("{}[a-z]{}", "(".repeat(n - 1), ")".repeat(n - 1))
}

fn is_nesting_error(result: Result<Regex, Error>) -> bool {
    match result {
        Err(Error::Syntax(message)) => {
            message == format!("Groups and character classes nested more than {LIMIT} levels deep")
        }
        _ => false,
    }
}

#[test]
fn test_nesting_up_to_the_limit_compiles_and_matches() {
    on_a_2mb_thread(|| {
        for pattern in [
            groups(LIMIT),
            repeated_groups(LIMIT),
            non_capturing_groups(LIMIT),
            alternatives(LIMIT),
            subtractions(LIMIT),
            groups_around_a_class(LIMIT),
        ] {
            // (the subtractions alternate, so at an even depth the class is
            // [b-z])
            let regex = Regex::xpath(&pattern, "").unwrap();
            assert!(regex.is_match("xaay"), "{}", pattern);
            assert_ne!(regex.replace_all("xay", "-").unwrap(), "xay");
            let entries = regex.analyze("xaay").unwrap().collect::<Vec<_>>();
            assert!(entries.len() >= 2);
            assert!(regex.tokenize("xay").unwrap().count() >= 2);
        }
        // the XML Schema dialect has no non-capturing groups
        for pattern in [groups(LIMIT), subtractions(LIMIT)] {
            assert!(Regex::xsd(&pattern, "").unwrap().is_match("xa"));
        }
    });
}

#[test]
fn test_nesting_one_level_over_the_limit_is_an_error() {
    for pattern in [
        groups(LIMIT + 1),
        repeated_groups(LIMIT + 1),
        non_capturing_groups(LIMIT + 1),
        alternatives(LIMIT + 1),
        subtractions(LIMIT + 1),
        groups_around_a_class(LIMIT + 1),
    ] {
        assert!(is_nesting_error(Regex::xpath(&pattern, "")), "{}", pattern);
    }
    assert!(is_nesting_error(Regex::xsd(&groups(LIMIT + 1), "")));
    assert!(is_nesting_error(Regex::xsd(&subtractions(LIMIT + 1), "")));
}

#[test]
fn test_nesting_counts_open_levels_only() {
    // the limit is on the levels open at the same time: expressions nested up
    // to the limit, one after another, are fine
    let pattern = groups(LIMIT).repeat(10);
    assert!(Regex::xpath(&pattern, "")
        .unwrap()
        .is_match(&"a".repeat(10)));
    let pattern = format!("({})", subtractions(LIMIT - 1).repeat(10));
    assert!(Regex::xpath(&pattern, "")
        .unwrap()
        .is_match(&"a".repeat(10)));
}

#[test]
fn test_very_deep_nesting_is_an_error_not_a_stack_overflow() {
    on_a_2mb_thread(|| {
        let n = 10_000;
        assert!(is_nesting_error(Regex::xpath(&groups(n), "")));
        assert!(is_nesting_error(Regex::xpath(&repeated_groups(n), "")));
        assert!(is_nesting_error(Regex::xpath(&subtractions(n), "")));
        assert!(is_nesting_error(Regex::xsd(&groups(n), "")));
        // unbalanced
        assert!(is_nesting_error(Regex::xpath(&"(".repeat(n), "")));
        assert!(is_nesting_error(Regex::xpath(&"[a-".repeat(n), "")));
        // with the 'x' flag
        let spaced = format!("{} a {}", "( ".repeat(n), ") ".repeat(n));
        assert!(is_nesting_error(Regex::xpath(&spaced, "x")));
    });
}
