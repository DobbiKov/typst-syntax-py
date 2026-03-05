use pyo3::prelude::*;
use typst_syntax::SyntaxKind as SK;

#[pyclass(name = "SyntaxKind")]
#[derive(Clone)]
pub struct PySyntaxKind {
    pub kind: SK,
}

fn psk(k: SK) -> PySyntaxKind {
    PySyntaxKind { kind: k }
}

#[pymethods]
impl PySyntaxKind {
    #[getter]
    pub fn name(&self) -> &'static str {
        self.kind.name()
    }

    #[getter]
    pub fn kind_id(&self) -> u16 {
        self.kind as u16
    }

    pub fn is_trivia(&self) -> bool {
        self.kind.is_trivia()
    }

    pub fn is_keyword(&self) -> bool {
        self.kind.is_keyword()
    }

    pub fn is_error(&self) -> bool {
        self.kind.is_error()
    }

    pub fn is_block(&self) -> bool {
        self.kind.is_block()
    }

    pub fn is_stmt(&self) -> bool {
        self.kind.is_stmt()
    }

    pub fn __repr__(&self) -> String {
        format!("SyntaxKind.{}", self.kind.name())
    }

    pub fn __eq__(&self, other: &PySyntaxKind) -> bool {
        self.kind == other.kind
    }

    pub fn __hash__(&self) -> u64 {
        self.kind as u64
    }

    // ── Class attributes for every variant ──────────────────────────────────

    // Structural
    #[classattr] fn END() -> PySyntaxKind { psk(SK::End) }
    #[classattr] fn ERROR() -> PySyntaxKind { psk(SK::Error) }
    #[classattr] fn SHEBANG() -> PySyntaxKind { psk(SK::Shebang) }
    #[classattr] fn LINE_COMMENT() -> PySyntaxKind { psk(SK::LineComment) }
    #[classattr] fn BLOCK_COMMENT() -> PySyntaxKind { psk(SK::BlockComment) }

    // Markup
    #[classattr] fn MARKUP() -> PySyntaxKind { psk(SK::Markup) }
    #[classattr] fn TEXT() -> PySyntaxKind { psk(SK::Text) }
    #[classattr] fn SPACE() -> PySyntaxKind { psk(SK::Space) }
    #[classattr] fn LINEBREAK() -> PySyntaxKind { psk(SK::Linebreak) }
    #[classattr] fn PARBREAK() -> PySyntaxKind { psk(SK::Parbreak) }
    #[classattr] fn ESCAPE() -> PySyntaxKind { psk(SK::Escape) }
    #[classattr] fn SHORTHAND() -> PySyntaxKind { psk(SK::Shorthand) }
    #[classattr] fn SMART_QUOTE() -> PySyntaxKind { psk(SK::SmartQuote) }
    #[classattr] fn STRONG() -> PySyntaxKind { psk(SK::Strong) }
    #[classattr] fn EMPH() -> PySyntaxKind { psk(SK::Emph) }
    #[classattr] fn RAW() -> PySyntaxKind { psk(SK::Raw) }
    #[classattr] fn RAW_LANG() -> PySyntaxKind { psk(SK::RawLang) }
    #[classattr] fn RAW_DELIM() -> PySyntaxKind { psk(SK::RawDelim) }
    #[classattr] fn RAW_TRIMMED() -> PySyntaxKind { psk(SK::RawTrimmed) }
    #[classattr] fn LINK() -> PySyntaxKind { psk(SK::Link) }
    #[classattr] fn LABEL() -> PySyntaxKind { psk(SK::Label) }
    #[classattr] fn REF() -> PySyntaxKind { psk(SK::Ref) }
    #[classattr] fn REF_MARKER() -> PySyntaxKind { psk(SK::RefMarker) }
    #[classattr] fn HEADING() -> PySyntaxKind { psk(SK::Heading) }
    #[classattr] fn HEADING_MARKER() -> PySyntaxKind { psk(SK::HeadingMarker) }
    #[classattr] fn LIST_ITEM() -> PySyntaxKind { psk(SK::ListItem) }
    #[classattr] fn LIST_MARKER() -> PySyntaxKind { psk(SK::ListMarker) }
    #[classattr] fn ENUM_ITEM() -> PySyntaxKind { psk(SK::EnumItem) }
    #[classattr] fn ENUM_MARKER() -> PySyntaxKind { psk(SK::EnumMarker) }
    #[classattr] fn TERM_ITEM() -> PySyntaxKind { psk(SK::TermItem) }
    #[classattr] fn TERM_MARKER() -> PySyntaxKind { psk(SK::TermMarker) }

    // Math
    #[classattr] fn EQUATION() -> PySyntaxKind { psk(SK::Equation) }
    #[classattr] fn MATH() -> PySyntaxKind { psk(SK::Math) }
    #[classattr] fn MATH_TEXT() -> PySyntaxKind { psk(SK::MathText) }
    #[classattr] fn MATH_IDENT() -> PySyntaxKind { psk(SK::MathIdent) }
    #[classattr] fn MATH_SHORTHAND() -> PySyntaxKind { psk(SK::MathShorthand) }
    #[classattr] fn MATH_ALIGN_POINT() -> PySyntaxKind { psk(SK::MathAlignPoint) }
    #[classattr] fn MATH_DELIMITED() -> PySyntaxKind { psk(SK::MathDelimited) }
    #[classattr] fn MATH_ATTACH() -> PySyntaxKind { psk(SK::MathAttach) }
    #[classattr] fn MATH_PRIMES() -> PySyntaxKind { psk(SK::MathPrimes) }
    #[classattr] fn MATH_FRAC() -> PySyntaxKind { psk(SK::MathFrac) }
    #[classattr] fn MATH_ROOT() -> PySyntaxKind { psk(SK::MathRoot) }

    // Punctuation / operators
    #[classattr] fn HASH() -> PySyntaxKind { psk(SK::Hash) }
    #[classattr] fn LEFT_BRACE() -> PySyntaxKind { psk(SK::LeftBrace) }
    #[classattr] fn RIGHT_BRACE() -> PySyntaxKind { psk(SK::RightBrace) }
    #[classattr] fn LEFT_BRACKET() -> PySyntaxKind { psk(SK::LeftBracket) }
    #[classattr] fn RIGHT_BRACKET() -> PySyntaxKind { psk(SK::RightBracket) }
    #[classattr] fn LEFT_PAREN() -> PySyntaxKind { psk(SK::LeftParen) }
    #[classattr] fn RIGHT_PAREN() -> PySyntaxKind { psk(SK::RightParen) }
    #[classattr] fn COMMA() -> PySyntaxKind { psk(SK::Comma) }
    #[classattr] fn SEMICOLON() -> PySyntaxKind { psk(SK::Semicolon) }
    #[classattr] fn COLON() -> PySyntaxKind { psk(SK::Colon) }
    #[classattr] fn STAR() -> PySyntaxKind { psk(SK::Star) }
    #[classattr] fn UNDERSCORE() -> PySyntaxKind { psk(SK::Underscore) }
    #[classattr] fn DOLLAR() -> PySyntaxKind { psk(SK::Dollar) }
    #[classattr] fn PLUS() -> PySyntaxKind { psk(SK::Plus) }
    #[classattr] fn MINUS() -> PySyntaxKind { psk(SK::Minus) }
    #[classattr] fn SLASH() -> PySyntaxKind { psk(SK::Slash) }
    #[classattr] fn HAT() -> PySyntaxKind { psk(SK::Hat) }
    #[classattr] fn PRIME() -> PySyntaxKind { psk(SK::Prime) }
    #[classattr] fn DOT() -> PySyntaxKind { psk(SK::Dot) }
    #[classattr] fn EQ() -> PySyntaxKind { psk(SK::Eq) }
    #[classattr] fn EQ_EQ() -> PySyntaxKind { psk(SK::EqEq) }
    #[classattr] fn EXCL_EQ() -> PySyntaxKind { psk(SK::ExclEq) }
    #[classattr] fn LT() -> PySyntaxKind { psk(SK::Lt) }
    #[classattr] fn LT_EQ() -> PySyntaxKind { psk(SK::LtEq) }
    #[classattr] fn GT() -> PySyntaxKind { psk(SK::Gt) }
    #[classattr] fn GT_EQ() -> PySyntaxKind { psk(SK::GtEq) }
    #[classattr] fn PLUS_EQ() -> PySyntaxKind { psk(SK::PlusEq) }
    #[classattr] fn HYPH_EQ() -> PySyntaxKind { psk(SK::HyphEq) }
    #[classattr] fn STAR_EQ() -> PySyntaxKind { psk(SK::StarEq) }
    #[classattr] fn SLASH_EQ() -> PySyntaxKind { psk(SK::SlashEq) }
    #[classattr] fn DOTS() -> PySyntaxKind { psk(SK::Dots) }
    #[classattr] fn ARROW() -> PySyntaxKind { psk(SK::Arrow) }
    #[classattr] fn ROOT() -> PySyntaxKind { psk(SK::Root) }
    #[classattr] fn NOT() -> PySyntaxKind { psk(SK::Not) }
    #[classattr] fn AND() -> PySyntaxKind { psk(SK::And) }
    #[classattr] fn OR() -> PySyntaxKind { psk(SK::Or) }

    // Literals
    #[classattr] fn NONE() -> PySyntaxKind { psk(SK::None) }
    #[classattr] fn AUTO() -> PySyntaxKind { psk(SK::Auto) }
    #[classattr] fn IDENT() -> PySyntaxKind { psk(SK::Ident) }
    #[classattr] fn BOOL() -> PySyntaxKind { psk(SK::Bool) }
    #[classattr] fn INT() -> PySyntaxKind { psk(SK::Int) }
    #[classattr] fn FLOAT() -> PySyntaxKind { psk(SK::Float) }
    #[classattr] fn NUMERIC() -> PySyntaxKind { psk(SK::Numeric) }
    #[classattr] fn STR() -> PySyntaxKind { psk(SK::Str) }

    // Keywords
    #[classattr] fn LET() -> PySyntaxKind { psk(SK::Let) }
    #[classattr] fn SET() -> PySyntaxKind { psk(SK::Set) }
    #[classattr] fn SHOW() -> PySyntaxKind { psk(SK::Show) }
    #[classattr] fn CONTEXT() -> PySyntaxKind { psk(SK::Context) }
    #[classattr] fn IF() -> PySyntaxKind { psk(SK::If) }
    #[classattr] fn ELSE() -> PySyntaxKind { psk(SK::Else) }
    #[classattr] fn FOR() -> PySyntaxKind { psk(SK::For) }
    #[classattr] fn IN() -> PySyntaxKind { psk(SK::In) }
    #[classattr] fn WHILE() -> PySyntaxKind { psk(SK::While) }
    #[classattr] fn BREAK() -> PySyntaxKind { psk(SK::Break) }
    #[classattr] fn CONTINUE() -> PySyntaxKind { psk(SK::Continue) }
    #[classattr] fn RETURN() -> PySyntaxKind { psk(SK::Return) }
    #[classattr] fn IMPORT() -> PySyntaxKind { psk(SK::Import) }
    #[classattr] fn INCLUDE() -> PySyntaxKind { psk(SK::Include) }
    #[classattr] fn AS() -> PySyntaxKind { psk(SK::As) }

    // Code constructs
    #[classattr] fn CODE() -> PySyntaxKind { psk(SK::Code) }
    #[classattr] fn CODE_BLOCK() -> PySyntaxKind { psk(SK::CodeBlock) }
    #[classattr] fn CONTENT_BLOCK() -> PySyntaxKind { psk(SK::ContentBlock) }
    #[classattr] fn PARENTHESIZED() -> PySyntaxKind { psk(SK::Parenthesized) }
    #[classattr] fn ARRAY() -> PySyntaxKind { psk(SK::Array) }
    #[classattr] fn DICT() -> PySyntaxKind { psk(SK::Dict) }
    #[classattr] fn NAMED() -> PySyntaxKind { psk(SK::Named) }
    #[classattr] fn KEYED() -> PySyntaxKind { psk(SK::Keyed) }
    #[classattr] fn UNARY() -> PySyntaxKind { psk(SK::Unary) }
    #[classattr] fn BINARY() -> PySyntaxKind { psk(SK::Binary) }
    #[classattr] fn FIELD_ACCESS() -> PySyntaxKind { psk(SK::FieldAccess) }
    #[classattr] fn FUNC_CALL() -> PySyntaxKind { psk(SK::FuncCall) }
    #[classattr] fn ARGS() -> PySyntaxKind { psk(SK::Args) }
    #[classattr] fn SPREAD() -> PySyntaxKind { psk(SK::Spread) }
    #[classattr] fn CLOSURE() -> PySyntaxKind { psk(SK::Closure) }
    #[classattr] fn PARAMS() -> PySyntaxKind { psk(SK::Params) }
    #[classattr] fn LET_BINDING() -> PySyntaxKind { psk(SK::LetBinding) }
    #[classattr] fn SET_RULE() -> PySyntaxKind { psk(SK::SetRule) }
    #[classattr] fn SHOW_RULE() -> PySyntaxKind { psk(SK::ShowRule) }
    #[classattr] fn CONTEXTUAL() -> PySyntaxKind { psk(SK::Contextual) }
    #[classattr] fn CONDITIONAL() -> PySyntaxKind { psk(SK::Conditional) }
    #[classattr] fn WHILE_LOOP() -> PySyntaxKind { psk(SK::WhileLoop) }
    #[classattr] fn FOR_LOOP() -> PySyntaxKind { psk(SK::ForLoop) }
    #[classattr] fn MODULE_IMPORT() -> PySyntaxKind { psk(SK::ModuleImport) }
    #[classattr] fn IMPORT_ITEMS() -> PySyntaxKind { psk(SK::ImportItems) }
    #[classattr] fn IMPORT_ITEM_PATH() -> PySyntaxKind { psk(SK::ImportItemPath) }
    #[classattr] fn RENAMED_IMPORT_ITEM() -> PySyntaxKind { psk(SK::RenamedImportItem) }
    #[classattr] fn MODULE_INCLUDE() -> PySyntaxKind { psk(SK::ModuleInclude) }
    #[classattr] fn LOOP_BREAK() -> PySyntaxKind { psk(SK::LoopBreak) }
    #[classattr] fn LOOP_CONTINUE() -> PySyntaxKind { psk(SK::LoopContinue) }
    #[classattr] fn FUNC_RETURN() -> PySyntaxKind { psk(SK::FuncReturn) }
    #[classattr] fn DESTRUCTURING() -> PySyntaxKind { psk(SK::Destructuring) }
    #[classattr] fn DESTRUCT_ASSIGNMENT() -> PySyntaxKind { psk(SK::DestructAssignment) }
}
