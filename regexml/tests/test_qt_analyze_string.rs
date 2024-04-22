use regexml::{AnalyzeEntry, Error, MatchEntry, Regex};

// analyze-string with empty string
#[test]
fn test_analyze_string_001() {
    let regex = Regex::xpath("abc", "").unwrap();
    let result = regex.analyze("").unwrap().collect::<Vec<_>>();
    assert_eq!(result, vec![])
}

// analyze-string with a mix of matching and non-matching strings
#[test]
fn test_analyze_string_003() {
    let regex = Regex::xpath("a", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("a".to_string())]),
            AnalyzeEntry::NonMatch("n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("a".to_string())]),
            AnalyzeEntry::NonMatch("n".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("a".to_string())]),
        ]
    )
}

// analyze-string with a single non-matching string
#[test]
fn test_analyze_string_004() {
    let regex = Regex::xpath("custard", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(result, vec![AnalyzeEntry::NonMatch("banana".to_string())])
}

// analyze-string with a single matching string
#[test]
fn test_analyze_string_005() {
    let regex = Regex::xpath(".+", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![AnalyzeEntry::Match(vec![MatchEntry::String(
            "banana".to_string()
        )])]
    )
}

// analyze-string with a adjacent matching strings
#[test]
fn test_analyze_string_006() {
    let regex = Regex::xpath("an", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![MatchEntry::String("an".to_string())]),
            AnalyzeEntry::Match(vec![MatchEntry::String("an".to_string())]),
            AnalyzeEntry::NonMatch("a".to_string()),
        ]
    )
}

// analyze-string with a single captured group
#[test]
fn test_analyze_string_007() {
    let regex = Regex::xpath("a(n)", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![
                MatchEntry::String("a".to_string()),
                MatchEntry::Group {
                    nr: 1,
                    value: vec![MatchEntry::String("n".to_string())]
                }
            ]),
            AnalyzeEntry::Match(vec![
                MatchEntry::String("a".to_string()),
                MatchEntry::Group {
                    nr: 1,
                    value: vec![MatchEntry::String("n".to_string())]
                }
            ]),
            AnalyzeEntry::NonMatch("a".to_string()),
        ]
    )
}

// analyze-string with nested captured groups
#[test]
fn test_analyze_string_008() {
    let regex = Regex::xpath("(a(n?))", "").unwrap();
    let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();

    let match0 = MatchEntry::Group {
        nr: 1,
        value: vec![
            MatchEntry::String("a".to_string()),
            MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("n".to_string())],
            },
        ],
    };

    let match1 = MatchEntry::Group {
        nr: 1,
        value: vec![
            MatchEntry::String("a".to_string()),
            MatchEntry::Group {
                nr: 2,
                value: vec![MatchEntry::String("n".to_string())],
            },
        ],
    };

    let match2 = MatchEntry::Group {
        nr: 1,
        value: vec![
            MatchEntry::String("a".to_string()),
            MatchEntry::Group {
                nr: 2,
                value: vec![],
            },
        ],
    };

    assert_eq!(
        result,
        vec![
            AnalyzeEntry::NonMatch("b".to_string()),
            AnalyzeEntry::Match(vec![match0]),
            AnalyzeEntry::Match(vec![match1]),
            AnalyzeEntry::Match(vec![match2]),
        ]
    );
}

