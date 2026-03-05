use pyo3::prelude::*;
use crate::span::PySpan;

#[pyclass(name = "SyntaxError")]
#[derive(Clone)]
pub struct PySyntaxError {
    pub span: PySpan,
    pub message: String,
    pub hints: Vec<String>,
}

impl PySyntaxError {
    pub fn from_rust(e: &typst_syntax::SyntaxError) -> Self {
        PySyntaxError {
            span: PySpan { span: e.span },
            message: e.message.to_string(),
            hints: e.hints.iter().map(|h| h.to_string()).collect(),
        }
    }
}

#[pymethods]
impl PySyntaxError {
    #[getter]
    pub fn span(&self) -> PySpan {
        self.span.clone()
    }

    #[getter]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[getter]
    pub fn hints(&self) -> Vec<String> {
        self.hints.clone()
    }

    pub fn __repr__(&self) -> String {
        format!("SyntaxError({:?})", self.message)
    }
}
