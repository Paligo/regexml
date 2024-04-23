# regexml

To start off, don't worry. We're not using regexes to parse XML in this crate.
We're not in fact dealing with XML directly at all.

If you need a Regex engine in Rust, this crate isn't likely for you; use the
[regex crate](https://crates.io/crates/regex) instead. This crate instead
implements a Regex engine compliant with varous XML-related standards, and
focuses on standard-compliance rather than performance.

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

## Origins

The Rust source code is based on the Java implementation in Saxon HE
`net.sf.saxon.regex`, which implements a spec-compliant Regex engine. In turn
this code is based on an engine implemented by Apache Jakarta:

https://blog.saxonica.com/mike/2012/01/a-new-regex-engine.html

The Java code has been translated by hand into Rust. There are some differences:

- `Operation` is an enum, instead of implemented using subclasses and dynamic
  dispatch as in the Java version. A trait `OperationControl` provides dispatch
  to the enums.

- The [`icu4x` project](https://docs.rs/icu/latest/icu/)'s `icu_` crates are
  used to provide various unicode features, including the implementation of
  character classes and casing rules. Especially `CodePointInversionList`and
  its associated builder proved very useful. Due to the way the regex compiler is
  organized`CharacterClass` does provide a special case for character class
  that matches with a single character.

- The original code had no internal tests, but a lot of integration tests were
  provided through the [qt3tests](https://github.com/w3c/qt3tests) project for
  testing XPath and XQuery. Most of those tests have been ported into simple
  Rust tests, which makes this package easier to maintain and debug.

Now that the port is complete we expect this package to evolve separately
wherever it may go - no 1 to 1 mapping with the original Java code is going to
be maintained.

See also the `PORT_NOTES.md` for some further nodes on porting.
