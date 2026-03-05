"""Type stubs for the compiled _typst_syntax extension module."""

from __future__ import annotations
from typing import Optional

# ── SyntaxKind ────────────────────────────────────────────────────────────────

class SyntaxKind:
    name: str
    kind_id: int

    def is_trivia(self) -> bool: ...
    def is_keyword(self) -> bool: ...
    def is_error(self) -> bool: ...
    def is_block(self) -> bool: ...
    def is_stmt(self) -> bool: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

    # Structural
    END: SyntaxKind
    ERROR: SyntaxKind
    SHEBANG: SyntaxKind
    LINE_COMMENT: SyntaxKind
    BLOCK_COMMENT: SyntaxKind
    # Markup
    MARKUP: SyntaxKind
    TEXT: SyntaxKind
    SPACE: SyntaxKind
    LINEBREAK: SyntaxKind
    PARBREAK: SyntaxKind
    ESCAPE: SyntaxKind
    SHORTHAND: SyntaxKind
    SMART_QUOTE: SyntaxKind
    STRONG: SyntaxKind
    EMPH: SyntaxKind
    RAW: SyntaxKind
    RAW_LANG: SyntaxKind
    RAW_DELIM: SyntaxKind
    RAW_TRIMMED: SyntaxKind
    LINK: SyntaxKind
    LABEL: SyntaxKind
    REF: SyntaxKind
    REF_MARKER: SyntaxKind
    HEADING: SyntaxKind
    HEADING_MARKER: SyntaxKind
    LIST_ITEM: SyntaxKind
    LIST_MARKER: SyntaxKind
    ENUM_ITEM: SyntaxKind
    ENUM_MARKER: SyntaxKind
    TERM_ITEM: SyntaxKind
    TERM_MARKER: SyntaxKind
    # Math
    EQUATION: SyntaxKind
    MATH: SyntaxKind
    MATH_TEXT: SyntaxKind
    MATH_IDENT: SyntaxKind
    MATH_SHORTHAND: SyntaxKind
    MATH_ALIGN_POINT: SyntaxKind
    MATH_DELIMITED: SyntaxKind
    MATH_ATTACH: SyntaxKind
    MATH_PRIMES: SyntaxKind
    MATH_FRAC: SyntaxKind
    MATH_ROOT: SyntaxKind
    # Operators / punctuation
    HASH: SyntaxKind
    LEFT_BRACE: SyntaxKind
    RIGHT_BRACE: SyntaxKind
    LEFT_BRACKET: SyntaxKind
    RIGHT_BRACKET: SyntaxKind
    LEFT_PAREN: SyntaxKind
    RIGHT_PAREN: SyntaxKind
    COMMA: SyntaxKind
    SEMICOLON: SyntaxKind
    COLON: SyntaxKind
    STAR: SyntaxKind
    UNDERSCORE: SyntaxKind
    DOLLAR: SyntaxKind
    PLUS: SyntaxKind
    MINUS: SyntaxKind
    SLASH: SyntaxKind
    HAT: SyntaxKind
    PRIME: SyntaxKind
    DOT: SyntaxKind
    EQ: SyntaxKind
    EQ_EQ: SyntaxKind
    EXCL_EQ: SyntaxKind
    LT: SyntaxKind
    LT_EQ: SyntaxKind
    GT: SyntaxKind
    GT_EQ: SyntaxKind
    PLUS_EQ: SyntaxKind
    HYPH_EQ: SyntaxKind
    STAR_EQ: SyntaxKind
    SLASH_EQ: SyntaxKind
    DOTS: SyntaxKind
    ARROW: SyntaxKind
    ROOT: SyntaxKind
    NOT: SyntaxKind
    AND: SyntaxKind
    OR: SyntaxKind
    # Literals
    NONE: SyntaxKind
    AUTO: SyntaxKind
    IDENT: SyntaxKind
    BOOL: SyntaxKind
    INT: SyntaxKind
    FLOAT: SyntaxKind
    NUMERIC: SyntaxKind
    STR: SyntaxKind
    # Keywords
    LET: SyntaxKind
    SET: SyntaxKind
    SHOW: SyntaxKind
    CONTEXT: SyntaxKind
    IF: SyntaxKind
    ELSE: SyntaxKind
    FOR: SyntaxKind
    IN: SyntaxKind
    WHILE: SyntaxKind
    BREAK: SyntaxKind
    CONTINUE: SyntaxKind
    RETURN: SyntaxKind
    IMPORT: SyntaxKind
    INCLUDE: SyntaxKind
    AS: SyntaxKind
    # Code constructs
    CODE: SyntaxKind
    CODE_BLOCK: SyntaxKind
    CONTENT_BLOCK: SyntaxKind
    PARENTHESIZED: SyntaxKind
    ARRAY: SyntaxKind
    DICT: SyntaxKind
    NAMED: SyntaxKind
    KEYED: SyntaxKind
    UNARY: SyntaxKind
    BINARY: SyntaxKind
    FIELD_ACCESS: SyntaxKind
    FUNC_CALL: SyntaxKind
    ARGS: SyntaxKind
    SPREAD: SyntaxKind
    CLOSURE: SyntaxKind
    PARAMS: SyntaxKind
    LET_BINDING: SyntaxKind
    SET_RULE: SyntaxKind
    SHOW_RULE: SyntaxKind
    CONTEXTUAL: SyntaxKind
    CONDITIONAL: SyntaxKind
    WHILE_LOOP: SyntaxKind
    FOR_LOOP: SyntaxKind
    MODULE_IMPORT: SyntaxKind
    IMPORT_ITEMS: SyntaxKind
    IMPORT_ITEM_PATH: SyntaxKind
    RENAMED_IMPORT_ITEM: SyntaxKind
    MODULE_INCLUDE: SyntaxKind
    LOOP_BREAK: SyntaxKind
    LOOP_CONTINUE: SyntaxKind
    FUNC_RETURN: SyntaxKind
    DESTRUCTURING: SyntaxKind
    DESTRUCT_ASSIGNMENT: SyntaxKind

