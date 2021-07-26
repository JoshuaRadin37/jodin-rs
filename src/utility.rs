//! Contains helper traits and function that can be used anywhere in the project

/// Create a string with an ident of some size,
pub fn with_indent<S: AsRef<str>>(s: S, indent: u32) -> String {
    format!("{}{}", "  ".repeat(indent as usize), s.as_ref())
}

/// A trait that helps a type represent itself as a tree
pub trait Tree {
    /// Gets the direct children of this node.
    fn direct_children(&self) -> Vec<&Self>;

    /// Returns the tree as a vector using a prefix-transversal
    fn children_prefix(&self) -> Vec<&Self> {
        let mut ret = vec![self];
        for child in self.direct_children() {
            ret.extend(child.children_prefix());
        }
        ret
    }

    /// Returns the tree as a vector using a postfix-transversal
    fn children_postfix(&self) -> Vec<&Self> {
        let mut ret = vec![];
        for child in self.direct_children() {
            ret.extend(child.children_postfix());
        }
        ret.push(self);
        ret
    }
}
