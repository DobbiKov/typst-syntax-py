use pyo3::prelude::*;
use typst_syntax::Tag;
use crate::linked_node::PyLinkedNode;

#[pyclass(name = "Tag")]
#[derive(Clone, PartialEq, Eq)]
pub struct PyTag {
    pub tag: Tag,
}

fn pt(t: Tag) -> PyTag {
    PyTag { tag: t }
}

#[pymethods]
impl PyTag {
    #[getter]
    pub fn name(&self) -> String {
        format!("{:?}", self.tag)
    }

    pub fn __repr__(&self) -> String {
        format!("Tag.{:?}", self.tag)
    }

    pub fn __eq__(&self, other: &PyTag) -> bool {
        self.tag == other.tag
    }

    pub fn __hash__(&self) -> u8 {
        self.tag as u8
    }

    // Class attributes for every Tag variant
    #[classattr] fn COMMENT() -> PyTag { pt(Tag::Comment) }
    #[classattr] fn PUNCTUATION() -> PyTag { pt(Tag::Punctuation) }
    #[classattr] fn ESCAPE() -> PyTag { pt(Tag::Escape) }
    #[classattr] fn STRONG() -> PyTag { pt(Tag::Strong) }
    #[classattr] fn EMPH() -> PyTag { pt(Tag::Emph) }
    #[classattr] fn LINK() -> PyTag { pt(Tag::Link) }
    #[classattr] fn RAW() -> PyTag { pt(Tag::Raw) }
    #[classattr] fn LABEL() -> PyTag { pt(Tag::Label) }
    #[classattr] fn REF() -> PyTag { pt(Tag::Ref) }
    #[classattr] fn HEADING() -> PyTag { pt(Tag::Heading) }
    #[classattr] fn LIST_MARKER() -> PyTag { pt(Tag::ListMarker) }
    #[classattr] fn LIST_TERM() -> PyTag { pt(Tag::ListTerm) }
    #[classattr] fn MATH_DELIMITER() -> PyTag { pt(Tag::MathDelimiter) }
    #[classattr] fn MATH_OPERATOR() -> PyTag { pt(Tag::MathOperator) }
    #[classattr] fn KEYWORD() -> PyTag { pt(Tag::Keyword) }
    #[classattr] fn OPERATOR() -> PyTag { pt(Tag::Operator) }
    #[classattr] fn NUMBER() -> PyTag { pt(Tag::Number) }
    #[classattr] fn STRING() -> PyTag { pt(Tag::String) }
    #[classattr] fn FUNCTION() -> PyTag { pt(Tag::Function) }
    #[classattr] fn INTERPOLATED() -> PyTag { pt(Tag::Interpolated) }
    #[classattr] fn ERROR() -> PyTag { pt(Tag::Error) }
}

#[pyfunction(name = "highlight")]
pub fn py_highlight(node: &PyLinkedNode) -> Option<PyTag> {
    let ln = node.navigate_to_node()?;
    typst_syntax::highlight(&ln).map(|t| PyTag { tag: t })
}

#[pyfunction(name = "highlight_html")]
pub fn py_highlight_html(source: &crate::source::PySource) -> String {
    typst_syntax::highlight_html(source.source.root())
}