# ── Span / FileId ─────────────────────────────────────────────────────────────

class FileId:
    package: Optional[str]
    vpath: str
    def __repr__(self) -> str: ...

class Span:
    is_detached: bool
    file_id: Optional[FileId]
    def range(self) -> Optional[tuple[int, int]]: ...
    def __repr__(self) -> str: ...

# ── SyntaxError ───────────────────────────────────────────────────────────────

class SyntaxError:
    span: Span
    message: str
    hints: list[str]
    def __repr__(self) -> str: ...

# ── SyntaxNode ────────────────────────────────────────────────────────────────

class SyntaxNode:
    span: Span

    def kind(self) -> SyntaxKind: ...
    def text(self) -> str: ...
    def full_text(self) -> str: ...
    def children(self) -> list[SyntaxNode]: ...
    def is_leaf(self) -> bool: ...
    def is_erroneous(self) -> bool: ...
    def errors(self) -> list[SyntaxError]: ...
    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...

# ── Source / Lines ────────────────────────────────────────────────────────────

class Lines:
    def byte_to_line(self, byte: int) -> Optional[int]: ...
    def byte_to_column(self, byte: int) -> Optional[int]: ...
    def byte_to_line_column(self, byte: int) -> Optional[tuple[int, int]]: ...
    def line_to_byte(self, line: int) -> Optional[int]: ...
    def line_column_to_byte(self, line: int, col: int) -> Optional[int]: ...
    def line_to_range(self, line: int) -> Optional[tuple[int, int]]: ...
    def byte_to_utf16(self, byte: int) -> Optional[int]: ...
    def utf16_to_byte(self, utf16: int) -> Optional[int]: ...
    def len_bytes(self) -> int: ...
    def len_lines(self) -> int: ...
    def len_utf16(self) -> int: ...

class Source:
    @staticmethod
    def detached(text: str) -> Source: ...
    def root(self) -> SyntaxNode: ...
    def text(self) -> str: ...
    def lines(self) -> Lines: ...
    def find(self, span: Span) -> Optional[LinkedNode]: ...
    def range(self, span: Span) -> Optional[tuple[int, int]]: ...
    def edit(self, start: int, end: int, replacement: str) -> tuple[int, int]: ...
    def __repr__(self) -> str: ...

# ── LinkedNode ────────────────────────────────────────────────────────────────

class LinkedNode:
    def get(self) -> SyntaxNode: ...
    def offset(self) -> int: ...
    def range(self) -> tuple[int, int]: ...
    def index(self) -> int: ...
    def children(self) -> list[LinkedNode]: ...
    def parent(self) -> Optional[LinkedNode]: ...
    def prev_sibling(self) -> Optional[LinkedNode]: ...
    def next_sibling(self) -> Optional[LinkedNode]: ...
    def prev_leaf(self) -> Optional[LinkedNode]: ...
    def next_leaf(self) -> Optional[LinkedNode]: ...
    def leftmost_leaf(self) -> Optional[LinkedNode]: ...
    def rightmost_leaf(self) -> Optional[LinkedNode]: ...
    def __repr__(self) -> str: ...

# ── Tag ───────────────────────────────────────────────────────────────────────

class Tag:
    name: str
    COMMENT: Tag
    PUNCTUATION: Tag
    ESCAPE: Tag
    STRONG: Tag
    EMPH: Tag
    LINK: Tag
    RAW: Tag
    LABEL: Tag
    REF: Tag
    HEADING: Tag
    LIST_MARKER: Tag
    LIST_TERM: Tag
    MATH_DELIMITER: Tag
    MATH_OPERATOR: Tag
    KEYWORD: Tag
    OPERATOR: Tag
    NUMBER: Tag
    STRING: Tag
    FUNCTION: Tag
    INTERPOLATED: Tag
    ERROR: Tag
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

# ── Top-level functions ───────────────────────────────────────────────────────

def parse(text: str) -> SyntaxNode: ...
def parse_code(text: str) -> SyntaxNode: ...
def parse_math(text: str) -> SyntaxNode: ...
def highlight(node: LinkedNode) -> Optional[Tag]: ...
def highlight_html(source: Source) -> str: ...
def is_ident(s: str) -> bool: ...
def split_newlines(text: str) -> list[str]: ...
def link_prefix(text: str) -> tuple[str, bool]: ...
