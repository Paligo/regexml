# Port Notes

Some notes on the Rust port. These are mainaly left for future maintainers if
problems are uncovered.

## History object

The original codebase features a history object, with a
`isDuplicateZeroLengthMatch` check. This relies on hashing `Operation`, but
making `Operation` hashable is not trivial; some of the data structures
involved rely on `CodePointInvList` which is itself not hashable.

Since the code passes all the tests without the `History` object, we have
assumed that this history object is not necessary and removed it, even though
perhaps some operations become more slow as a result.

It would be possible to make `Operation` hashable by writing a manual hasher
for the character class object.

Seee `saxon/regex/History.java` for the original.
