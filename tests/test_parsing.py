"""Tests for basic parsing functionality."""
import typst_syntax as ts


def test_parse_returns_syntax_node():
    node = ts.parse("= Hello\n\nWorld")
    assert node is not None


def test_parse_root_kind():
    node = ts.parse("= Hello")
    assert node.kind() == ts.SyntaxKind.MARKUP


def test_parse_code_kind():
    node = ts.parse_code("let x = 1")
    assert node.kind() == ts.SyntaxKind.CODE


def test_parse_math_kind():
    node = ts.parse_math("x + y")
    assert node.kind() == ts.SyntaxKind.MATH


def test_source_detached():
    src = ts.Source.detached("= Hello\n\nWorld")
    assert src is not None
    assert "Hello" in src.text()


def test_source_root():
    src = ts.Source.detached("= Hello")
    root = src.root()
    assert root.kind() == ts.SyntaxKind.MARKUP


def test_parse_source_convenience():
    src = ts.parse_source("hello world")
    assert isinstance(src, ts.Source)


def test_syntax_node_full_text():
    node = ts.parse("= Hello\n\nWorld")
    text = node.full_text()
    assert "Hello" in text
    assert "World" in text


def test_syntax_node_len():
    node = ts.parse("abc")
    assert len(node) == 3


def test_syntax_node_is_leaf():
    node = ts.parse("abc")
    # The root node is not a leaf
    assert not node.is_leaf()
    # Children may include leaf nodes
    children = node.children()
    assert len(children) > 0


def test_syntax_kind_repr():
    node = ts.parse("= Hello")
    kind = node.kind()
    r = repr(kind)
    assert "SyntaxKind" in r and ("markup" in r.lower())


def test_syntax_kind_eq():
    node = ts.parse("= Hello")
    assert node.kind() == ts.SyntaxKind.MARKUP
    assert node.kind() != ts.SyntaxKind.HEADING


def test_syntax_kind_hash():
    k1 = ts.SyntaxKind.MARKUP
    k2 = ts.SyntaxKind.MARKUP
    assert hash(k1) == hash(k2)
    d = {k1: "test"}
    assert d[k2] == "test"


def test_no_errors_on_valid_input():
    node = ts.parse("= Hello\n\nWorld #strong[bold]")
    assert not node.is_erroneous()
    assert node.errors() == []


def test_lines_byte_to_line():
    src = ts.Source.detached("line1\nline2\nline3")
    lines = src.lines()
    assert lines.byte_to_line(0) == 0
    assert lines.byte_to_line(6) == 1
    assert lines.len_lines() == 3


def test_is_ident():
    assert ts.is_ident("hello") is True
    assert ts.is_ident("123") is False
    assert ts.is_ident("") is False


def test_split_newlines():
    parts = ts.split_newlines("a\nb\nc")
    assert len(parts) == 3


def test_link_prefix():
    prefix, has_proto = ts.link_prefix("https://example.com/foo")
    assert "https" in prefix
    assert has_proto is True
