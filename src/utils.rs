use pyo3::prelude::*;

#[pyfunction(name = "is_ident")]
pub fn py_is_ident(s: &str) -> bool {
    typst_syntax::is_ident(s)
}

#[pyfunction(name = "split_newlines")]
pub fn py_split_newlines(text: &str) -> Vec<String> {
    typst_syntax::split_newlines(text)
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Returns (matched_prefix, had_protocol) from `link_prefix(text)`.
#[pyfunction(name = "link_prefix")]
pub fn py_link_prefix(text: &str) -> (String, bool) {
    let (prefix, has_proto) = typst_syntax::link_prefix(text);
    (prefix.to_string(), has_proto)
}
