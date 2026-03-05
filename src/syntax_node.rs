use pyo3::prelude::*;
use typst_syntax::SyntaxNode;
use crate::syntax_kind::PySyntaxKind;
use crate::span::PySpan;
use crate::error::PySyntaxError;

#[pyclass(name = "SyntaxNode")]
#[derive(Clone)]
pub struct PySyntaxNode {
    pub node: SyntaxNode,
}

#[pymethods]
impl PySyntaxNode {
    pub fn kind(&self) -> PySyntaxKind {
        PySyntaxKind { kind: self.node.kind() }
    }

    /// Leaf text (empty string for inner nodes).
    pub fn text(&self) -> String {
        self.node.text().to_string()
    }

    /// Reconstructed source text (works for both leaf and inner nodes).
    pub fn full_text(&self) -> String {
        collect_text(&self.node)
    }

    pub fn children(&self) -> Vec<PySyntaxNode> {
        self.node
            .children()
            .map(|c| PySyntaxNode { node: c.clone() })
            .collect()
    }

    #[getter]
    pub fn span(&self) -> PySpan {
        PySpan { span: self.node.span() }
    }

    pub fn is_leaf(&self) -> bool {
        self.node.children().next().is_none()
    }

    pub fn is_erroneous(&self) -> bool {
        self.node.erroneous()
    }

    pub fn errors(&self) -> Vec<PySyntaxError> {
        self.node.errors().iter().map(PySyntaxError::from_rust).collect()
    }

    pub fn __len__(&self) -> usize {
        self.node.len()
    }

    pub fn __repr__(&self) -> String {
        let text = self.node.text();
        if text.is_empty() {
            format!("SyntaxNode({})", self.node.kind().name())
        } else {
            let preview: String = text.chars().take(20).collect();
            format!("SyntaxNode({}, {:?})", self.node.kind().name(), preview)
        }
    }
}

/// Recursively collect all leaf text in DFS order.
pub fn collect_text(node: &SyntaxNode) -> String {
    let leaf = node.text();
    if !leaf.is_empty() {
        return leaf.to_string();
    }
    node.children().map(collect_text).collect()
}
