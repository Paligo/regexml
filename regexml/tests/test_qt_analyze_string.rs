use regexml::{AnalyzeEntry, Error, Regex};

// analyze-string with a mix of matching and non-matching strings
// #[test]
// fn test_analyze_string_003() {
//     let regex = Regex::xpath("a", "").unwrap();
//     let result = regex.analyze("banana").unwrap().collect::<Vec<_>>();
//     assert_eq!(result, vec![])
// }

// <test-case name="analyzeString-003">
// <description> analyze-string with a mix of matching and non-matching strings</description>
// <created by="Michael Kay" on="2009-10-18"/>
// <modified by="Michael Kay" on="2011-11-17" change="fix bug 14822"/>
// <test>analyze-string("banana", "a")</test>
// <result>
//    <assert-xml ignore-prefixes="true"><![CDATA[<fn:analyze-string-result xmlns:fn="http://www.w3.org/2005/xpath-functions"><fn:non-match>b</fn:non-match><fn:match>a</fn:match><fn:non-match>n</fn:non-match><fn:match>a</fn:match><fn:non-match>n</fn:non-match><fn:match>a</fn:match></fn:analyze-string-result>]]></assert-xml>
// </result>
// </test-case>
