//! The standard identifier is used for every declaration within Jodin, from local declarations to
//! class definitions to modules.

use std::array::IntoIter;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FromIterator;

/// Contains this id and an optional parent
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    parent: Option<Box<Identifier>>,
    id: String,
}

impl Identifier {
    /// Creates an identifier from an array.
    ///
    /// # Arguments
    ///
    /// * `array`: An array of string-like values
    ///
    /// returns: Identifier
    ///
    /// # Example
    ///
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// let id = Identifier::from_array(["hello", "world"]);
    /// ```
    pub fn from_array<S: AsRef<str>, const N: usize>(array: [S; N]) -> Self {
        Self::from_iter(IntoIter::new(array))
    }

    /// The parent of the identifier.
    pub fn parent(&self) -> &Option<Box<Identifier>> {
        &self.parent
    }
    /// Consumes the identifier to get it's maybe parent
    pub fn into_parent(self) -> Option<Identifier> {
        self.parent.map(|r| *r)
    }

    /// Gets the "this" part of the identifier.
    ///
    /// # Example
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// let id = Identifier::from_array(["hello", "world"]);
    /// assert_eq!(id, "world");
    /// ```
    pub fn this(&self) -> &str {
        &self.id
    }

    /// Gets the ultimate parent of the identifier, or itself if it has not parent
    ///
    /// # Examples
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// let id = Identifier::from_array(["lvl1", "lvl2", "lvl3"]);
    /// assert_eq!(id, "lvl3");
    /// assert_eq!(id.parent().unwrap().this(), "lvl2");
    /// assert_eq!(id.highest_parent().this(), "lvl3");
    ///
    /// let id = Identifier::from("id");
    /// assert_eq!(id.highest_parent().this(), "id");
    /// ```
    pub fn highest_parent(&self) -> &Identifier {
        match &self.parent {
            None => self,
            Some(parent) => parent.highest_parent(),
        }
    }

    fn apply_to_highest_parent_mut<F: Fn(&mut Identifier)>(&mut self, f: F) {
        if let Some(parent) = self.parent.as_mut() {
            parent.apply_to_highest_parent_mut(f);
        } else {
            f(self)
        }
    }

    /// Creates a new identifier that's the concatenation of two Identifier-like values.
    ///
    /// # Arguments
    ///
    /// * `parent`: The parent identifier
    /// * `child`: The child identifier
    ///
    /// returns: the concatenated Identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// let example1 = Identifier::new_concat(Identifier::from("hello"), Identifier::from("world2"));
    /// let example2 = Identifier::new_concat("hello", "world");
    ///
    /// assert_eq!(example1, example2);
    /// ```
    pub fn new_concat<N1: Into<Identifier>, N2: Into<Identifier>>(
        parent: N1,
        child: N2,
    ) -> Identifier {
        let mut child = child.into();
        let parent = parent.into();
        child.apply_to_highest_parent_mut(move |highest| {
            highest.parent = Some(Box::new(parent.clone()));
        });

        child
    }

    fn from_iterator_backwards<S: AsRef<str>, T: IntoIterator<Item = S>>(iter: T) -> Option<Self> {
        let mut iter = iter.into_iter();
        if let Some(last) = iter.next() {
            let parent = Self::from_iterator_backwards(iter).map(|id| Box::new(id));
            Some(Self {
                parent,
                id: last.as_ref().to_string(),
            })
        } else {
            None
        }
    }

    /// Removes the highest parent from this identifier, and returns the remaining value if
    /// it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// let id = Identifier::from_array(["lvl1", "lvl2", "lvl3"]);
    /// let new_id = id.strip_highest_parent();
    /// assert_eq!(id, Identifier::from_array(["lvl2", "lvl3"]));
    /// ```
    pub fn strip_highest_parent(self) -> Option<Identifier> {
        let Identifier { parent, id } = self;
        match parent {
            None => return None,
            Some(p) => Some(Identifier {
                parent: p.strip_highest_parent().map(|id| Box::new(id)),
                id,
            }),
        }
    }
}

impl<S: AsRef<str>> From<S> for Identifier {
    fn from(s: S) -> Self {
        Identifier {
            parent: None,
            id: s.as_ref().to_string(),
        }
    }
}

impl From<&Identifier> for Identifier {
    fn from(i: &Identifier) -> Self {
        i.clone()
    }
}

impl<S: AsRef<str>> FromIterator<S> for Identifier {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let mut vec = Vec::from_iter(iter);
        vec.reverse();
        Identifier::from_iterator_backwards(vec).unwrap()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            write!(f, "{}::{}", parent, self.id)
        } else {
            write!(f, "{}", self.id)
        }
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<S: AsRef<str>> PartialEq<S> for Identifier {
    fn eq(&self, other: &S) -> bool {
        let string = other.as_ref().to_string();
        let this_string: String = self.to_string();
        string == this_string
    }
}

impl From<Identifier> for String {
    fn from(i: Identifier) -> Self {
        format!("{}", i)
    }
}

/// Represents that this type has an associated identifier, and thus can be namespaced
pub trait Namespaced {
    /// Gets the identifier that this value has.
    fn get_identifier(&self) -> &Identifier;
}

impl Namespaced for Identifier {
    fn get_identifier(&self) -> &Identifier {
        self
    }
}

impl IntoIterator for &Identifier {
    type Item = String;
    type IntoIter = IdentifierIterator;

    fn into_iter(self) -> Self::IntoIter {
        let mut buffer = VecDeque::new();
        let mut ptr = Some(self.clone());
        while let Some(val) = ptr {
            let Identifier { parent, id } = val;
            buffer.push_back(id);
            ptr = parent.map(|b| *b)
        }
        IdentifierIterator { id: buffer }
    }
}

/// Iterates through the parts of an identifier
pub struct IdentifierIterator {
    id: VecDeque<String>,
}

impl Iterator for IdentifierIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.pop_back()
    }
}

/// A wrapper type that can attach an identifier to a type that doesn't implement [Namespaced].
///
/// [Namespaced]: Namespaced
pub struct Identifiable<T> {
    id: Identifier,
    /// The associated value
    pub val: T,
}

impl<T> Identifiable<T> {
    /// Creates a new instance of an Identifiable.
    ///
    /// # Arguments
    ///
    /// * `id`: The id to associate with the value
    /// * `val`: The value
    ///
    /// returns: Identifiable<T>
    pub fn new<I: Into<Identifier>>(id: I, val: T) -> Self {
        Identifiable { id: id.into(), val }
    }
}

impl<T> Namespaced for Identifiable<T> {
    fn get_identifier(&self) -> &Identifier {
        &self.id
    }
}

#[cfg(test)]
mod test {
    use crate::core::identifier::Identifier;
    use std::iter::FromIterator;

    #[test]
    fn id_from_iter() {
        let id = Identifier::from_iter(&["std", "iter", "FromIterator"]);
        assert_eq!(id.to_string(), "std::iter::FromIterator");
    }
}