// <test-case name="analyzeString-008">
// <description> analyze-string with nested captured groups</description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("banana", "(a(n?))")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:non-match>b</fn:non-match><fn:match><fn:group nr="1">a<fn:group nr="2">n</fn:group></fn:group></fn:match><fn:match><fn:group nr="1">a<fn:group nr="2">n</fn:group></fn:group></fn:match><fn:match><fn:group nr="1">a<fn:group nr="2"/></fn:group></fn:match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-009">
// <description> analyze-string, groups in alternatives </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("how now brown cow", "(how)|(now)|(brown)|(cow)")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match><fn:group nr="1">how</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="2">now</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="3">brown</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="4">cow</fn:group></fn:match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-010">
// <description> analyze-string, with i flag </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("how now brown cow", "(HOW)|(NOW)|(BROWN)|(COW)", "i")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match><fn:group nr="1">how</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="2">now</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="3">brown</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="4">cow</fn:group></fn:match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-011">
// <description> analyze-string, with i and x flag </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("how now brown cow", " (HOW) | (NOW) &#xa;| (BROWN) | (COW) ", "ix")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match><fn:group nr="1">how</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="2">now</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="3">brown</fn:group></fn:match><fn:non-match> </fn:non-match><fn:match><fn:group nr="4">cow</fn:group></fn:match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-012">
// <description> analyze-string, with flags set to empty string</description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("how now brown cow", "(.*?ow\s+)+", "")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match>how <fn:group nr="1">now </fn:group></fn:match><fn:non-match>brown cow</fn:non-match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-013">
// <description> analyze-string, with "s" flag</description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>let $in :=
// "Mary had a little lamb,
// it's fleece was black as soot,
// and everywhere that Mary went,
// it put its sooty foot."
//       return analyze-string($in, "Mary.*foot", "s")</test>
// <result>
//       <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match>Mary had a little lamb,
// it's fleece was black as soot,
// and everywhere that Mary went,
// it put its sooty foot</fn:match><fn:non-match>.</fn:non-match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-014">
// <description> analyze-string, multiple lines without "s" flag </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>let $in :=
// "Mary had a little lamb,
// it's fleece was black as soot,
// and everywhere that Mary went,
// it put its sooty foot."
//       return analyze-string($in, ".+", "")</test>
// <result>
//    <any-of>
//       <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match>Mary had a little lamb,</fn:match><fn:non-match>
// </fn:non-match><fn:match>it's fleece was black as soot,</fn:match><fn:non-match>
// </fn:non-match><fn:match>and everywhere that Mary went,</fn:match><fn:non-match>
// </fn:non-match><fn:match>it put its sooty foot.</fn:match></fn:analyze-string-result>]]></assert-xml>
//       </any-of>
// </result>
// </test-case>
// <test-case name="analyzeString-015">
// <description> analyze-string, multiple lines with "m" flag </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>let $in :=
// "Mary had a little lamb,
// it's fleece was black as soot,
// and everywhere that Mary went,
// it put its sooty foot."
//       return analyze-string($in, "^.+$", "m")</test>
// <result>
//    <any-of>
//       <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match>Mary had a little lamb,</fn:match><fn:non-match>
// </fn:non-match><fn:match>it's fleece was black as soot,</fn:match><fn:non-match>
// </fn:non-match><fn:match>and everywhere that Mary went,</fn:match><fn:non-match>
// </fn:non-match><fn:match>it put its sooty foot.</fn:match></fn:analyze-string-result>]]></assert-xml>
//       </any-of>
// </result>
// </test-case>
// <test-case name="analyzeString-016">
// <description>
// analyze-string, multiple lines with "m" flag
// </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>let $in :=
// "Mary had a little lamb,
// it's fleece was black as soot,
// and everywhere that Mary went,
// it put its sooty foot."
//       return analyze-string($in, "^.+$", "")</test>
// <result>
//    <any-of>
//       <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:non-match>Mary had a little lamb,
// it's fleece was black as soot,
// and everywhere that Mary went,
// it put its sooty foot.</fn:non-match></fn:analyze-string-result>]]></assert-xml>
//       </any-of>
// </result>
// </test-case>
// <test-case name="analyzeString-017">
// <description> subtle distinction in the positioning of an empty captured group </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("banana", "(b)(x?)")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match><fn:group nr="1">b</fn:group><fn:group nr="2"/></fn:match><fn:non-match>anana</fn:non-match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-017a">
// <description> subtle distinction in the positioning of an empty captured group</description>
// <created by="Michael Kay" on="2009-10-18"/><modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("banana", "(b(x?))")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match><fn:group nr="1">b<fn:group nr="2"/></fn:group></fn:match><fn:non-match>anana</fn:non-match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-018" covers-30="regex-non-capturing">
// <description> non-capturing group indicated by "(?:...)" </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("banana", "(?:b(an)*a)")</test>
// <result>
//   <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:match>ban<fn:group nr="1">an</fn:group>a</fn:match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-019" covers-30="regex-q-flag">
// <description> "q" flag suppresses grouping</description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("((banana))", "(banana)", "q")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:non-match>(</fn:non-match><fn:match>(banana)</fn:match><fn:non-match>)</fn:non-match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
// <test-case name="analyzeString-020">
// <description> test typing of result of analyze-string: with no import-schema </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <dependency type="feature" value="schemaValidation"/>
// <test>
//   let $result := analyze-string("banana", "(b)(anana)")
//   return ($result//@nr)[1] instance of attribute(nr, xs:positiveInteger)
// </test>
// <result>
//    <assert-true/>
// </result>
// </test-case>
// <test-case name="analyzeString-021">
// <description> test typing of result of analyze-string: with no import-schema </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <dependency type="feature" value="schemaValidation"/>
// <test>let $result := analyze-string("banana", "(b)(anana)") return $result instance of element(*, xs:untyped)</test>
// <result>
//    <assert-false/>
// </result>
// </test-case>
// <test-case name="analyzeString-022">
// <description> test string value of result of analyze-string </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>let $result := analyze-string("banana", "(b)(anana)") return string($result)</test>
// <result>
//       <assert-string-value>banana</assert-string-value>
// </result>
// </test-case>
// <test-case name="analyzeString-023">
// <description> test string value of result of analyze-string </description>
// <created by="Michael Kay" on="2009-10-18"/><modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>let $result := analyze-string("banana", "(b)(anana)") return string($result/fn:match[1])</test>
// <result>
//    <assert-string-value>banana</assert-string-value>
// </result>
// </test-case>
// <test-case name="analyzeString-024">
// <description> test typed value of result of analyze-string: referencing a name defined in the schema </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <modified by="O'Neil Delpratt" on="2012-08-17" change="fix bug 14873"/>
// <environment ref="analyze-string-schema" />
// <dependency type="spec" value="XQ30+"/>
// <dependency type="feature" value="schemaValidation"/>
// <dependency type="feature" value="schemaImport"/>
// <test>
//   import schema "http://www.w3.org/2005/xpath-functions";
//   let $result := analyze-string("banana", "(b)(anana)")
//   return $result/fn:match[1] instance of schema-element(fn:match)</test>
// <result>
//    <assert-true/>
// </result>
// </test-case>
// <test-case name="analyzeString-025">
// <description> test typing of result of analyze-string: with import-schema </description>
// <created by="Tim Mills" on="2012-03-22"/>
// <environment ref="analyze-string-schema" />
// <dependency type="spec" value="XQ30+"/>
// <dependency type="feature" value="schemaImport"/>
// <dependency type="feature" value="schemaValidation"/>
// <test>
// import schema "http://www.w3.org/2005/xpath-functions";
//   analyze-string("banana", "(b)(anana)") instance of schema-element(fn:analyze-string-result)
// </test>
// <result>
//    <assert-true/>
// </result>
// </test-case>
// <test-case name="analyzeString-026" covers-30="regex-dot-matching-cr">
// <description> "." does NOT match CR in default mode</description>
// <created by="Tim Mills" on="2012-09-25"/>
// <dependency type="spec" value="XQ30+"/>
// <test>exactly-one(fn:analyze-string(concat('Mary', codepoints-to-string(13), 'Jones'), 'y.J')/fn:non-match)/string()</test>
// <result>
// <assert-eq>concat('Mary', codepoints-to-string(13), 'Jones')</assert-eq>
// </result>
// </test-case>

