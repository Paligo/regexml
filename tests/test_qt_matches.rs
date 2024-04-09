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

// Evaluation of matches function as per example 2 (for this function). Pattern
// set to "^a.*a$".
#[test]
fn test_matches_2() {
    let regex = Regex::xpath("^a.*a$", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

// Evaluation of matches function as per example 3 (for this function). Pattern
// set to "^bra"
#[test]
fn test_matches_3() {
    let regex = Regex::xpath("^bra", "").unwrap();
    assert!(!regex.is_match("abracadabra"));
}

// Evaluate the fn:matches function with the input string set to the empty
// sequence.
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

// Evaluation of matches function with pattern set to "\?" for an input string
// that contains "?".
#[test]
fn test_matches_7() {
    let regex = Regex::xpath("\\?", "").unwrap();
    assert!(regex.is_match("abracadabra?abracadabra"));
}

// Evaluation of matches function with pattern set to "\*" for an input string
// that contains "*".
#[test]
fn test_matches_8() {
    let regex = Regex::xpath("\\*", "").unwrap();
    assert!(regex.is_match("abracadabra*abracadabra"));
}

// Evaluation of matches function with pattern set to "\+" for an input string
// that contains "+".
#[test]
fn test_matches_9() {
    let regex = Regex::xpath("\\+", "").unwrap();
    assert!(regex.is_match("abracadabra+abracadabra"));
}

// Evaluation of matches function with pattern set to "\{" for an input string
// that contains "}".
#[test]
fn test_matches_10() {
    let regex = Regex::xpath("\\{", "").unwrap();
    assert!(regex.is_match("abracadabra{abracadabra"));
}

// Evaluation of matches function with pattern set to "\}" for an input string
// that contains "}".
#[test]
fn test_matches_11() {
    let regex = Regex::xpath("\\}", "").unwrap();
    assert!(regex.is_match("abracadabra}abracadabra"));
}

// Evaluation of matches function with pattern set to "\(" for an input string
// that contains "(".
#[test]
fn test_matches_12() {
    let regex = Regex::xpath("\\(", "").unwrap();
    assert!(regex.is_match("abracadabra(abracadabra"));
}

// Evaluation of matches function with pattern set to "\)" for an input string
// that contains ")".
#[test]
fn test_matches_13() {
    let regex = Regex::xpath("\\)", "").unwrap();
    assert!(regex.is_match("abracadabra)abracadabra"));
}

// Evaluation of matches function with pattern set to "\[" for an input string
// that contains "[".
#[test]
fn test_matches_14() {
    let regex = Regex::xpath("\\[", "").unwrap();
    assert!(regex.is_match("abracadabra[abracadabra"));
}

// Evaluation of matches function with pattern set to "\]" for an input string
// that contains "]".
#[test]
fn test_matches_15() {
    let regex = Regex::xpath("\\]", "").unwrap();
    assert!(regex.is_match("abracadabra]abracadabra"));
}

// Evaluation of matches function with pattern set to "\-" for an input string
// that contains "-".
#[test]
fn test_matches_16() {
    let regex = Regex::xpath("\\-", "").unwrap();
    assert!(regex.is_match("abracadabra-abracadabra"));
}

// Evaluation of matches function with pattern set to "\." for an input string
// that contains ".".
#[test]
fn test_matches_17() {
    let regex = Regex::xpath("\\.", "").unwrap();
    assert!(regex.is_match("abracadabra.abracadabra"));
}

// Evaluation of matches function with pattern set to "\|" for an input string
// that contains "|".
#[test]
fn test_matches_18() {
    let regex = Regex::xpath("\\|", "").unwrap();
    assert!(regex.is_match("abracadabra|abracadabra"));
}

// Evaluation of matches function with pattern set to "\\" for an input string
// that contains "\".
#[test]
fn test_matches_19() {
    let regex = Regex::xpath("\\\\", "").unwrap();
    assert!(regex.is_match("abracadabra\\abracadabra"));
}

// Evaluation of matches function with pattern set to "\t" for an input string
// that contains the tab character.
#[test]
fn test_matches_20() {
    let regex = Regex::xpath("\\t", "").unwrap();
    assert!(regex.is_match("abracadabra\tabracadabra"));
}

// Evaluation of matches function with pattern set to "\n" for an input string
// that contains the newline character.
#[test]
fn test_matches_21() {
    let regex = Regex::xpath("\\n", "").unwrap();
    assert!(regex.is_match("abracadabra\nabracadabra"));
}

// Evaluation of matches function with pattern set to "aa{1}" (exact quantity)
// for an input string that contains the "aa" string.
#[test]
fn test_matches_22() {
    let regex = Regex::xpath("aa{1}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabra"));
}

// Evaluation of matches function with pattern set to "aa{1,}" (min quantity)
// for an input string that contains the "aa" string twice.
#[test]
fn test_matches_23() {
    let regex = Regex::xpath("aa{1,}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabraabracadabra"));
}

