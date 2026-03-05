use pyo3::prelude::*;

pub mod syntax_kind;
pub mod span;
pub mod error;
pub mod syntax_node;
pub mod source;
pub mod linked_node;
pub mod highlight;
pub mod utils;
pub mod ast;

use syntax_node::PySyntaxNode;

#[pyfunction]
fn parse(text: &str) -> PySyntaxNode {
    PySyntaxNode { node: typst_syntax::parse(text) }
}

#[pyfunction]
fn parse_code(text: &str) -> PySyntaxNode {
    PySyntaxNode { node: typst_syntax::parse_code(text) }
}

#[pyfunction]
fn parse_math(text: &str) -> PySyntaxNode {
    PySyntaxNode { node: typst_syntax::parse_math(text) }
}

#[pymodule]
fn _typst_syntax(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Core types
    m.add_class::<syntax_kind::PySyntaxKind>()?;
    m.add_class::<span::PySpan>()?;
    m.add_class::<span::PyFileId>()?;
    m.add_class::<error::PySyntaxError>()?;
    m.add_class::<syntax_node::PySyntaxNode>()?;
    m.add_class::<source::PySource>()?;
    m.add_class::<source::PyLines>()?;
    m.add_class::<linked_node::PyLinkedNode>()?;
    m.add_class::<highlight::PyTag>()?;

    // Parse functions
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(parse_code, m)?)?;
    m.add_function(wrap_pyfunction!(parse_math, m)?)?;

    // Highlight functions
    m.add_function(wrap_pyfunction!(highlight::py_highlight, m)?)?;
    m.add_function(wrap_pyfunction!(highlight::py_highlight_html, m)?)?;

    // Utility functions
    m.add_function(wrap_pyfunction!(utils::py_is_ident, m)?)?;
    m.add_function(wrap_pyfunction!(utils::py_split_newlines, m)?)?;
    m.add_function(wrap_pyfunction!(utils::py_link_prefix, m)?)?;

    // AST classes registered directly (Python-side ast/ package re-exports them)
    ast::markup::register(m.py(), m)?;
    ast::code::register(m.py(), m)?;
    ast::math::register(m.py(), m)?;
    m.add_function(wrap_pyfunction!(ast::cast_node, m)?)?;

    Ok(())
}
