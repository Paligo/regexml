use std::io::Write;

use xot::Xot;

struct Test {
    id: String,
    regex: String,
    input: String,
    result: String,
    flags: String,
}

fn main() {
    // load perl-tests.xml file data
    let data = include_str!("perl-tests.xml");
    let mut xot = Xot::new();
    let id = xot.add_name("id");
    let regex = xot.add_name("regex");
    let input = xot.add_name("input");
    let result = xot.add_name("result");
    let flags = xot.add_name("flags");

    let root = xot.parse(data).unwrap();
    let doc = xot.document_element(root).unwrap();

    let mut tests = Vec::new();
    for child in xot.children(doc) {
        if xot.is_element(child) {
            let attributes = xot.attributes(child);
            let id = attributes.get(id).unwrap();
            let regex = attributes.get(regex).unwrap();
            let input = attributes.get(input).unwrap();
            let result = attributes.get(result).unwrap();
            let empty_flags = "".to_string();
            let flags = attributes.get(flags).unwrap_or(&empty_flags);
            tests.push(Test {
                id: id.clone(),
                regex: regex.clone(),
                input: input.clone(),
                result: result.clone(),
                flags: flags.clone(),
            });
        }
    }

    let mut stdout = std::io::stdout();
    writeln!(
        stdout,
        "// This file is generated by regexml-convert-perl-tests"
    )
    .unwrap();
    writeln!(
        stdout,
        "// Do not edit this file directly (except to auto-format)"
    )
    .unwrap();
    writeln!(stdout).unwrap();

    writeln!(stdout, "use regexml::Regex;").unwrap();
    writeln!(stdout).unwrap();
    for test in &tests {
        // There are a bunch of weirdo expected results in the original XML file, which
        // are unexplained
        // 'c', 'p', 'sc', 'Sc', 'Sy', 'yB', 'yM', 'Sn'
        // The QT3 tests do the following:
        // * if we have a match, the status must be 'y' or it's a failure
        // * if we have a non-match, the status must be 'n' or it's a failure
        // * if we have another status and not an error, it's silently ignored
        // * if there is an error but we expect 'y' or n', that's an error
        // * if there is an error but we expect something else, it's silently ignored.
        // This means that we can silently ignore these special statuses, and only
        // care about unexpected errors. We can do that with the unwrap.
        if matches!(
            test.result.as_str(),
            "c" | "p" | "sc" | "Sc" | "Sy" | "yB" | "yM" | "Sn"
        ) {
            continue;
        }
        generate_test_case(&mut stdout, test);
    }
}

fn generate_test_case(w: &mut impl Write, test: &Test) {
    writeln!(w, "#[test]").unwrap();
    writeln!(w, "fn test_{}() {{", test.id).unwrap();
    writeln!(
        w,
        "    let regex = Regex::xpath(r#\"{}\"#, \"{}\");",
        test.regex, test.flags
    )
    .unwrap();

    writeln!(w, "    // {}", test.result).unwrap();

    match test.result.as_str() {
        "y" => {
            writeln!(w, "    let regex = regex.unwrap();").unwrap();
            writeln!(w, "    assert!(regex.is_match(r#\"{}\"#));", test.input).unwrap();
        }
        "n" => {
            writeln!(w, "    let regex = regex.unwrap();").unwrap();
            writeln!(w, "    assert!(!regex.is_match(r#\"{}\"#));", test.input).unwrap();
        }
        _ => {
            panic!("unexpected result {}", test.result)
        }
    }
    writeln!(w, "}}").unwrap();
    writeln!(w).unwrap();
}