// Evaluation of matches function with pattern set to "aa{1,2}" (range
// quantity) for an input string that contains the "aa" string twice.
#[test]
fn test_matches_24() {
    let regex = Regex::xpath("aa{1,2}", "").unwrap();
    assert!(regex.is_match("abracadabraabracadabraabracadabra"));
}

// Evaluation of matches function with invalid regular expression
#[test]
fn test_matches_err_25() {
    let regex = Regex::xpath("**%%", "");
    assert_eq!(
        regex.unwrap_err(),
        Error::Syntax("No expression before quantifier".to_string())
    );
}

// Check for the correct behavior of ^ and $ in multi-line mode This test case
// was motivated by the resolution of Bug Report 4543. Note that '^' matches
// the position after any newline other than a newline that is the last
// character in the input string.
#[test]
fn test_matches_26() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(!regex.is_match("abcd\ndefg\n"));
}

// Check for the correct behavior of ^ and $ in multi-line mode This test case
// was motivated by the resolution of Bug Report 4543.
#[test]
fn test_matches_27() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(regex.is_match("\nabcd\ndefg\n"));
}

// Check for the correct behavior of ^ and $ in multi-line mode This test case
// was motivated by the resolution of Bug Report 4543
#[test]
fn test_matches_28() {
    let regex = Regex::xpath("^$", "m").unwrap();
    assert!(regex.is_match("abcd\n\ndefg\n"));
}

// 2-digits not treated as a back-reference See erratum FO.E24
#[test]
fn test_matches_29() {
    let regex = Regex::xpath("^(#)abc\\11$", "").unwrap();
    assert!(regex.is_match("#abc#1"));
}

// 2-digits treated as a back-reference See erratum FO.E24
#[test]
fn test_matches_30() {
    let regex = Regex::xpath(
        "^(#)(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)(m)(n)(o)(p)(q)\\11$",
        "",
    )
    .unwrap();
    assert!(!regex.is_match("#abcdefghijklmnopq#1"));
}

// Evaluation of matches function with non-capturing groups (allowed in XPath
// 3.0)
#[test]
fn test_matches_31() {
    let regex = Regex::xpath("(?:abra(?:cad)?)*", "").unwrap();
    assert!(regex.is_match("abracadabra"));
}

// <test-case name="fn-matches-32" covers-30="regex-non-capturing regex-q-flag">
// <description> Evaluation of matches function with "q" flag (allowed in XQuery 3.0) </description>
// <created by="Michael Kay" on="2009-10-23"/>
// <modified by="Michael Kay" on="2011-09-05" change="remove option of returning error code"/>
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>fn:matches("abracadabra", "(?:abra(?:cad)?)*", "q")</test>
// <result>
//     <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-33" covers-30="regex-q-flag">
// <description> Evaluation of matches function with "q" flag (allowed in XQuery 3.0) </description>
// <created by="Michael Kay" on="2009-10-23"/>
// <modified by="Michael Kay" on="2011-09-05" change="remove option of returning error code"/>
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>fn:matches("x[y-z]", "x[y-z]", "q")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-34" covers-30="regex-q-flag">
// <description> Evaluation of matches function with "q" and "i" flags (allowed in XQuery 3.0) </description>
// <created by="Michael Kay" on="2009-10-23"/>
// <modified by="Michael Kay" on="2011-09-05" change="remove option of returning error code"/>
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>fn:matches("x[Y-z]", "X[y-Z]", "qi")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-35">
// <description> Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \99 is an invalid reference as 99th subexpression does not exist </description>
// <created by="Zhen Hua  Liu" on="2009-11-15"/>
// <test>fn:matches('aA', '(a)\99')</test>
// <result>
//     <error code="FORX0002"/>
// </result>
// </test-case>

// <test-case name="fn-matches-36">
// <description> Test for bug fix of 5348 in Errata for F+O. ok match here </description>
// <created by="Zhen Hua  Liu" on="2009-11-15"/>
// <test>fn:matches('abcdefghijj', '(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)\10')</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-37">
// <description> Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \11 reference is made before the closing right parenthesis of 11th reference </description>
// <created by="Zhen Hua  Liu" on="2009-11-15"/>
// <test>fn:matches('abcdefghijk', '(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k\11)')</test>
// <result>
//     <error code="FORX0002"/>
// </result>
// </test-case>

