use regexml::{Error, Regex};

#[test]
fn test_matches2args_1() {
    let regex = Regex::xpath("This is a characte", "").unwrap();
    assert!(regex.is_match("This is a characte"));
}

#[test]
fn test_matches_err_1() {
    let regex = Regex::xpath("bra", "p");
    assert_eq!(
        regex.unwrap_err(),
        Error::InvalidFlags("Unrecognized flag 'p'".to_string())
    );
}

// back-reference illegal in square backets. See erratum FO.E24
#[test]
fn test_matches_err_2() {
    let regex = Regex::xpath("^(#)abc[\\1]1$", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("Backreferences not allowed within character classes".to_string())
    );
}

// single-digit back-reference to non-existent group. See erratum FO.E24
#[test]
fn test_matches_err_3() {
    let regex = Regex::xpath("^(#)abc\\2$", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\2 (no such group)".to_string())
    );
}

// single-digit back-reference to group not yet closed. See erratum FO.E24
#[test]
fn test_matches_err_4() {
    let regex = Regex::xpath("^((#)abc\\1)$", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\1 (group not yet closed)".to_string())
    );
}

// double-digit back-reference to group not yet closed. See erratum FO.E24
#[test]
fn test_matches_err_5() {
    let regex = Regex::xpath(
        "^(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)((m)(n)(o)(p)(q)\\13)$",
        "",
    );
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("invalid backreference \\13 (group not yet closed)".to_string())
    );
}

// Evaluation of matches function as per example 1 (for this function)
#[test]
fn test_matches_1() {
    let regex = Regex::xpath("bra", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

//   <test-case name="fn-matches-2">
// <description> Evaluation of matches function as per example 2 (for this function). Pattern set to "^a.*a$". </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:matches("abracadabra", "^a.*a$")</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-3">
// <description> Evaluation of matches function as per example 3 (for this function). Pattern set to "^bra" </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:matches("abracadabra", "^bra")</test>
// <result>
//    <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-4">
// <description> Test that calling the function with flags set to the empty string is the same as ommiting the flags. </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:concat(fn:matches("abracadabra", "^bra"),fn:matches("abracadabra", "^bra", ""))</test>
// <result>
//    <assert-string-value>falsefalse</assert-string-value>
// </result>
// </test-case>

// <test-case name="fn-matches-5">
// <description> Evaluate the fn:mathes function with the input string set to the empty sequence. fn:count used to avoid empty file. </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:count(fn:matches("()", "^bra"))</test>
// <result>
//    <assert-string-value>1</assert-string-value>
// </result>
// </test-case>

// <test-case name="fn-matches-6">
// <description> Evaluation of matches function with pattern set to "\^". </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:matches("abracadabra^abracadabra", "\^")</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-7">
// <description> Evaluation of matches function with pattern set to "\?" for an input string that contains "?". </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:matches("abracadabra?abracadabra", "\?")</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-8">
// <description> Evaluation of matches function with pattern set to "\*" for an input string that contains "*". </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:matches("abracadabra*abracadabra", "\*")</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-9">
// <description> Evaluation of matches function with pattern set to "\+" for an input string that contains "+". </description>
// <created by="Carmelo Montanez" on="2005-10-14"/>
// <test>fn:matches("abracadabra+abracadabra", "\+")</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>

//   <test-case name="fn-matches-2">
//       <description> Evaluation of matches function as per example 2 (for this function). Pattern set to "^a.*a$". </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:matches("abracadabra", "^a.*a$")</test>
//       <result>
//          <assert-true/>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-3">
//       <description> Evaluation of matches function as per example 3 (for this function). Pattern set to "^bra" </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:matches("abracadabra", "^bra")</test>
//       <result>
//          <assert-false/>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-4">
//       <description> Test that calling the function with flags set to the empty string is the same as ommiting the flags. </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:concat(fn:matches("abracadabra", "^bra"),fn:matches("abracadabra", "^bra", ""))</test>
//       <result>
//          <assert-string-value>falsefalse</assert-string-value>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-5">
//       <description> Evaluate the fn:mathes function with the input string set to the empty sequence. fn:count used to avoid empty file. </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:count(fn:matches("()", "^bra"))</test>
//       <result>
//          <assert-string-value>1</assert-string-value>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-6">
//       <description> Evaluation of matches function with pattern set to "\^". </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:matches("abracadabra^abracadabra", "\^")</test>
//       <result>
//          <assert-true/>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-7">
//       <description> Evaluation of matches function with pattern set to "\?" for an input string that contains "?". </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:matches("abracadabra?abracadabra", "\?")</test>
//       <result>
//          <assert-true/>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-8">
//       <description> Evaluation of matches function with pattern set to "\*" for an input string that contains "*". </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:matches("abracadabra*abracadabra", "\*")</test>
//       <result>
//          <assert-true/>
//       </result>
//    </test-case>

//    <test-case name="fn-matches-9">
//       <description> Evaluation of matches function with pattern set to "\+" for an input string that contains "+". </description>
//       <created by="Carmelo Montanez" on="2005-10-14"/>
//       <test>fn:matches("abracadabra+abracadabra", "\+")</test>
//       <result>
//          <assert-true/>
//       </result>
//    </test-case>

// Evaluation of matches function as per example 2 (for this function). Pattern set to "^a.*a$".
#[test]
fn test_matches_2() {
    let regex = Regex::xpath("^a.*a$", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

// Evaluation of matches function as per example 3 (for this function). Pattern set to "^bra"
#[test]
fn test_matches_3() {
    let regex = Regex::xpath("^bra", "").unwrap();
    assert!(!regex.is_match("abracadabra"));
}

// Evaluate the fn:matches function with the input string set to the empty sequence.
#[test]
fn test_matches_5() {
    let regex = Regex::xpath("^bra", "").unwrap();
    assert!(!regex.is_match("()"));
}

// Evaluation of matches function with pattern set to "\^".
#[test]
fn test_matches_6() {
    let regex = Regex::xpath("\\^", "").unwrap();
    assert!(regex.is_match("abracadabra^abracadabra"));
}

// Evaluation of matches function with pattern set to "\?" for an input string that contains "?".
#[test]
fn test_matches_7() {
    let regex = Regex::xpath("\\?", "").unwrap();
    assert!(regex.is_match("abracadabra?abracadabra"));
}

// Evaluation of matches function with pattern set to "\*" for an input string that contains "*".
#[test]
fn test_matches_8() {
    let regex = Regex::xpath("\\*", "").unwrap();
    assert!(regex.is_match("abracadabra*abracadabra"));
}

// Evaluation of matches function with pattern set to "\+" for an input string that contains "+".
#[test]
fn test_matches_9() {
    let regex = Regex::xpath("\\+", "").unwrap();
    assert!(regex.is_match("abracadabra+abracadabra"));
}
