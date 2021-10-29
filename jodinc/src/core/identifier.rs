//! The standard identifier is used for every declaration within Jodin, from local declarations to
//! class definitions to modules.

use crate::utility::IntoBox;
use itertools::Itertools;
use std::array::IntoIter;
use std::cmp::{min, Ordering};
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FromIterator;
use std::ops::{Add, Shl, Shr};

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
    #[deprecated = "No longer necessary, as arrays not directly implement IntoIterator"]
    pub fn from_array<S: AsRef<str>, const N: usize>(array: [S; N]) -> Self {
        Self::from_iter(IntoIter::new(array))
    }

    /// The parent of the identifier.
    pub fn parent(&self) -> Option<&Identifier> {
        self.parent.as_deref()
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
    /// use std::iter::FromIterator;
    /// let id = Identifier::from_iter(["hello", "world"]);
    /// assert_eq!(id, "hello::world");
    /// ```
    pub fn this(&self) -> &str {
        &self.id
    }

    /// Gets the "this" part of the identifier as an identifier.
    ///
    /// # Example
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// use std::iter::FromIterator;
    /// let id = Identifier::from_iter(["hello", "world"]);
    /// assert_eq!(id, "hello::world");
    /// ```
    pub fn this_as_id(&self) -> Identifier {
        Identifier::from(&self.id)
    }

    /// Gets the ultimate parent of the identifier, or itself if it has not parent
    ///
    /// # Examples
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// use std::iter::FromIterator;
    /// let id = Identifier::from_iter(["lvl1", "lvl2", "lvl3"]);
    /// assert_eq!(id, "lvl1::lvl2::lvl3");
    /// assert_eq!(id.parent().unwrap().this(), "lvl2");
    /// assert_eq!(id.highest_parent().this(), "lvl1");
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
    /// let example1 = Identifier::new_concat(Identifier::from("hello"), Identifier::from("world"));
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
    /// let new_id = id.strip_highest_parent().unwrap();
    /// assert_eq!(new_id, Identifier::from_array(["lvl2", "lvl3"]));
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

    /// Creates an iterator through the different parts of this identifier
    pub fn iter(&self) -> IdentifierIterator {
        IntoIterator::into_iter(self)
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

impl PartialEq<&str> for &Identifier {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
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

impl PartialOrd for Identifier {
    /// The comparison is based off of if the namespaces are subsets of each other.
    ///
    /// # Examples
    /// None => Neither this or the other identifier represents subsets of each other
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// let id1 = Identifier::from("Hello");
    /// let id2 = Identifier::from("Goodbye");
    /// assert_eq!(id1.partial_cmp(&id2), None);
    /// ```
    /// Some(Greater) => The `self` identifier is a super set of `other`
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// use std::cmp::Ordering::Greater;
    /// let id1 = Identifier::from("Hello");
    /// let id2 = Identifier::new_concat("Hello", "World");
    /// assert_eq!(id1.partial_cmp(&id2), Some(Greater));
    /// ```
    /// Some(Equal) => The `self` identifier and the `other` identifier represent the same set
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// use std::cmp::Ordering::Equal;
    /// let id1 = Identifier::from("Hello");
    /// let id2 = id1.clone();
    /// assert_eq!(id1.partial_cmp(&id2), Some(Equal));
    /// ```
    /// Some(Less) => The `self` identifier is a sub set of `other`
    /// ```
    /// use jodin_rs::core::identifier::Identifier;
    /// use std::cmp::Ordering::Less;
    /// let id1 = Identifier::new_concat("Hello", "World");
    /// let id2 = Identifier::from("Hello");
    /// assert_eq!(id1.partial_cmp(&id2), Some(Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let me: Vec<String> = self.iter().collect();
        let other: Vec<String> = other.iter().collect();
        let shorter = min(me.len(), other.len());
        if &me[..shorter] != &other[..shorter] {
            return None;
        }
        me.partial_cmp(&other).map(|o| o.reverse())
    }
}

impl Add for Identifier {
    type Output = Identifier;

    fn add(self, rhs: Self) -> Self::Output {
        Identifier::new_concat(self, rhs)
    }
}

impl Add for &Identifier {
    type Output = Identifier;

    fn add(self, rhs: Self) -> Self::Output {
        Identifier::new_concat(self, rhs)
    }
}

impl Shl for &Identifier {
    type Output = Identifier;

    fn shl(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

impl Shr for &Identifier {
    type Output = Identifier;

    fn shr(self, rhs: Self) -> Self::Output {
        rhs + self
    }
}

impl Shl for Identifier {
    type Output = Identifier;

    fn shl(self, rhs: Self) -> Self::Output {
        self + rhs
    }
}

impl Shr for Identifier {
    type Output = Identifier;

    fn shr(self, rhs: Self) -> Self::Output {
        rhs + self
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

#[derive(Debug, Clone)]
pub struct IdentifierChain {
    this: Identifier,
    next: Option<Box<IdentifierChain>>,
}

impl IdentifierChain {
    pub fn new<I: Into<Identifier>>(id: I) -> Self {
        Self {
            this: id.into(),
            next: None,
        }
    }

    pub fn with_child<I: Into<Identifier>>(mut self, id: I) -> Self {
        self.add_child(id);
        self
    }

    pub fn add_child<I: Into<Identifier>>(&mut self, id: I) {
        self._add_child(&id.into());
    }

    fn _add_child(&mut self, id: &Identifier) {
        if self.has_next() {
            self.next(move |child| child._add_child(id));
        } else {
            self.next = Some(IdentifierChain::new(id).boxed());
        }
    }

    fn has_next(&self) -> bool {
        self.next.is_some()
    }

    fn next<T, F: Fn(&mut IdentifierChain) -> T>(&mut self, closure: F) -> Option<T> {
        match self.next.as_deref_mut() {
            None => None,
            Some(inner) => Some(closure(inner)),
        }
    }

    pub fn iter(&self) -> IdentifierChainIterator {
        (&self).into_iter()
    }
}

impl<'a> IntoIterator for &'a IdentifierChain {
    type Item = &'a Identifier;
    type IntoIter = IdentifierChainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IdentifierChainIterator { chain: Some(self) }
    }
}

impl Display for IdentifierChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().join("->"))
    }
}

#[derive(Debug)]
pub struct IdentifierChainIterator<'i> {
    chain: Option<&'i IdentifierChain>,
}

impl<'i> Iterator for IdentifierChainIterator<'i> {
    type Item = &'i Identifier;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = std::mem::replace(&mut self.chain, None);
        match prev {
            None => None,
            Some(IdentifierChain { this, next }) => {
                let next = next.as_deref();
                let dest = &mut self.chain;
                std::mem::replace(dest, next);
                Some(this)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::identifier::{Identifier, IdentifierChain};
    use std::cmp::Ordering;
    use std::iter::FromIterator;

    #[test]
    fn id_from_iter() {
        let id = Identifier::from_iter(&["std", "iter", "FromIterator"]);
        assert_eq!(id.to_string(), "std::iter::FromIterator");
    }

    #[test]
    fn id_ops() {
        let id1 = Identifier::from("hello");
        let id2 = Identifier::from("world");
        assert_eq!(&id1 + &id2, Identifier::new_concat("hello", "world"));
        assert_eq!(&id1 << &id2, Identifier::new_concat("hello", "world"));
        assert_eq!(&id1 >> &id2, Identifier::new_concat("world", "hello"));
    }

    #[test]
    fn id_comparisons() {
        let id1 = Identifier::from("hello");
        let id2 = id1.clone();
        assert_eq!(id1.partial_cmp(&id2), Some(Ordering::Equal));

        let id2 = Identifier::from("goodbye");
        assert_eq!(id1.partial_cmp(&id2), None);

        let id2 = id1.clone();
        let id1 = Identifier::from_iter(["hello", "world"]);
        assert_eq!(
            id1.partial_cmp(&id2),
            Some(Ordering::Less),
            "Id1 should be less because it's more specific than id2"
        );

        let id2 = Identifier::from_iter(["hello", "world", "earth"]);
        assert_eq!(
            id1.partial_cmp(&id2),
            Some(Ordering::Greater),
            "Id1 should be greater because its less specific than id2"
        );

        let id1 = Identifier::from_iter(["hello", "world", "mars"]);
        assert_eq!(id1.partial_cmp(&id2),
        None,
        "Although they are both the same length, because neither is a subset of the other they can't be compared")
    }

    #[test]
    fn id_chain() {
        let chain = IdentifierChain::new("hello")
            .with_child("world")
            .with_child("goodbye");
        assert_eq!(chain.to_string(), "hello->world->goodbye");
    }
}
