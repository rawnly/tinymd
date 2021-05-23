# TinyMD
> Project inspired to [Jesse Lawson rust guide](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler).

## Usage
Install the latest release from the [releases page](https://github.com/rawnly/tinymd/releases/latest)

```sh
  $ tinymd <filename.md>
```

## Language support
This compiler is intended to be as simple as possible. Right now supports just a few markdown features:

- Headings (h1 to h6)
- Bold
- Underline
- Italic
- Code blocks
- Code word

Example output of this README: [README.html](README.html)

## To Do
- [x] Links and Images.
- [ ] Nested bold and italics.
- [ ] Tables (really?)

### Caveats
it currently doesn't support nested **bold** or *italic* such as
```md
  This is **some *nested* italic-bold**
```