// <test-case name="analyzeString-027" covers-30="regex-dot-matching-cr">
// <description> "." does NOT match CR in default mode</description>
// <created by="Tim Mills" on="2012-09-25"/>
// <dependency type="spec" value="XQ30+"/>
// <test>exactly-one(fn:analyze-string(concat('Mary', codepoints-to-string(13), 'Jones'), 'y.J', 's')/fn:match)/string()</test>
// <result>
// <assert-eq>concat('y', codepoints-to-string(13), 'J')</assert-eq>
// </result>
// </test-case>

// <test-case name="analyzeString-028" covers="map-general">
// <description> Result of analyze-string must have the right in-scope namespaces </description>
// <created by="Michael Kay" on="2016-09-26"/>
// <modified by="Tim Mills" on="2016-10-19" change="Reference map env"/>
// <environment ref="map" />
// <dependency type="spec" value="XQ31+"/>
// <test>
//    declare function local:namespaces($e as element(*)) as map(xs:string, xs:anyURI) {
//      map:merge(in-scope-prefixes($e) ! map{. : namespace-uri-for-prefix(., $e)})
//    };
//    let $m := local:namespaces(analyze-string((), "abc"))
//    return sort($m?*)
// </test>
// <result>
//    <assert-deep-eq>"http://www.w3.org/2005/xpath-functions", "http://www.w3.org/XML/1998/namespace"</assert-deep-eq>
// </result>
// </test-case>

