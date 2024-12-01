# `textmate-scope-selector-peg`

Performant port of the Textmate scope selector parser from JavaScript to Rust.

The original parser grammar (PegJS) lives at [atom/first-mate](https://github.com/atom/first-mate).

## Performance

### `selector.matches`

The `rust-peg` parser was benchmarked for matching scopes against the following input:

- Selector: `source.matlab -comment -entity -support -string -variable -interpolation -source.shell`
- Match: `source.matlab meta.class.matlab meta.class.declaration.matlab entity.name.type.class.matlab`

The Rust crates parser produced speeds of $7.47ns/iter$, reduced from $31.8ns/iter$ in the `peggy` parser.

### `selector.get_prefix`

The `rust-peg` parser was benchmarked for prefix matching against the following input:

- Selector: `L:text.html.markdown - (comment, string, meta.paragraph.markdown, markup.*.block.markdown)`
- Match: `text.html.markdown meta.paragraph.markdown`

The Rust crate's parser produced speeds of $19.78ns/iter$.