"""Tests for syntax highlighting."""
import typst_syntax as ts


def test_highlight_html_returns_string():
    src = ts.Source.detached("= Hello\n\nWorld *bold*")
    html = ts.highlight_html(src)
    assert isinstance(html, str)
    assert len(html) > 0


def test_highlight_html_contains_spans():
    src = ts.Source.detached("= Hello")
    html = ts.highlight_html(src)
    assert "<span" in html


def test_highlight_html_heading():
    src = ts.Source.detached("= My Heading")
    html = ts.highlight_html(src)
    assert "My Heading" in html


def test_tag_repr():
    tag = ts.Tag.HEADING
    assert "HEADING" in repr(tag) or "Heading" in repr(tag)


def test_tag_eq():
    assert ts.Tag.COMMENT == ts.Tag.COMMENT
    assert ts.Tag.COMMENT != ts.Tag.HEADING


def test_tag_hash():
    d = {ts.Tag.KEYWORD: "keyword"}
    assert d[ts.Tag.KEYWORD] == "keyword"


def test_tag_name():
    assert isinstance(ts.Tag.COMMENT.name, str)
    assert len(ts.Tag.COMMENT.name) > 0


def test_highlight_function_linked_node():
    src = ts.Source.detached("= Hello")
    root = src.root()
    # Find a heading child
    headings = [c for c in root.children() if c.kind() == ts.SyntaxKind.HEADING]
    if headings:
        span = headings[0].span
        linked = src.find(span)
        if linked is not None:
            # highlight() may return None for some nodes
            result = ts.highlight(linked)
            assert result is None or isinstance(result, ts.Tag)


def test_highlight_all_variants_exist():
    """Check that all Tag class attrs are accessible."""
    tags = [
        ts.Tag.COMMENT, ts.Tag.PUNCTUATION, ts.Tag.ESCAPE,
        ts.Tag.STRONG, ts.Tag.EMPH, ts.Tag.LINK, ts.Tag.RAW,
        ts.Tag.LABEL, ts.Tag.REF, ts.Tag.HEADING,
        ts.Tag.LIST_MARKER, ts.Tag.LIST_TERM,
        ts.Tag.MATH_DELIMITER, ts.Tag.MATH_OPERATOR,
        ts.Tag.KEYWORD, ts.Tag.OPERATOR, ts.Tag.NUMBER,
        ts.Tag.STRING, ts.Tag.FUNCTION, ts.Tag.INTERPOLATED,
        ts.Tag.ERROR,
    ]
    assert len(tags) == 21
    for t in tags:
        assert isinstance(t, ts.Tag)
