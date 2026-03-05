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

// ── Ident ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Ident")]
#[derive(Clone)]
pub struct PyIdent { pub node: SyntaxNode }

#[pymethods]
impl PyIdent {
    pub fn as_str(&self) -> String {
        self.node.cast::<ast::Ident>().map(|i| i.as_str().to_string()).unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Ident({:?})", self.as_str()) }
}

// ── Bool ──────────────────────────────────────────────────────────────────────

#[pyclass(name = "Bool")]
#[derive(Clone)]
pub struct PyBool { pub node: SyntaxNode }

#[pymethods]
impl PyBool {
    pub fn get(&self) -> bool {
        self.node.cast::<ast::Bool>().map(|b| b.get()).unwrap_or(false)
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Bool({})", self.get()) }
}

// ── Int ───────────────────────────────────────────────────────────────────────

#[pyclass(name = "Int")]
#[derive(Clone)]
pub struct PyInt { pub node: SyntaxNode }

#[pymethods]
impl PyInt {
    pub fn get(&self) -> i64 {
        self.node.cast::<ast::Int>().map(|i| i.get()).unwrap_or(0)
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Int({})", self.get()) }
}

// ── Float ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Float")]
#[derive(Clone)]
pub struct PyFloat { pub node: SyntaxNode }

#[pymethods]
impl PyFloat {
    pub fn get(&self) -> f64 {
        self.node.cast::<ast::Float>().map(|f| f.get()).unwrap_or(0.0)
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Float({})", self.get()) }
}

// ── Str ───────────────────────────────────────────────────────────────────────

#[pyclass(name = "Str")]
#[derive(Clone)]
pub struct PyStr { pub node: SyntaxNode }

#[pymethods]
impl PyStr {
    pub fn get(&self) -> String {
        self.node.cast::<ast::Str>().map(|s| s.get().to_string()).unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Str({:?})", self.get()) }
}

// ── Numeric ───────────────────────────────────────────────────────────────────

#[pyclass(name = "Numeric")]
#[derive(Clone)]
pub struct PyNumeric { pub node: SyntaxNode }

#[pymethods]
impl PyNumeric {
    pub fn v(&self) -> f64 {
        self.node.cast::<ast::Numeric>().map(|n| n.get().0).unwrap_or(0.0)
    }
    pub fn unit(&self) -> String {
        self.node.cast::<ast::Numeric>().map(|n| format!("{:?}", n.get().1)).unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Numeric({} {})", self.v(), self.unit()) }
}

// ── Array ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Array")]
#[derive(Clone)]
pub struct PyArray { pub node: SyntaxNode }

#[pymethods]
impl PyArray {
    pub fn items(&self) -> Vec<PySyntaxNode> {
        self.node.cast::<ast::Array>()
            .map(|a| a.items().map(|i| untyped(i)).collect())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Array(...)".to_string() }
}

// ── Dict ──────────────────────────────────────────────────────────────────────

#[pyclass(name = "Dict")]
#[derive(Clone)]
pub struct PyDict { pub node: SyntaxNode }

#[pymethods]
impl PyDict {
    pub fn items(&self) -> Vec<PySyntaxNode> {
        self.node.cast::<ast::Dict>()
            .map(|d| d.items().map(|i| untyped(i)).collect())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Dict(...)".to_string() }
}

// ── Named ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Named")]
#[derive(Clone)]
pub struct PyNamed { pub node: SyntaxNode }

#[pymethods]
impl PyNamed {
    #[getter]
    pub fn name(&self) -> PySyntaxNode {
        self.node.cast::<ast::Named>()
            .map(|n| untyped(n.name()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn expr(&self) -> PySyntaxNode {
        self.node.cast::<ast::Named>()
            .map(|n| untyped(n.expr()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Named(...)".to_string() }
}

// ── FieldAccess ───────────────────────────────────────────────────────────────

#[pyclass(name = "FieldAccess")]
#[derive(Clone)]
pub struct PyFieldAccess { pub node: SyntaxNode }

#[pymethods]
impl PyFieldAccess {
    #[getter]
    pub fn target(&self) -> PySyntaxNode {
        self.node.cast::<ast::FieldAccess>()
            .map(|f| untyped(f.target()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn field(&self) -> String {
        self.node.cast::<ast::FieldAccess>()
            .map(|f| f.field().as_str().to_string())
            .unwrap_or_default()
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("FieldAccess(field={:?})", self.field()) }
}

// ── FuncCall ──────────────────────────────────────────────────────────────────

#[pyclass(name = "FuncCall")]
#[derive(Clone)]
pub struct PyFuncCall { pub node: SyntaxNode }

#[pymethods]
impl PyFuncCall {
    #[getter]
    pub fn callee(&self) -> PySyntaxNode {
        self.node.cast::<ast::FuncCall>()
            .map(|f| untyped(f.callee()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn args(&self) -> PySyntaxNode {
        self.node.cast::<ast::FuncCall>()
            .map(|f| untyped(f.args()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "FuncCall(...)".to_string() }
}

// ── Closure ───────────────────────────────────────────────────────────────────

#[pyclass(name = "Closure")]
#[derive(Clone)]
pub struct PyClosure { pub node: SyntaxNode }

#[pymethods]
impl PyClosure {
    #[getter]
    pub fn params(&self) -> PySyntaxNode {
        self.node.cast::<ast::Closure>()
            .map(|c| untyped(c.params()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::Closure>()
            .map(|c| untyped(c.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Closure(...)".to_string() }
}

// ── LetBinding ────────────────────────────────────────────────────────────────

#[pyclass(name = "LetBinding")]
#[derive(Clone)]
pub struct PyLetBinding { pub node: SyntaxNode }

#[pymethods]
impl PyLetBinding {
    #[getter]
    pub fn kind(&self) -> String {
        self.node.cast::<ast::LetBinding>()
            .map(|l| format!("{:?}", l.kind()))
            .unwrap_or_default()
    }
    #[getter]
    pub fn init(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::LetBinding>()
            .and_then(|l| l.init())
            .map(|e| untyped(e))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "LetBinding(...)".to_string() }
}

// ── SetRule ───────────────────────────────────────────────────────────────────

#[pyclass(name = "SetRule")]
#[derive(Clone)]
pub struct PySetRule { pub node: SyntaxNode }

#[pymethods]
impl PySetRule {
    #[getter]
    pub fn target(&self) -> PySyntaxNode {
        self.node.cast::<ast::SetRule>()
            .map(|s| untyped(s.target()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn args(&self) -> PySyntaxNode {
        self.node.cast::<ast::SetRule>()
            .map(|s| untyped(s.args()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn condition(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::SetRule>()
            .and_then(|s| s.condition())
            .map(|e| untyped(e))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "SetRule(...)".to_string() }
}

// ── ShowRule ──────────────────────────────────────────────────────────────────

#[pyclass(name = "ShowRule")]
#[derive(Clone)]
pub struct PyShowRule { pub node: SyntaxNode }

#[pymethods]
impl PyShowRule {
    #[getter]
    pub fn selector(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::ShowRule>()
            .and_then(|s| s.selector())
            .map(|e| untyped(e))
    }
    #[getter]
    pub fn transform(&self) -> PySyntaxNode {
        self.node.cast::<ast::ShowRule>()
            .map(|s| untyped(s.transform()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "ShowRule(...)".to_string() }
}

// ── Conditional ───────────────────────────────────────────────────────────────

#[pyclass(name = "Conditional")]
#[derive(Clone)]
pub struct PyConditional { pub node: SyntaxNode }

#[pymethods]
impl PyConditional {
    #[getter]
    pub fn condition(&self) -> PySyntaxNode {
        self.node.cast::<ast::Conditional>()
            .map(|c| untyped(c.condition()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn if_body(&self) -> PySyntaxNode {
        self.node.cast::<ast::Conditional>()
            .map(|c| untyped(c.if_body()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn else_body(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::Conditional>()
            .and_then(|c| c.else_body())
            .map(|e| untyped(e))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Conditional(...)".to_string() }
}

// ── WhileLoop ─────────────────────────────────────────────────────────────────

#[pyclass(name = "WhileLoop")]
#[derive(Clone)]
pub struct PyWhileLoop { pub node: SyntaxNode }

#[pymethods]
impl PyWhileLoop {
    #[getter]
    pub fn condition(&self) -> PySyntaxNode {
        self.node.cast::<ast::WhileLoop>()
            .map(|w| untyped(w.condition()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::WhileLoop>()
            .map(|w| untyped(w.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "WhileLoop(...)".to_string() }
}

// ── ForLoop ───────────────────────────────────────────────────────────────────

#[pyclass(name = "ForLoop")]
#[derive(Clone)]
pub struct PyForLoop { pub node: SyntaxNode }

#[pymethods]
impl PyForLoop {
    #[getter]
    pub fn pattern(&self) -> PySyntaxNode {
        self.node.cast::<ast::ForLoop>()
            .map(|f| untyped(f.pattern()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn iterable(&self) -> PySyntaxNode {
        self.node.cast::<ast::ForLoop>()
            .map(|f| untyped(f.iterable()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::ForLoop>()
            .map(|f| untyped(f.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "ForLoop(...)".to_string() }
}

// ── ModuleImport ──────────────────────────────────────────────────────────────

#[pyclass(name = "ModuleImport")]
#[derive(Clone)]
pub struct PyModuleImport { pub node: SyntaxNode }

#[pymethods]
impl PyModuleImport {
    #[getter]
    pub fn source(&self) -> PySyntaxNode {
        self.node.cast::<ast::ModuleImport>()
            .map(|m| untyped(m.source()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn imports(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::ModuleImport>()
            .and_then(|m| m.imports())
            .and_then(|i| match i {
                ast::Imports::Items(items) => Some(untyped(items)),
                ast::Imports::Wildcard => None,
            })
    }
    #[getter]
    pub fn new_name(&self) -> Option<String> {
        self.node.cast::<ast::ModuleImport>()
            .and_then(|m| m.new_name())
            .map(|i| i.as_str().to_string())
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "ModuleImport(...)".to_string() }
}

// ── ModuleInclude ─────────────────────────────────────────────────────────────

#[pyclass(name = "ModuleInclude")]
#[derive(Clone)]
pub struct PyModuleInclude { pub node: SyntaxNode }

#[pymethods]
impl PyModuleInclude {
    #[getter]
    pub fn source(&self) -> PySyntaxNode {
        self.node.cast::<ast::ModuleInclude>()
            .map(|m| untyped(m.source()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "ModuleInclude(...)".to_string() }
}

// ── Binary ────────────────────────────────────────────────────────────────────

#[pyclass(name = "Binary")]
#[derive(Clone)]
pub struct PyBinary { pub node: SyntaxNode }

#[pymethods]
impl PyBinary {
    #[getter]
    pub fn lhs(&self) -> PySyntaxNode {
        self.node.cast::<ast::Binary>()
            .map(|b| untyped(b.lhs()))
            .unwrap_or_else(|| node(&self.node))
    }
    #[getter]
    pub fn op(&self) -> String {
        self.node.cast::<ast::Binary>()
            .map(|b| format!("{:?}", b.op()))
            .unwrap_or_default()
    }
    #[getter]
    pub fn rhs(&self) -> PySyntaxNode {
        self.node.cast::<ast::Binary>()
            .map(|b| untyped(b.rhs()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Binary(op={:?})", self.op()) }
}

// ── Unary ─────────────────────────────────────────────────────────────────────

#[pyclass(name = "Unary")]
#[derive(Clone)]
pub struct PyUnary { pub node: SyntaxNode }

#[pymethods]
impl PyUnary {
    #[getter]
    pub fn op(&self) -> String {
        self.node.cast::<ast::Unary>()
            .map(|u| format!("{:?}", u.op()))
            .unwrap_or_default()
    }
    #[getter]
    pub fn expr(&self) -> PySyntaxNode {
        self.node.cast::<ast::Unary>()
            .map(|u| untyped(u.expr()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { format!("Unary(op={:?})", self.op()) }
}

// ── FuncReturn ────────────────────────────────────────────────────────────────

#[pyclass(name = "FuncReturn")]
#[derive(Clone)]
pub struct PyFuncReturn { pub node: SyntaxNode }

#[pymethods]
impl PyFuncReturn {
    #[getter]
    pub fn body(&self) -> Option<PySyntaxNode> {
        self.node.cast::<ast::FuncReturn>()
            .and_then(|r| r.body())
            .map(|e| untyped(e))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "FuncReturn(...)".to_string() }
}

// ── Parenthesized ─────────────────────────────────────────────────────────────

#[pyclass(name = "Parenthesized")]
#[derive(Clone)]
pub struct PyParenthesized { pub node: SyntaxNode }

#[pymethods]
impl PyParenthesized {
    #[getter]
    pub fn expr(&self) -> PySyntaxNode {
        self.node.cast::<ast::Parenthesized>()
            .map(|p| untyped(p.expr()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "Parenthesized(...)".to_string() }
}

// ── CodeBlock ─────────────────────────────────────────────────────────────────

#[pyclass(name = "CodeBlock")]
#[derive(Clone)]
pub struct PyCodeBlock { pub node: SyntaxNode }

#[pymethods]
impl PyCodeBlock {
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::CodeBlock>()
            .map(|c| untyped(c.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "CodeBlock(...)".to_string() }
}

// ── ContentBlock ──────────────────────────────────────────────────────────────

#[pyclass(name = "ContentBlock")]
#[derive(Clone)]
pub struct PyContentBlock { pub node: SyntaxNode }

#[pymethods]
impl PyContentBlock {
    #[getter]
    pub fn body(&self) -> PySyntaxNode {
        self.node.cast::<ast::ContentBlock>()
            .map(|c| untyped(c.body()))
            .unwrap_or_else(|| node(&self.node))
    }
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "ContentBlock(...)".to_string() }
}

// ── LoopBreak / LoopContinue ──────────────────────────────────────────────────

#[pyclass(name = "LoopBreak")]
#[derive(Clone)]
pub struct PyLoopBreak { pub node: SyntaxNode }
#[pymethods]
impl PyLoopBreak {
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "LoopBreak".to_string() }
}

#[pyclass(name = "LoopContinue")]
#[derive(Clone)]
pub struct PyLoopContinue { pub node: SyntaxNode }
#[pymethods]
impl PyLoopContinue {
    pub fn to_untyped(&self) -> PySyntaxNode { node(&self.node) }
    pub fn __repr__(&self) -> String { "LoopContinue".to_string() }
}

pub fn register(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIdent>()?;
    m.add_class::<PyBool>()?;
    m.add_class::<PyInt>()?;
    m.add_class::<PyFloat>()?;
    m.add_class::<PyStr>()?;
    m.add_class::<PyNumeric>()?;
    m.add_class::<PyArray>()?;
    m.add_class::<PyDict>()?;
    m.add_class::<PyNamed>()?;
    m.add_class::<PyFieldAccess>()?;
    m.add_class::<PyFuncCall>()?;
    m.add_class::<PyClosure>()?;
    m.add_class::<PyLetBinding>()?;
    m.add_class::<PySetRule>()?;
    m.add_class::<PyShowRule>()?;
    m.add_class::<PyConditional>()?;
    m.add_class::<PyWhileLoop>()?;
    m.add_class::<PyForLoop>()?;
    m.add_class::<PyModuleImport>()?;
    m.add_class::<PyModuleInclude>()?;
    m.add_class::<PyBinary>()?;
    m.add_class::<PyUnary>()?;
    m.add_class::<PyFuncReturn>()?;
    m.add_class::<PyParenthesized>()?;
    m.add_class::<PyCodeBlock>()?;
    m.add_class::<PyContentBlock>()?;
    m.add_class::<PyLoopBreak>()?;
    m.add_class::<PyLoopContinue>()?;
    Ok(())
}
