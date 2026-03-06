# typst-syntax-py

Python bindings for the [typst-syntax](https://crates.io/crates/typst-syntax) crate — the official Typst parser. Lets you parse Typst source files and traverse the resulting CST/AST from Python.

## Installation

```bash
pip install typst-syntax
```

## Quick start

```python
import typst_syntax as ts

source = ts.Source.detached("= Hello\n\nWorld *bold*")

root = source.root()
print(root.kind())        # SyntaxKind.markup
print(root.full_text())   # = Hello\n\nWorld *bold*

for child in root.children():
    print(child.kind(), repr(child.full_text()))
```

## Parsing

```python
# Full markup document
node = ts.parse("= Heading\n\nParagraph")

# Code snippet
node = ts.parse_code("let x = 1 + 2")

# Math snippet
node = ts.parse_math("x^2 + y^2")
```

## Typed AST nodes

`cast_node()` converts a raw `SyntaxNode` into a typed wrapper with named fields:

```python
from typst_syntax import ast

root = ts.parse("= My Heading\n")

for child in root.children():
    if child.kind() == ts.SyntaxKind.HEADING:
        h = ast.cast_node(child)   # -> ast.Heading
        print(h.depth)             # 1
        print(h.body.full_text())  # "My Heading"
```

Available typed nodes include `Heading`, `Strong`, `Emph`, `Raw`, `Link`, `FuncCall`,
`LetBinding`, `Binary`, `Equation`, `MathFrac`, and [many more](python/typst_syntax/ast/__init__.py).

## Syntax highlighting

```python
source = ts.Source.detached("= Hello\n\n#strong[world]")

# Full HTML with inline <span> tags
html = ts.highlight_html(source)

# Per-node tag via LinkedNode
linked = source.find(some_span)
tag = ts.highlight(linked)   # ts.Tag.HEADING, ts.Tag.KEYWORD, etc.
```

## Line / column info

```python
source = ts.Source.detached("line one\nline two\nline three")
lines  = source.lines()

lines.byte_to_line(9)          # 1  (0-indexed)
lines.byte_to_line_column(9)   # (1, 0)
lines.line_to_byte(2)          # 18
lines.len_lines()              # 3
```

## Utilities

```python
ts.is_ident("hello")      # True
ts.is_ident("123")        # False

ts.split_newlines("a\nb\nc")           # ["a", "b", "c"]
ts.link_prefix("https://example.com")  # ("https://example.com", True)
```

## Building from source

Requires Rust and [maturin](https://github.com/PyO3/maturin).

```bash
git clone https://github.com/you/typst-syntax-py
cd typst-syntax-py
python -m venv .venv && source .venv/bin/activate
maturin develop
```

## License

MIT
