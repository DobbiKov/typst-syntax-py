"""Tests for typed AST node access."""
import typst_syntax as ts
from typst_syntax import ast


def _find_kind(node, kind):
    """Recursively find first child with given kind."""
    if node.kind() == kind:
        return node
    for c in node.children():
        result = _find_kind(c, kind)
        if result is not None:
            return result
    return None


def test_cast_heading():
    root = ts.parse("= Hello World\n")
    heading_node = _find_kind(root, ts.SyntaxKind.HEADING)
    assert heading_node is not None
    typed = ast.cast_node(heading_node)
    assert typed is not None
    assert isinstance(typed, ast.Heading)
    assert typed.depth >= 1


def test_heading_depth():
    root = ts.parse("=== Level 3\n")
    heading_node = _find_kind(root, ts.SyntaxKind.HEADING)
    assert heading_node is not None
    typed = ast.cast_node(heading_node)
    assert isinstance(typed, ast.Heading)
    assert typed.depth == 3


def test_heading_body():
    root = ts.parse("= Hello\n")
    heading_node = _find_kind(root, ts.SyntaxKind.HEADING)
    typed = ast.cast_node(heading_node)
    assert isinstance(typed, ast.Heading)
    body = typed.body
    assert body is not None
    assert isinstance(body, ts.SyntaxNode)


def test_cast_strong():
    root = ts.parse("*bold text*")
    strong_node = _find_kind(root, ts.SyntaxKind.STRONG)
    if strong_node is not None:
        typed = ast.cast_node(strong_node)
        assert isinstance(typed, ast.Strong)
        assert typed.body is not None


def test_cast_emph():
    root = ts.parse("_italic text_")
    emph_node = _find_kind(root, ts.SyntaxKind.EMPH)
    if emph_node is not None:
        typed = ast.cast_node(emph_node)
        assert isinstance(typed, ast.Emph)


def test_cast_raw():
    root = ts.parse("`code`")
    raw_node = _find_kind(root, ts.SyntaxKind.RAW)
    if raw_node is not None:
        typed = ast.cast_node(raw_node)
        assert isinstance(typed, ast.Raw)
        assert isinstance(typed.block, bool)


def test_cast_func_call():
    root = ts.parse_code("foo(1, 2)")
    func_node = _find_kind(root, ts.SyntaxKind.FUNC_CALL)
    if func_node is not None:
        typed = ast.cast_node(func_node)
        assert isinstance(typed, ast.FuncCall)
        assert typed.callee is not None
        assert typed.args is not None


def test_cast_let_binding():
    root = ts.parse_code("let x = 42")
    let_node = _find_kind(root, ts.SyntaxKind.LET_BINDING)
    if let_node is not None:
        typed = ast.cast_node(let_node)
        assert isinstance(typed, ast.LetBinding)


def test_cast_ident():
    root = ts.parse_code("myVariable")
    ident_node = _find_kind(root, ts.SyntaxKind.IDENT)
    if ident_node is not None:
        typed = ast.cast_node(ident_node)
        assert isinstance(typed, ast.Ident)
        assert typed.as_str() == "myVariable"


def test_cast_int():
    root = ts.parse_code("42")
    int_node = _find_kind(root, ts.SyntaxKind.INT)
    if int_node is not None:
        typed = ast.cast_node(int_node)
        assert isinstance(typed, ast.Int)
        assert typed.get() == 42


def test_cast_float():
    root = ts.parse_code("3.14")
    float_node = _find_kind(root, ts.SyntaxKind.FLOAT)
    if float_node is not None:
        typed = ast.cast_node(float_node)
        assert isinstance(typed, ast.Float)
        assert abs(typed.get() - 3.14) < 1e-9


def test_cast_str():
    root = ts.parse_code('"hello"')
    str_node = _find_kind(root, ts.SyntaxKind.STR)
    if str_node is not None:
        typed = ast.cast_node(str_node)
        assert isinstance(typed, ast.Str)
        assert typed.get() == "hello"


def test_cast_equation():
    root = ts.parse("$x + y$")
    eq_node = _find_kind(root, ts.SyntaxKind.EQUATION)
    if eq_node is not None:
        typed = ast.cast_node(eq_node)
        assert isinstance(typed, ast.Equation)
        assert isinstance(typed.block, bool)
        assert typed.body is not None


def test_cast_binary():
    root = ts.parse_code("1 + 2")
    bin_node = _find_kind(root, ts.SyntaxKind.BINARY)
    if bin_node is not None:
        typed = ast.cast_node(bin_node)
        assert isinstance(typed, ast.Binary)
        assert "Add" in typed.op or "+" in typed.op


def test_cast_unknown_returns_none():
    root = ts.parse("hello")
    # A plain TEXT node has no typed AST wrapper
    text_node = _find_kind(root, ts.SyntaxKind.TEXT)
    if text_node is not None:
        result = ast.cast_node(text_node)
        assert result is None


def test_markup_exprs():
    root = ts.parse("= Hello\n\nWorld")
    markup_node = _find_kind(root, ts.SyntaxKind.MARKUP)
    assert markup_node is not None
    typed = ast.cast_node(markup_node)
    assert isinstance(typed, ast.Markup)
    exprs = typed.exprs()
    assert len(exprs) > 0
