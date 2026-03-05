use pyo3::prelude::*;
use typst_syntax::{Source, Lines};
use crate::syntax_node::PySyntaxNode;
use crate::linked_node::PyLinkedNode;
use crate::span::PySpan;

#[pyclass(name = "Source")]
pub struct PySource {
    pub source: Source,
}

#[pymethods]
impl PySource {
    /// Create a detached (in-memory) source from text.
    #[staticmethod]
    pub fn detached(text: &str) -> Self {
        PySource { source: Source::detached(text) }
    }

    /// The root CST node.
    pub fn root(&self) -> PySyntaxNode {
        PySyntaxNode { node: self.source.root().clone() }
    }

    /// The full source text.
    pub fn text(&self) -> &str {
        self.source.text()
    }

    /// Get a Lines helper for byte/line/column conversions.
    pub fn lines(&self) -> PyLines {
        PyLines { lines: Lines::new(self.source.text().to_string()) }
    }

    /// Find the LinkedNode spanning the given Span, or None.
    pub fn find(&self, span: &PySpan) -> Option<PyLinkedNode> {
        let ln = self.source.find(span.span)?;
        // Build the path from root to this node using offset
        let root = self.source.root().clone();
        let offset = ln.offset();
        PyLinkedNode::from_root_and_offset(root, offset)
    }

    /// Return (start, end) byte range for a Span, or None.
    pub fn range(&self, span: &PySpan) -> Option<(usize, usize)> {
        self.source.range(span.span).map(|r| (r.start, r.end))
    }

    /// Edit the source: replace bytes `start..end` with `replacement`.
    /// Returns the new (start, end) of the affected range.
    pub fn edit(&mut self, start: usize, end: usize, replacement: &str) -> (usize, usize) {
        let range = self.source.edit(start..end, replacement);
        (range.start, range.end)
    }

    pub fn __repr__(&self) -> String {
        let preview: String = self.source.text().chars().take(40).collect();
        format!("Source({:?})", preview)
    }
}

/// Line/column/byte conversion helper.
#[pyclass(name = "Lines")]
pub struct PyLines {
    pub lines: Lines<String>,
}

#[pymethods]
impl PyLines {
    pub fn byte_to_line(&self, byte: usize) -> Option<usize> {
        self.lines.byte_to_line(byte)
    }

    pub fn byte_to_column(&self, byte: usize) -> Option<usize> {
        self.lines.byte_to_column(byte)
    }

    pub fn byte_to_line_column(&self, byte: usize) -> Option<(usize, usize)> {
        self.lines.byte_to_line_column(byte)
    }

    pub fn line_to_byte(&self, line: usize) -> Option<usize> {
        self.lines.line_to_byte(line)
    }

    pub fn line_column_to_byte(&self, line: usize, col: usize) -> Option<usize> {
        self.lines.line_column_to_byte(line, col)
    }

    pub fn line_to_range(&self, line: usize) -> Option<(usize, usize)> {
        self.lines.line_to_range(line).map(|r| (r.start, r.end))
    }

    pub fn byte_to_utf16(&self, byte: usize) -> Option<usize> {
        self.lines.byte_to_utf16(byte)
    }

    pub fn utf16_to_byte(&self, utf16: usize) -> Option<usize> {
        self.lines.utf16_to_byte(utf16)
    }

    pub fn len_bytes(&self) -> usize {
        self.lines.len_bytes()
    }

    pub fn len_lines(&self) -> usize {
        self.lines.len_lines()
    }

    pub fn len_utf16(&self) -> usize {
        self.lines.len_utf16()
    }
}
