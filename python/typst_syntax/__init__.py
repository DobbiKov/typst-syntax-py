"""
typst-syntax: Python bindings for the Typst parser and CST/AST crate.

Quick start::

    import typst_syntax as ts

    source = ts.Source.detached("= Hello\\n\\nWorld")
    root = source.root()
    print(root.kind())          # SyntaxKind.MARKUP
    for child in root.children():
        print(child.kind(), child.full_text())

Common tasks:
- Parse a document: ``ts.parse(text)`` → SyntaxNode (full markup)
- Parse code snippet: ``ts.parse_code(text)``
- Parse math snippet: ``ts.parse_math(text)``
- Rich source with line info: ``ts.Source.detached(text)``
- Syntax highlighting: ``ts.highlight_html(source)``
- Cast to typed AST: ``ts.ast.cast_node(node)``
"""

from ._typst_syntax import (  # noqa: F401
    # Core classes
    SyntaxKind,
    Span,
    FileId,
    SyntaxError,
    SyntaxNode,
    Source,
    Lines,
    LinkedNode,
    Tag,
    # Parse functions
    parse,
    parse_code,
    parse_math,
    # Highlight
    highlight,
    highlight_html,
    # Utils
    is_ident,
    split_newlines,
    link_prefix,
)
from . import ast  # noqa: F401


def parse_source(text: str) -> "Source":
    """Convenience: create a Source from text (same as Source.detached)."""
    return Source.detached(text)
