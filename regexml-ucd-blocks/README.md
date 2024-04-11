## regexml-ucd-blocks

This contains a binary that produces (on standard output) a Rust module, using
the `Blocks.txt`, which is taken from the Unicode Consortium's `UCD.zip`.

If you have an updated `Blocks.txt` you can check it in here, and then update
the `block.rs` module in `regexml` as follows (in the project root):

```
cargo run -p regexml-ucd-blocks > regexml/src/block.rs
```

It would be better if `icu_properties` contains this information so that they are
up to date. I asked about this here:
https://github.com/unicode-org/icu4x/discussions/4798

I also considered the `unicode_blocks` module but it doesn't allow iteration
through the blocks which is what I require to do the name-based lookup.
