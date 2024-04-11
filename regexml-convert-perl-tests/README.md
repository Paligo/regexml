# regexml-convert-perl-tests

The [qt3 test suite](https://github.com/w3c/qt3tests) for XPath and XML Schema
contains, in the directory `fn/matches/perl-tests.xml` testing information for
XML XPath regex. The provenance of this file is unclear, and so it the meaning
of some of its `result` attributes, but it probably derives from a test suite
for Perl regexes.

This contains code to generate the `regexml/tests/test_qt_perl_matches.rs` module
from the included copy of the `perl-tests.xml` file.

Should you need to update `perl-tests.xml` you can update it in this module,
re-run the script as follows (from the project root):

```
cargo run -p regexml-convert-perl-tests > regexml/tests/test_qt_perl_matches.rs
```
