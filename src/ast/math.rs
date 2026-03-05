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

// ── Equation ──────────────────────────────────────────────────────────────────

#[pyclass(name = "Equation")]
#[derive(Clone)]
pub struct PyEquation { pub node: SyntaxNode }

#[pymethods]
impl PyEquation {
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::Equation>()
            .map(|e| untyped(e.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn block(&self) -> bool {
        self.node.cast::<ast::Equation>().map(|e| e.block()).unwrap_or(false)
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Equation(block={})", self.block()) }
}

// ── MathIdent ────────────────────────────────────────────────────────────────

#[pyclass(name = "MathIdent")]
#[derive(Clone)]
pub struct PyMathIdent { pub node: SyntaxNode }

#[pymethods]
impl PyMathIdent {
    pub fn as_str(&self) -> String {
        self.node.cast::<ast::MathIdent>()
            .map(|i| i.as_str().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("MathIdent({:?})", self.as_str()) }
}

// ── MathAttach ───────────────────────────────────────────────────────────────

#[pyclass(name = "MathAttach")]
#[derive(Clone)]
pub struct PyMathAttach { pub node: SyntaxNode }

#[pymethods]
impl PyMathAttach {
    #[getter]
    pub fn base(&self) -> PySyntaxNode {
        self.node.cast::<ast::MathAttach>()
            .map(|a| untyped(a.base()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn top(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::MathAttach>()
            .and_then(|a| a.top())
            .map(|e| untyped(e))
    }
    #[getter]
    pub fn bottom(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::MathAttach>()
            .and_then(|a| a.bottom())
            .map(|e| untyped(e))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "MathAttach(...)".to_string() }
}

// ── MathDelimited ────────────────────────────────────────────────────────────

#[pyclass(name = "MathDelimited")]
#[derive(Clone)]
pub struct PyMathDelimited { pub node: SyntaxNode }

#[pymethods]
impl PyMathDelimited {
    #[getter]
    pub fn open(&self) -> String {
        self.node.cast::<ast::MathDelimited>()
            .map(|d| d.open().to_untyped().text().to_string())
            .unwrap_or_default()
    }
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::MathDelimited>()
            .map(|d| untyped(d.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn close(&self) -> String {
        self.node.cast::<ast::MathDelimited>()
            .map(|d| d.close().to_untyped().text().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "MathDelimited(...)".to_string() }
}

// ── MathFrac ─────────────────────────────────────────────────────────────────

#[pyclass(name = "MathFrac")]
#[derive(Clone)]
pub struct PyMathFrac { pub node: SyntaxNode }

#[pymethods]
impl PyMathFrac {
    #[getter]
    pub fn num(&self) -> PySyntaxNode {
        self.node.cast::<ast::MathFrac>()
            .map(|f| untyped(f.num()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn denom(&self) -> PySyntaxNode {
        self.node.cast::<ast::MathFrac>()
            .map(|f| untyped(f.denom()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "MathFrac(...)".to_string() }
}

// ── MathRoot ─────────────────────────────────────────────────────────────────

#[pyclass(name = "MathRoot")]
#[derive(Clone)]
pub struct PyMathRoot { pub node: SyntaxNode }

#[pymethods]
impl PyMathRoot {
    #[getter]
    pub fn index(&self) -> Option<u8> {
        self.node.cast::<ast::MathRoot>().and_then(|r| r.index())
    }
    #[getter]
    pub fn radicand(&self) -> PySyntaxNode {
        self.node.cast::<ast::MathRoot>()
            .map(|r| untyped(r.radicand()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "MathRoot(...)".to_string() }
}

// ── MathPrimes ───────────────────────────────────────────────────────────────

#[pyclass(name = "MathPrimes")]
#[derive(Clone)]
pub struct PyMathPrimes { pub node: SyntaxNode }

#[pymethods]
impl PyMathPrimes {
    #[getter]
    pub fn count(&self) -> usize {
        self.node.cast::<ast::MathPrimes>().map(|p| p.count()).unwrap_or(0)
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("MathPrimes(count={})", self.count()) }
}

pub fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEquation>()?;
    m.add_class::<PyMathIdent>()?;
    m.add_class::<PyMathAttach>()?;
    m.add_class::<PyMathDelimited>()?;
    m.add_class::<PyMathFrac>()?;
    m.add_class::<PyMathRoot>()?;
    m.add_class::<PyMathPrimes>()?;
    Ok(())
}
