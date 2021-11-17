//! Contains helper traits and function that can be used anywhere in the project

use std::fmt::{Display, Formatter};

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

pub fn node_count(tree: &impl Tree) -> usize {
    let mut output = 1;
    for child in tree.direct_children() {
        output += node_count(child);
    }
    output
}

/// Box a value
pub trait IntoBox: Sized {
    /// Put this value into the heap
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<T: Sized> IntoBox for T {}

/// A human readable interpretation of a value. This can be the same as display, but may
/// be different. For example, a date object can have a display implementation
/// showing the the exact date, while the human readable version would show the approximate
/// difference in time (eg: "7 seconds ago")
pub trait HumanReadable: ToString {
    /// The human readable string
    fn human_readable(&self) -> String {
        self.to_string()
    }
}

/// A wrapper to get human writable bytes
#[derive(Debug)]
pub struct Bytes(usize);

impl Display for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

impl HumanReadable for Bytes {
    fn human_readable(&self) -> String {
        let (power, units) = match &self.0 {
            0..=1023 => (0, "B"),
            1024..=1048575 => (1, "KB"),
            1048576..=1073741823 => (2, "MB"),
            1073741824..=1099511627775 => (3, "GB"),
            _ => (4, "TB"),
        };

        let value = self.0 / (1024usize.pow(power));
        format!("{} {}", value, units)
    }
}

impl Bytes {
    /// Creates a new bytes object
    pub fn new(bytes: usize) -> Self {
        Bytes(bytes)
    }
}

pub trait Visitor<'t, Visited, Output> {
    fn visit(&'t self, environment: &'t Visited) -> Output;
}

/// An acceptor takes in an object
pub trait Acceptor<'a, Visitor> {
    type Output;
    fn accept(&'a self, visitor: Visitor) -> Self::Output;
}

pub trait AcceptorMut<'a, Visitor> : Acceptor<'a, Visitor> {
    type MutOutput;
    fn accept_mut(&'a mut self, visitor: Visitor) -> Self::MutOutput;
}


pub trait Flatten<T, E> {
    fn flatten(this: Self) -> Result<T, E>;
}

impl<T, E> Flatten<T, E> for Result<Result<T, E>, E> {
    fn flatten(this: Self) -> Result<T, E> {
        match this {
            Ok(Ok(t)) => Ok(t),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(e),
        }
    }
}

pub fn usum<F : Fn(usize) -> usize>(from: usize, to: usize, f: F) -> usize {
    (from..=to)
        .into_iter()
        .map(|index| f(index))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_readable_bytes() {
        assert_eq!(Bytes::new(92).human_readable(), "92 B");
        assert_eq!(Bytes::new(1023).human_readable(), "1023 B");
        assert_eq!(Bytes::new(1024).human_readable(), "1 KB");
        assert_eq!(Bytes::new(1024 * 1024 - 1).human_readable(), "1023 KB");
        assert_eq!(Bytes::new(1024 * 1024).human_readable(), "1 MB");
        assert_eq!(
            Bytes::new(1024 * 1024 * 1024 - 1).human_readable(),
            "1023 MB"
        );
        assert_eq!(Bytes::new(1024 * 1024 * 1024).human_readable(), "1 GB");
        assert_eq!(
            Bytes::new(1024 * 1024 * 1024 * 1024 - 1).human_readable(),
            "1023 GB"
        );
        assert_eq!(
            Bytes::new(1024 * 1024 * 1024 * 1024).human_readable(),
            "1 TB"
        );
    }

    #[test]
    fn flatten() {
        let mut big_error: Result<Result<(), ()>, ()> = Ok(Err(()));
        assert_eq!(Flatten::flatten(big_error), Err(()));
        big_error = Ok(Ok(()));
        assert_eq!(Flatten::flatten(big_error), Ok(()));
    }
}
