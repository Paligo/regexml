# regexml

This crate implements a regular expression engine that's compliant with regular
expressions as defined in appendix G of the XML Schema 1.1 standard, part 2:

https://www.w3.org/TR/xmlschema11-2/#regexs

This is the regex language that XML Schema uses so the user can define patterns
as an additional constraint on string data in an XML document:

https://www.w3.org/TR/xmlschema11-2/#dc-pattern

The XPath and XQuery Functions and Operators 3.1 specification defines an
extension of these regular expressions for the purposes of use within the XPath
and XQuery standard function library:

https://www.w3.org/TR/xpath-functions-31/#regex-syntax

This crate also implements this extension.