// <test-case name="fn-matches-38">
// <description> Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \10 reference is made before the closing right parenthesis of 10th reference </description>
// <created by="Andrew Eisenberg" on="2009-12-23"/>
// <test>fn:matches('abcdefghijj', '(a)(b)(c)(d)(e)(f)(g)(h)(i)(j\10)')</test>
// <result>
//     <error code="FORX0002"/>
// </result>
// </test-case>

// <test-case name="fn-matches-39">
// <description> Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \9 reference is made before the closing right parenthesis of 9th reference </description>
// <created by="Andrew Eisenberg" on="2009-12-23"/>
// <test>fn:matches('abcdefghii', '(a)(b)(c)(d)(e)(f)(g)(h)(i\9)')</test>
// <result>
//     <error code="FORX0002"/>
// </result>
// </test-case>

// <test-case name="fn-matches-40">
// <description> Test for bug fix of 5348 in Errata for F+O. Expect FORX0002 err because \1 reference is made before the closing right parenthesis of 1st reference </description>
// <created by="Andrew Eisenberg" on="2009-12-23"/>
// <test>fn:matches('aa', '(a\1)')</test>
// <result>
//     <error code="FORX0002"/>
// </result>
// </test-case>

// <test-case name="fn-matches-41">
// <description> Handling of final newline with non-multiline mode </description>
// <created by="Michael Kay" on="2012-01-13"/>
// <test>fn:matches(concat('Mary', codepoints-to-string(10)), 'Mary$')</test>
// <result>
//     <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-42">
// <description> Handling of final newline with $ in dot-all mode </description>
// <created by="Michael Kay" on="2012-01-13"/>
// <test>fn:matches(concat('Mary', codepoints-to-string(10)), 'Mary$', 's')</test>
// <result>
//     <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-43">
// <description> "." doesn't normally match newline </description>
// <created by="Michael Kay" on="2012-01-13"/>
// <test>fn:matches(concat('Mary', codepoints-to-string(10), 'Jones'), 'Mary.Jones')</test>
// <result>
//     <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-44">
// <description> "." does match newline in dot-all mode</description>
// <created by="Michael Kay" on="2012-01-13"/>
// <test>fn:matches(concat('Mary', codepoints-to-string(10), 'Jones'), 'Mary.Jones', 's')</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-45" covers-30="regex-dot-matching-cr">
// <description> "." does NOT match CR in default mode</description>
// <created by="Michael Kay" on="2012-01-13"/>
// <modified by="Michael Kay" on="2012-03-28" change="See bug 15594. WG agreed that '.' should match everything except CR and NL"/>
// <test>fn:matches(concat('Mary', codepoints-to-string(13), 'Jones'), 'Mary.Jones')</test>
// <result>
//     <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-46" covers-30="regex-dot-matching-cr">
// <description> "." does match CR in dot-all mode</description>
// <created by="Michael Kay" on="2012-01-13"/>
// <test>fn:matches(concat('Mary', codepoints-to-string(13), 'Jones'), 'Mary.Jones', 's')</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-47">
// <description> Check for the correct behavior of $ when not in multi-line mode.
//  The correct answer according to the spec is false; though some regex engines
//  are known to report true.</description>
// <created by="Michael Kay" on="2012-04-19"/>
// <test>fn:matches(concat('abcd', codepoints-to-string(10), 'defg', codepoints-to-string(10)), "g$")</test>
// <result>
//     <assert-false/>
// </result>
// </test-case>

// <test-case name="fn-matches-48">
// <description> Edge condition: match occurs at last character. </description>
// <created by="Michael Kay" on="2012-12-14"/>
// <test>fn:matches("abracadabra-abracadabra.", "\.")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-49">
// <description> Edge condition: match occurs at last character. </description>
// <created by="Michael Kay" on="2012-12-14"/>
// <test>fn:matches("abracadabra-abracadabra-3", "(124|864|377|3)")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-50">
// <description>Composite test executing regular expressions from Perl-derived test file </description>
// <created by="Michael Kay" on="2015-05-20"/>
// <modified by="Michael Kay" on="2015-12-02" change="Changed p303 and p908 as per bug 29253"/>
// <modified by="Michael Kay" on="2016-12-03" change="Added XSD 1.0 dependency as per bug 29253"/>
// <environment>
//     <source role="." file="matches/perl-tests.xml"/>
// </environment>
// <dependency type="spec" value="XQ30+"/>
// <dependency type="xsd-version" value="1.0"/>
// <!-- See bug 30029 - four of the subtests assume XSD 1.0 rules on hyphens -->

