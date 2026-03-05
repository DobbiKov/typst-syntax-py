use pyo3::prelude::*;
use typst_syntax::{SyntaxNode, ast};
use typst_syntax::ast::AstNode;
use crate::syntax_node::PySyntaxNode;

fn node(n: &SyntaxNode) -> PySyntaxNode {
    PySyntaxNode { node: n.clone() }
}

fn untyped<'a, T: AstNode<'a>>(n: T) -> PySyntaxNode {
    PySyntaxNode { node: n.to_untyped().clone() }
}

// ── Markup ───────────────────────────────────────────────────────────────────

#[pyclass(name = "Markup")]
#[derive(Clone)]
pub struct PyMarkup { pub node: SyntaxNode }

#[pymethods]
impl PyMarkup {
    pub fn exprs(&self) -> Vec<PySyntaxNode> {
        self.node.cast::<ast::Markup>()
            .map(|m| m.exprs().map(|e| untyped(e)).collect())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Markup(...)".to_string() }
}

// ── Heading ──────────────────────────────────────────────────────────────────

#[pyclass(name = "Heading")]
#[derive(Clone)]
pub struct PyHeading { pub node: SyntaxNode }

#[pymethods]
impl PyHeading {
    #[getter]
    pub fn depth(&self) -> usize {
        self.node.cast::<ast::Heading>().map(|h| h.depth().get()).unwrap_or(1)
    }
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::Heading>()
            .map(|h| untyped(h.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Heading(depth={})", self.depth()) }
}

// ── Strong ───────────────────────────────────────────────────────────────────

#[pyclass(name = "Strong")]
#[derive(Clone)]
pub struct PyStrong { pub node: SyntaxNode }

#[pymethods]
impl PyStrong {
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::Strong>()
            .map(|s| untyped(s.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Strong(...)".to_string() }
}

// ── Emph ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Emph")]
#[derive(Clone)]
pub struct PyEmph { pub node: SyntaxNode }

#[pymethods]
impl PyEmph {
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::Emph>()
            .map(|e| untyped(e.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Emph(...)".to_string() }
}

// ── Raw ──────────────────────────────────────────────────────────────────────

#[pyclass(name = "Raw")]
#[derive(Clone)]
pub struct PyRaw { pub node: SyntaxNode }

#[pymethods]
impl PyRaw {
    #[getter]
    pub fn lang(&self) -> Option<String> {
        self.node.cast::<ast::Raw>()
            .and_then(|r| r.lang())
            .map(|l| l.get().to_string())
    }
    #[getter]
    pub fn block(&self) -> bool {
        self.node.cast::<ast::Raw>().map(|r| r.block()).unwrap_or(false)
    }
    pub fn text(&self) -> String {
        self.node.cast::<ast::Raw>()
            .map(|r| r.lines().map(|l| l.get().to_string()).collect::<Vec<_>>().join("\n"))
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Raw(lang={:?})", self.lang()) }
}

// ── Link ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Link")]
#[derive(Clone)]
pub struct PyLink { pub node: SyntaxNode }

#[pymethods]
impl PyLink {
    #[getter]
    pub fn dest(&self) -> String {
        self.node.cast::<ast::Link>()
            .map(|l| l.get().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Link({:?})", self.dest()) }
}

// ── Label ────────────────────────────────────────────────────────────────────

#[pyclass(name = "Label")]
#[derive(Clone)]
pub struct PyLabel { pub node: SyntaxNode }

#[pymethods]
impl PyLabel {
    pub fn as_str(&self) -> String {
        self.node.cast::<ast::Label>()
            .map(|l| l.get().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Label({:?})", self.as_str()) }
}

// ── Ref ──────────────────────────────────────────────────────────────────────

#[pyclass(name = "Ref")]
#[derive(Clone)]
pub struct PyRef { pub node: SyntaxNode }

#[pymethods]
impl PyRef {
    #[getter]
    pub fn target(&self) -> String {
        self.node.cast::<ast::Ref>()
            .map(|r| r.target().to_string())
            .unwrap_or_default()
    }
    #[getter]
    pub fn supplement(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::Ref>()
            .and_then(|r| r.supplement())
            .map(|s| untyped(s))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Ref({:?})", self.target()) }
}

// ── ListItem ─────────────────────────────────────────────────────────────────

#[pyclass(name = "ListItem")]
#[derive(Clone)]
pub struct PyListItem { pub node: SyntaxNode }

#[pymethods]
impl PyListItem {
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::ListItem>()
            .map(|i| untyped(i.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "ListItem(...)".to_string() }
}

// ── EnumItem ─────────────────────────────────────────────────────────────────

#[pyclass(name = "EnumItem")]
#[derive(Clone)]
pub struct PyEnumItem { pub node: SyntaxNode }

#[pymethods]
impl PyEnumItem {
    #[getter]
    pub fn number(&self) -> Option<u64> {
        self.node.cast::<ast::EnumItem>().and_then(|i| i.number())
    }
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::EnumItem>()
            .map(|i| untyped(i.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("EnumItem(number={:?})", self.number()) }
}

// ── TermItem ─────────────────────────────────────────────────────────────────

#[pyclass(name = "TermItem")]
#[derive(Clone)]
pub struct PyTermItem { pub node: SyntaxNode }

#[pymethods]
impl PyTermItem {
    #[getter]
    pub fn term(&self) -> PySyntaxNode {
        self.node.cast::<ast::TermItem>()
            .map(|i| untyped(i.term()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn description(&self) -> PySyntaxNode {
        self.node.cast::<ast::TermItem>()
            .map(|i| untyped(i.description()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "TermItem(...)".to_string() }
}

// ── SmartQuote ───────────────────────────────────────────────────────────────

#[pyclass(name = "SmartQuote")]
#[derive(Clone)]
pub struct PySmartQuote { pub node: SyntaxNode }

#[pymethods]
impl PySmartQuote {
    #[getter]
    pub fn double(&self) -> bool {
        self.node.cast::<ast::SmartQuote>().map(|q| q.double()).unwrap_or(false)
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("SmartQuote(double={})", self.double()) }
}

// ── Escape ───────────────────────────────────────────────────────────────────

#[pyclass(name = "Escape")]
#[derive(Clone)]
pub struct PyEscape { pub node: SyntaxNode }

#[pymethods]
impl PyEscape {
    pub fn get(&self) -> String {
        self.node.cast::<ast::Escape>()
            .map(|e| e.get().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Escape({:?})", self.get()) }
}

// ── Shorthand ────────────────────────────────────────────────────────────────

#[pyclass(name = "Shorthand")]
#[derive(Clone)]
pub struct PyShorthand { pub node: SyntaxNode }

#[pymethods]
impl PyShorthand {
    pub fn get(&self) -> String {
        self.node.cast::<ast::Shorthand>()
            .map(|s| s.get().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Shorthand({:?})", self.get()) }
}

pub fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMarkup>()?;
    m.add_class::<PyHeading>()?;
    m.add_class::<PyStrong>()?;
    m.add_class::<PyEmph>()?;
    m.add_class::<PyRaw>()?;
    m.add_class::<PyLink>()?;
    m.add_class::<PyLabel>()?;
    m.add_class::<PyRef>()?;
    m.add_class::<PyListItem>()?;
    m.add_class::<PyEnumItem>()?;
    m.add_class::<PyTermItem>()?;
    m.add_class::<PySmartQuote>()?;
    m.add_class::<PyEscape>()?;
    m.add_class::<PyShorthand>()?;
    Ok(())
}