// <test-case name="analyzeString-029" covers="map-general">
// <description> Matching groups within repeated clause </description>
// <created by="Michael Kay after Martin Honnen" on="2017-03-13"/>
// <dependency type="spec" value="XQ30+"/>
// <test><![CDATA[
//    let $data :=
//      <Root>
//        <DATA>/OPDH/FLOWING SOLUTION/SGDE/Number0983713/EKPH/Sample test/some other keys/</DATA>
//        <DATA>/some other keys/afdsf/SGDE/Number0983713/some other keys/PIHSAGA/OPDH/FLOWING SOLUTION/some other keys/No exception/EKPH/Sample test/some other keys/</DATA>
//      </Root>
//    return document{<out>{
//      $data/DATA!analyze-string(., '(/OPDH/|/EKPH/|/SGDE/|/some other keys/)(.*?)(/OPDH/|/EKPH/|/SGDE/|/some other keys/)((.*?)(/OPDH/|/EKPH/|/SGDE/|/some other keys/))*')
//    }</out>}
// ]]></test>
// <result>
//    <all-of>
//       <assert>$result/out/fn:analyze-string-result[1]/fn:match[1]/fn:group[@nr=1] = '/OPDH/'</assert>
//       <assert>$result/out/fn:analyze-string-result[1]/fn:match[1]/fn:group[@nr=2] = 'FLOWING SOLUTION'</assert>
//       <assert>$result/out/fn:analyze-string-result[1]/fn:match[1]/fn:group[@nr=3] = '/SGDE/'</assert>
//       <assert>$result/out/fn:analyze-string-result[1]/fn:match[1]/fn:group[@nr=4] = 'Sample test/some other keys/'</assert>
//       <assert>$result/out/fn:analyze-string-result[1]/fn:match[1]//fn:group[@nr=5][../@nr=4] = 'Sample test'</assert>
//       <assert>$result/out/fn:analyze-string-result[1]/fn:match[1]//fn:group[@nr=6][../@nr=4] = '/some other keys/'</assert>
//       <assert>$result/out/fn:analyze-string-result[2]/fn:match[1]/fn:group[@nr=1] = '/some other keys/'</assert>
//       <assert>$result/out/fn:analyze-string-result[2]/fn:match[1]/fn:group[@nr=2] = 'afdsf'</assert>
//       <assert>$result/out/fn:analyze-string-result[2]/fn:match[1]/fn:group[@nr=3] = '/SGDE/'</assert>
//       <assert>$result/out/fn:analyze-string-result[2]/fn:match[1]/fn:group[@nr=4] = 'Sample test/some other keys/'</assert>
//       <assert>$result/out/fn:analyze-string-result[2]/fn:match[1]//fn:group[@nr=5][../@nr=4] = 'Sample test'</assert>
//       <assert>$result/out/fn:analyze-string-result[2]/fn:match[1]//fn:group[@nr=6][../@nr=4] = '/some other keys/'</assert>
//    </all-of>
// </result>
// </test-case>

// <test-case name="analyzeString-901">
// <description> analyze-string, error, bad regex pattern</description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <modified by="O'Neil Delpratt" on="2012-05-22" change="Bug fix related to bug #14936: Changed test first argument"/>
// <test>analyze-string("abc", ")-(")</test>
// <result>
//       <error code="FORX0002"/>
// </result>
// </test-case>
// <test-case name="analyzeString-902">
// <description> analyze-string, error, bad flags </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("abc", "abc", "w")</test>
// <result>
//       <error code="FORX0001"/>
// </result>
// </test-case>
// <test-case name="analyzeString-903">
// <description> analyze-string, error, pattern matches a zero-length string </description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("abc", "a|b|c?")</test>
// <result>
//       <error code="FORX0003"/>
// </result>
// </test-case>
