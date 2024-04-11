## regexml-ucd-blocks

This contains a binary that produces (on standard output) a Rust module, using
the `Blocks.txt`, which is taken from the Unicode Consortium's `UCD.zip`.

If you have an updated `Blocks.txt` you can check it in here, and then update
the `blocks.rs` module in `regexml` as follows (in the project root):

```
cargo run -p regexml-ucd-blocks > regexml/src/blocks.rs
```

If the icu4x project includes blocks information, the code can be rewritten to
use that instead.
