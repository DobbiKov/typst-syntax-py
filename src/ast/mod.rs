pub mod markup;
pub mod code;
pub mod math;

use pyo3::prelude::*;
use typst_syntax::SyntaxKind as SK;
use crate::syntax_node::PySyntaxNode;

/// Try to cast a SyntaxNode to the most specific typed AST wrapper.
/// Returns None if the node kind has no dedicated wrapper.
#[pyfunction]
pub fn cast_node(node: &PySyntaxNode) -> Option<PyObject> {
    let n = &node.node;
    let kind = n.kind();
    Python::with_gil(|py| -> Option<PyObject> {
        let obj: PyObject = match kind {
            // Markup
            SK::Markup       => markup::PyMarkup { node: n.clone() }.into_py(py),
            SK::Heading      => markup::PyHeading { node: n.clone() }.into_py(py),
            SK::Strong       => markup::PyStrong { node: n.clone() }.into_py(py),
            SK::Emph         => markup::PyEmph { node: n.clone() }.into_py(py),
            SK::Raw          => markup::PyRaw { node: n.clone() }.into_py(py),
            SK::Link         => markup::PyLink { node: n.clone() }.into_py(py),
            SK::Label        => markup::PyLabel { node: n.clone() }.into_py(py),
            SK::Ref          => markup::PyRef { node: n.clone() }.into_py(py),
            SK::ListItem     => markup::PyListItem { node: n.clone() }.into_py(py),
            SK::EnumItem     => markup::PyEnumItem { node: n.clone() }.into_py(py),
            SK::TermItem     => markup::PyTermItem { node: n.clone() }.into_py(py),
            SK::SmartQuote   => markup::PySmartQuote { node: n.clone() }.into_py(py),
            SK::Escape       => markup::PyEscape { node: n.clone() }.into_py(py),
            SK::Shorthand    => markup::PyShorthand { node: n.clone() }.into_py(py),
            // Code
            SK::Ident        => code::PyIdent { node: n.clone() }.into_py(py),
            SK::Bool         => code::PyBool { node: n.clone() }.into_py(py),
            SK::Int          => code::PyInt { node: n.clone() }.into_py(py),
            SK::Float        => code::PyFloat { node: n.clone() }.into_py(py),
            SK::Str          => code::PyStr { node: n.clone() }.into_py(py),
            SK::Numeric      => code::PyNumeric { node: n.clone() }.into_py(py),
            SK::Array        => code::PyArray { node: n.clone() }.into_py(py),
            SK::Dict         => code::PyDict { node: n.clone() }.into_py(py),
            SK::Named        => code::PyNamed { node: n.clone() }.into_py(py),
            SK::FieldAccess  => code::PyFieldAccess { node: n.clone() }.into_py(py),
            SK::FuncCall     => code::PyFuncCall { node: n.clone() }.into_py(py),
            SK::Closure      => code::PyClosure { node: n.clone() }.into_py(py),
            SK::LetBinding   => code::PyLetBinding { node: n.clone() }.into_py(py),
            SK::SetRule      => code::PySetRule { node: n.clone() }.into_py(py),
            SK::ShowRule     => code::PyShowRule { node: n.clone() }.into_py(py),
            SK::Conditional  => code::PyConditional { node: n.clone() }.into_py(py),
            SK::WhileLoop    => code::PyWhileLoop { node: n.clone() }.into_py(py),
            SK::ForLoop      => code::PyForLoop { node: n.clone() }.into_py(py),
            SK::ModuleImport => code::PyModuleImport { node: n.clone() }.into_py(py),
            SK::ModuleInclude=> code::PyModuleInclude { node: n.clone() }.into_py(py),
            SK::Binary       => code::PyBinary { node: n.clone() }.into_py(py),
            SK::Unary        => code::PyUnary { node: n.clone() }.into_py(py),
            SK::FuncReturn   => code::PyFuncReturn { node: n.clone() }.into_py(py),
            SK::LoopBreak    => code::PyLoopBreak { node: n.clone() }.into_py(py),
            SK::LoopContinue => code::PyLoopContinue { node: n.clone() }.into_py(py),
            SK::Parenthesized=> code::PyParenthesized { node: n.clone() }.into_py(py),
            SK::CodeBlock    => code::PyCodeBlock { node: n.clone() }.into_py(py),
            SK::ContentBlock => code::PyContentBlock { node: n.clone() }.into_py(py),
            // Math
            SK::Equation     => math::PyEquation { node: n.clone() }.into_py(py),
            SK::MathIdent    => math::PyMathIdent { node: n.clone() }.into_py(py),
            SK::MathAttach   => math::PyMathAttach { node: n.clone() }.into_py(py),
            SK::MathDelimited=> math::PyMathDelimited { node: n.clone() }.into_py(py),
            SK::MathFrac     => math::PyMathFrac { node: n.clone() }.into_py(py),
            SK::MathRoot     => math::PyMathRoot { node: n.clone() }.into_py(py),
            SK::MathPrimes   => math::PyMathPrimes { node: n.clone() }.into_py(py),
            _ => return None,
        };
        Some(obj)
    })
}

pub fn register(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    markup::register(py, m)?;
    code::register(py, m)?;
    math::register(py, m)?;
    m.add_function(wrap_pyfunction!(cast_node, m)?)?;
    Ok(())
}
