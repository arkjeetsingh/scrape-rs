//! DOM tree representation and manipulation.
//!
//! This module provides the internal DOM tree structure used by scrape-core.
//! It uses arena-based allocation for efficient memory management.
//!
//! # Architecture
//!
//! - **Arena**: Memory arena for efficient node allocation
//! - **Tree**: Tree structure for parent/child relationships
//! - **Node**: Individual DOM node types (element, text, comment, etc.)
//!
//! The DOM is designed to be:
//! - Memory efficient through arena allocation
//! - Fast to traverse with direct parent/child/sibling links
//! - Safe through Rust's ownership system

// TODO: implement DOM submodules
// mod arena;
// mod node;
// mod tree;

/// A node ID in the DOM tree.
///
/// This is an opaque handle to a node in the arena.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    /// Creates a new node ID.
    #[must_use]
    pub(crate) const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Types of nodes in the DOM tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    /// Document root.
    Document,
    /// Element node (e.g., `<div>`, `<span>`).
    Element {
        /// Tag name.
        name: String,
    },
    /// Text node.
    Text {
        /// Text content.
        content: String,
    },
    /// Comment node.
    Comment {
        /// Comment content.
        content: String,
    },
    /// Document type declaration.
    Doctype,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id() {
        let id = NodeId::new(42);
        assert_eq!(id.index(), 42);
    }

    #[test]
    fn test_node_type() {
        let element = NodeType::Element {
            name: "div".to_string(),
        };
        assert!(matches!(element, NodeType::Element { name } if name == "div"));

        let text = NodeType::Text {
            content: "Hello".to_string(),
        };
        assert!(matches!(text, NodeType::Text { content } if content == "Hello"));
    }
}
