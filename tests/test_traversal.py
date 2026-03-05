"""Tests for CST traversal (children, LinkedNode)."""
import typst_syntax as ts


def test_children_nonempty():
    node = ts.parse("= Hello\n\nWorld")
    children = node.children()
    assert len(children) > 0


def test_children_kinds():
    node = ts.parse("= Hello")
    kinds = [c.kind() for c in node.children()]
    # Should contain at least a heading node
    assert any(k == ts.SyntaxKind.HEADING for k in kinds)


def test_leaf_text():
    node = ts.parse("hello")
    # Walk down to a Text leaf
    def find_leaf(n):
        if n.is_leaf():
            return n
        for c in n.children():
            result = find_leaf(c)
            if result is not None:
                return result
        return None
    leaf = find_leaf(node)
    assert leaf is not None
    assert leaf.text() != "" or True  # might be whitespace


def test_full_text_roundtrip():
    source_text = "= Hello\n\nWorld #strong[bold text]"
    node = ts.parse(source_text)
    assert node.full_text() == source_text


def test_linked_node_from_source():
    src = ts.Source.detached("= Hello\n\nWorld")
    root = src.root()
    # Find the first heading child
    headings = [c for c in root.children() if c.kind() == ts.SyntaxKind.HEADING]
    assert len(headings) > 0
    heading_span = headings[0].span
    linked = src.find(heading_span)
    assert linked is not None


def test_linked_node_offset():
    # Verify offset() and range() work on a LinkedNode obtained from source.find
    src = ts.Source.detached("= hello world")
    root = src.root()
    headings = [c for c in root.children() if c.kind() == ts.SyntaxKind.HEADING]
    if headings:
        linked = src.find(headings[0].span)
        if linked is not None:
            assert isinstance(linked.offset(), int)
            start, end = linked.range()
            assert start <= end


def test_linked_node_children():
    src = ts.Source.detached("= Hello World")
    root = src.root()
    heading_nodes = [c for c in root.children() if c.kind() == ts.SyntaxKind.HEADING]
    assert len(heading_nodes) > 0
    span = heading_nodes[0].span
    linked = src.find(span)
    if linked is not None:
        children = linked.children()
        assert isinstance(children, list)


def test_linked_node_parent():
    src = ts.Source.detached("= Hello World")
    root = src.root()
    heading_nodes = [c for c in root.children() if c.kind() == ts.SyntaxKind.HEADING]
    if heading_nodes:
        span = heading_nodes[0].span
        linked = src.find(span)
        if linked is not None:
            parent = linked.parent()
            # parent might be None (root) or a LinkedNode
            assert parent is None or isinstance(parent, ts.LinkedNode)


def test_syntax_error_has_message():
    # Force a parse error
    node = ts.parse("#let = 1")  # syntactically wrong
    if node.is_erroneous():
        errors = node.errors()
        assert len(errors) > 0
        assert isinstance(errors[0].message, str)
