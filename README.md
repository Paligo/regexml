# regexml

If you need a Regex engine in Rust, this crate isn't likely for you; use the
[regex crate](https://crates.io/crates/regex) instead. This crate implements a
Regex engine compliant with varous XML-related standards, and focuses on
standard-compliance rather than performance.

`regexml` implements a regular expression engine that's compliant with regular
expressions as defined in appendix G of the _XML Schema 1.1 standard, part 2_:

https://www.w3.org/TR/xmlschema11-2/#regexs

This is the regex language that XML Schema uses so the user can define patterns
as additional constraints on string data in an XML document:

https://www.w3.org/TR/xmlschema11-2/#dc-pattern

The _XPath and XQuery Functions and Operators 3.1_ specification defines an
extension of these regular expressions for the purposes of use within the XPath
and XQuery standard function library:

https://www.w3.org/TR/xpath-functions-31/#regex-syntax

`regexml` also implements this extension.

The Rust source code is based on the Java implementation in Saxon HE
`net.sf.saxon.regex`, which implements a spec-compliant Regex engine. In turn
this code is based on an engine implemented by Apache Jakarta:

https://blog.saxonica.com/mike/2012/01/a-new-regex-engine.html
