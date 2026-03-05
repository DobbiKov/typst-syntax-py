use pyo3::prelude::*;
use typst_syntax::{FileId, Span};

#[pyclass(name = "Span")]
#[derive(Clone)]
pub struct PySpan {
    pub span: Span,
}

#[pymethods]
impl PySpan {
    #[getter]
    pub fn is_detached(&self) -> bool {
        self.span.is_detached()
    }

    #[getter]
    pub fn file_id(&self) -> Option<PyFileId> {
        self.span.id().map(|id| PyFileId { id })
    }

    /// Returns (start, end) byte range, or None if detached.
    pub fn range(&self) -> Option<(usize, usize)> {
        self.span.range().map(|r| (r.start, r.end))
    }

    pub fn __repr__(&self) -> String {
        if self.span.is_detached() {
            "Span(detached)".to_string()
        } else {
            format!("Span({:?})", self.span)
        }
    }
}

#[pyclass(name = "FileId")]
#[derive(Clone)]
pub struct PyFileId {
    pub id: FileId,
}

#[pymethods]
impl PyFileId {
    /// The package spec string (e.g. "@preview/foo:0.1.0"), or None for local files.
    #[getter]
    pub fn package(&self) -> Option<String> {
        self.id.package().map(|p| p.to_string())
    }

    /// The virtual path within the package/project.
    #[getter]
    pub fn vpath(&self) -> String {
        self.id.vpath().as_rooted_path().display().to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("FileId({})", self.vpath())
    }
}
