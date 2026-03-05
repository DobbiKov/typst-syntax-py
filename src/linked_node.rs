use pyo3::prelude::*;
use typst_syntax::{LinkedNode, SyntaxNode};
use crate::syntax_node::PySyntaxNode;

/// Python wrapper for LinkedNode.
///
/// Stores an owned clone of the root SyntaxNode plus the child-index path
/// from the root to this node. Methods reconstruct a `LinkedNode<'_>` on demand.
#[pyclass(name = "LinkedNode")]
#[derive(Clone)]
pub struct PyLinkedNode {
    /// The root of the whole tree (Arc-backed, cheap clone).
    pub root: SyntaxNode,
    /// Child-index path from root to this node (empty = root).
    pub path: Vec<usize>,
}

impl PyLinkedNode {
    /// Construct from a root node and a target byte offset.
    /// Walks the tree to find the node whose span contains `offset`.
    pub fn from_root_and_offset(root: SyntaxNode, offset: usize) -> Option<Self> {
        let ln = LinkedNode::new(&root);
        let target = ln.leaf_at(offset, typst_syntax::Side::After)
            .or_else(|| ln.leaf_at(offset.saturating_sub(1), typst_syntax::Side::Before))?;
        // Rebuild path by comparing offsets / spans
        let mut path = Vec::new();
        let mut current = LinkedNode::new(&root);
        'outer: loop {
            if std::ptr::eq(current.get(), target.get()) {
                break;
            }
            let children: Vec<_> = current.children().collect();
            for (i, child) in children.iter().enumerate() {
                let child_range = child.range();
                if child_range.contains(&offset) || child_range.start == offset {
                    path.push(i);
                    current = children.into_iter().nth(i).unwrap();
                    continue 'outer;
                }
            }
            break;
        }
        Some(PyLinkedNode { root, path })
    }

    /// Reconstruct a LinkedNode by walking the stored path.
    pub fn navigate_to_node<'a>(&'a self) -> Option<LinkedNode<'a>> {
        let root_ln = LinkedNode::new(&self.root);
        if self.path.is_empty() {
            return Some(root_ln);
        }
        let mut current = root_ln;
        for &idx in &self.path {
            let children: Vec<_> = current.children().collect();
            current = children.into_iter().nth(idx)?;
        }
        Some(current)
    }
}

#[pymethods]
impl PyLinkedNode {
    /// The underlying SyntaxNode.
    pub fn get(&self) -> PySyntaxNode {
        match self.navigate_to_node() {
            Some(ln) => PySyntaxNode { node: ln.get().clone() },
            None => PySyntaxNode { node: self.root.clone() },
        }
    }

    /// Byte offset of this node within the source.
    pub fn offset(&self) -> usize {
        self.navigate_to_node().map(|n| n.offset()).unwrap_or(0)
    }

    /// Byte range (start, end) of this node.
    pub fn range(&self) -> (usize, usize) {
        self.navigate_to_node()
            .map(|n| { let r = n.range(); (r.start, r.end) })
            .unwrap_or((0, 0))
    }

    /// Index of this node among its siblings.
    pub fn index(&self) -> usize {
        self.navigate_to_node().map(|n| n.index()).unwrap_or(0)
    }

    /// Child LinkedNodes.
    pub fn children(&self) -> Vec<PyLinkedNode> {
        match self.navigate_to_node() {
            None => vec![],
            Some(ln) => {
                let count = ln.get().children().count();
                (0..count).map(|i| {
                    let mut child_path = self.path.clone();
                    child_path.push(i);
                    PyLinkedNode { root: self.root.clone(), path: child_path }
                }).collect()
            }
        }
    }

    /// Parent node, or None if this is the root.
    pub fn parent(&self) -> Option<PyLinkedNode> {
        if self.path.is_empty() {
            return None;
        }
        let mut parent_path = self.path.clone();
        parent_path.pop();
        Some(PyLinkedNode { root: self.root.clone(), path: parent_path })
    }

    /// Previous sibling, or None.
    pub fn prev_sibling(&self) -> Option<PyLinkedNode> {
        let idx = *self.path.last()?;
        if idx == 0 { return None; }
        let mut sib_path = self.path.clone();
        *sib_path.last_mut()? = idx - 1;
        Some(PyLinkedNode { root: self.root.clone(), path: sib_path })
    }

    /// Next sibling, or None.
    pub fn next_sibling(&self) -> Option<PyLinkedNode> {
        let idx = *self.path.last()?;
        let mut sib_path = self.path.clone();
        *sib_path.last_mut()? = idx + 1;
        // Validate the sibling exists
        let _ = PyLinkedNode { root: self.root.clone(), path: sib_path.clone() }
            .navigate_to_node()?;
        Some(PyLinkedNode { root: self.root.clone(), path: sib_path })
    }

    /// Previous leaf (skipping trivia).
    pub fn prev_leaf(&self) -> Option<PyLinkedNode> {
        let ln = self.navigate_to_node()?;
        let prev = ln.prev_leaf()?;
        let offset = prev.offset();
        PyLinkedNode::from_root_and_offset(self.root.clone(), offset)
    }

    /// Next leaf (skipping trivia).
    pub fn next_leaf(&self) -> Option<PyLinkedNode> {
        let ln = self.navigate_to_node()?;
        let next = ln.next_leaf()?;
        let offset = next.offset();
        PyLinkedNode::from_root_and_offset(self.root.clone(), offset)
    }

    /// Leftmost leaf under this node.
    pub fn leftmost_leaf(&self) -> Option<PyLinkedNode> {
        let ln = self.navigate_to_node()?;
        let leaf = ln.leftmost_leaf()?;
        let offset = leaf.offset();
        PyLinkedNode::from_root_and_offset(self.root.clone(), offset)
    }

    /// Rightmost leaf under this node.
    pub fn rightmost_leaf(&self) -> Option<PyLinkedNode> {
        let ln = self.navigate_to_node()?;
        let leaf = ln.rightmost_leaf()?;
        let offset = leaf.offset();
        PyLinkedNode::from_root_and_offset(self.root.clone(), offset)
    }

    pub fn __repr__(&self) -> String {
        let kind = self.navigate_to_node()
            .map(|n| n.get().kind().name().to_string())
            .unwrap_or_else(|| "?".to_string());
        format!("LinkedNode({} @ {})", kind, self.offset())
    }
}
