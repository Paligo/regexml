# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/Paligo/regexml/releases/tag/regexml-v0.1.0) - 2025-03-19

### Other

- Upgrade dependencies.
- Quiet clippy on 1.83.0
- make the complex history of this project more clear.
- Something weird is going on, so let's just get the debug versions.
- More information when range is out of whack.
- Optimize can use move semantics so we can avoid clones.
- Clean up.
- Eliminate Rc<Operation>.
- Get rid of refcell in the main regex struct.
- And remove it again.
- This has been fixed so going to remove it, but want to leave a historical record of how I debugged this just in case.
- Construct word char with a better range.
- Fix bug in parser where a parser error wasn't properly detected.
- Handle PrivateUse.
- Add some compatibility blocks.
- We do enable IsHighSurrogates and IsLowSurrogates, as xsd 1.1 seems to want us to do so.
- Surprisingly saxon has this same behavior. It may be a bug, we're bug for bug compatible at least.
- Thinking about a test to replicate this.
- This test now passes.
- We need this import for tests.
- A bit of test whackamole. We fix the problem case, but we get another failing test instead.
- We do have the history object now.
- Add a history system in the hope it makes the bug go away. Unfortunately it doesn't.
- It's possible we need to implement the history object after all. So make everything hashable.
- A test ported over that is failing, we're debugging it now.
- A few tweaks.
- Match the original code better. Add problem case, commented out.
- Simplify (fix?) this logic.
- Simplify logic.
- optimization logic should now be enabled
- Add a is_repeating method which will help later.
- document the operations a bit better.
- Clean up some stray dbgs, start documenting a bit better.
- Simplify code some more.
- Make this an enumerate too.
- Use an iterator.
- Further simplification of initialization.
- Use cached version instead of recalculating it.
- More clippy.
- Remove History object for now.
- Remove more unused code.
- Remove unused warning system.
- Various cleanups.
- Implement optimization logic. Hard to test though.
- Implement more optimization rules.
- Complete translation of get_initial_character_class.
- Start building up get_initial_character_class.
- Translated all the analyze-string tests now.
- Lots more test cases translated.
- Nested groups starting to work.
- Make analyze-string work.
- Fix a bug in analyze string.
- A basic analyze string test is starting to work.
- Make the analyze string code compile at least.
- Reorganize the way the compiler is constructed and used, and how the pattern is passed along.
- Move analyze-string code into its own module.
- Move regex implementation out of the lib
- Add a whole bunch of tokenize tests and fix bugs.
- Handle matching empty string.
- Another note.
- Update the readme.
- More improvements with more tests.
- Port over more tests, fix a few more bugs.
- Handle emoticons.
- Translate more replace tests.
- Transpose more tests.
- Convert more tests.
- Port some more tests.
- Add a lot more test cases.
- Fix a bug with Is character classes.
- Complete caseless match tests.
- make more case insensitive compare work.
- Thanks to icu4x we can do the proper lower/upper casing.
- Start to enable some more tests. Struggle with case rules though.
- Fix last perl regex test failure!
- Simplify the code that handles ranges.
- The backtracking tests work now!
- It turns out these are tests for greedy fixed and reluctant fixed.
- Various tests for repeat.
- Some tests for repeat.
- More tests.
- Clean up test.
- Debug method so we can get all matches.
- path access, better constructor of matcher.
- Add a structural accessor, children.
- Disable failing tests for now.
- Cleanup; get rid of display as we're not using it. We use Debug instead.
- Reorganize more.
- Reorganize things to help debug this.
- Still trying to debug the backtracking with captured state bug.
- Add a note.
- Stray debug print.
- Fix category group membership \p and \P
- Document problems.
- Tweak test generation, fix more test behavior.
- Clean up stray println and dbg
- Extraneous debug.
- Tweak.
- Update tests, try to make more work.
- More tweaking.
- Add a note about the script.
- Move this into a multi-crate project.