// <test><![CDATA[
// declare namespace err="http://www.w3.org/2005/xqt-errors";
//   <results>{
//       for $t in /tests/test
//       return try {
//           let $matches := matches($t/@input, $t/@regex, string($t/@flags))
//           return
//              if ($matches (:trace($matches, $t/@id):) and $t/@result ne 'y')
//                then <fail>{$t}</fail>
//              else if (not($matches) and $t/@result ne 'n')
//                then <fail>{$t}</fail>
//              else ()
//       } catch * {
//           if ($t/@result = ('y', 'n'))
//           then <fail error="{$err:code}">{$t}</fail>
//           else ()
//       }
//   }</results>
// ]]>        </test>
// <result>
//     <assert>empty($result//fail)</assert>
// </result>
// </test-case>

// <test-case name="fn-matches-51">
// <description> Unescaped left parens inside a charClass are allowed and don't affect the subexpression count</description>
// <created by="Michael Kay" on="2016-02-09"/>
// <test>fn:matches("ab()cd()ef()gh", "^(ab)([()]*)(cd)([)(]*)ef\4gh$")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-52">
// <description> A use case involving backtracking and ambiguity</description>
// <created by="Michael Kay" on="2016-12-03"/>
// <test>fn:matches("aaababaaabaa", "^(a*b?a*){3,3}$")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-53">
// <description> A use case involving repetition of a back-reference. Saxon bug 3712.</description>
// <created by="Michael Kay" on="2018-03-06"/>
// <test>fn:matches("A", "([A-Z])\1*")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-54">
// <description> A use case involving optional matching of start-of-string. Saxon bug 3782.</description>
// <created by="Michael Kay" on="2018-05-15"/>
// <test>fn:matches("kZ", "(^|:)?Z")</test>
// <result>
//     <assert-true/>
// </result>
// </test-case>

// <test-case name="fn-matches-55">
// <description> Matching reluctant quantifier with min cardinality. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "ab", "aab", "aaab", "aaaab", "aaaaab") ! fn:matches(., "^(a{3,}?)b")</test>
// <result>
//     <assert-deep-eq>false(), false(), false(), true(), true(), true()</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="fn-matches-56">
// <description> Matching reluctant quantifier with max cardinality. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "ab", "aab", "aaab", "aaaab", "aaaaab") ! fn:matches(., "^(a{0,3}?)b")</test>
// <result>
//     <assert-deep-eq>true(), true(), true(), true(), false(), false()</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="fn-matches-57">
// <description> Matching reluctant quantifier with min and max cardinality. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "ab", "aab", "aaab", "aaaab", "aaaaab") ! fn:matches(., "^(a{2,3}?)b")</test>
// <result>
//     <assert-deep-eq>false(), false(), true(), true(), false(), false()</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="fn-matches-58">
// <description> Matching reluctant quantifier with min cardinality, variable length item that repeats. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "ab", "aab", "aaab", "aaazab", "aaaaab") ! fn:matches(., "^((az?){3,}?)b")</test>
// <result>
//     <assert-deep-eq>false(), false(), false(), true(), true(), true()</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="fn-matches-59">
// <description> Matching reluctant quantifier with max cardinality, variable length item that repeats. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "ab", "aazb", "aaab", "aaaab", "aaaaab") ! fn:matches(., "^((az?){0,3}?)b")</test>
// <result>
//     <assert-deep-eq>true(), true(), true(), true(), false(), false()</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="fn-matches-60">
// <description> Matching reluctant quantifier with min and max cardinality, variable length item that repeats. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "ab", "aazb", "aaab", "aaaab", "aaaaab") ! fn:matches(., "^((az?){2,3}?)b")</test>
// <result>
//     <assert-deep-eq>false(), false(), true(), true(), false(), false()</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="fn-matches-61">
// <description> Matching reluctant quantifier with min and max cardinality, backtracking required. See Saxon bug 3902</description>
// <created by="Michael Kay" on="2018-09-13"/>
// <modified by="Michael Kay" on="2018-09-26" change="Requires XP30+|XQ30+" />
// <dependency type="spec" value="XP30+ XQ30+"/>
// <test>("b", "aa", "aaza", "aaaa", "aaaaa", "aaaaaa") ! fn:matches(., "^((az?){2,3}?)a$")</test>
// <result>
//     <assert-deep-eq>false(), false(), true(), true(), false(), false()</assert-deep-eq>
// </result>
// </test-case>
