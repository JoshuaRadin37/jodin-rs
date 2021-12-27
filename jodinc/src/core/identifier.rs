//! The standard identifier is used for every declaration within Jodin, from local declarations to
//! class definitions to modules.

use crate::utility::IntoBox;
use itertools::Itertools;
use std::array::IntoIter;
use std::borrow::Borrow;
use std::cmp::{min, Ordering};
use std::collections::{Bound, VecDeque};
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::iter::{FromIterator, FusedIterator};
use std::ops::{Add, Div, Index, Range, RangeBounds, Shl, Shr};

/// Contains this id and an optional parent
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    parent: Option<Box<Identifier>>,
    id: String,
}

impl Identifier {
    /// Create a new identifier from any type that can be converted into an identifier
    #[inline]
    pub fn new<I: Into<Identifier>>(into: I) -> Self {
        into.into()
    }

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
    /// use jodinc::ore::identifier::Identifier;
    /// let id = Identifier::from_array(["hello", "world"]);
    /// ```
    #[deprecated = "No longer necessary, as arrays not directly implement IntoIterator"]
    pub fn from_array<S: AsRef<str>, const N: usize>(array: [S; N]) -> Self {
        Self::from_iter(IntoIter::new(array))
    }

    /// Creates an empty identifier, should only ever be used to represent missing data
    pub fn empty() -> Self {
        Self {
            parent: None,
            id: "".to_string(),
        }
    }

    /// Check if this identifier is empty
    pub fn is_empty(&self) -> bool {
        self.id.is_empty() && self.parent.is_none()
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
    /// use jodinc::ore::identifier::Identifier;
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
    /// use jodinc::ore::identifier::Identifier;
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
    /// use jodinc::ore::identifier::Identifier;
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
    /// use jodinc::ore::identifier::Identifier;
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

        if child.is_empty() {
            return parent;
        } else if parent.is_empty() {
            return child;
        }

        child.apply_to_highest_parent_mut(move |highest| {
            highest.parent = Some(Box::new(parent.clone()));
        });

        child
    }

    fn from_iterator_backwards<S: AsRef<str>, T: IntoIterator<Item = S>>(iter: T) -> Option<Self> {
        let mut iter = iter.into_iter();
        if let Some(last) = iter.next() {
            let parent = Self::from_iterator_backwards(iter).map(|id| Box::new(id));
            match parent {
                None => Some(Identifier::new(last)),
                Some(parent) => Some(Self::new_concat(*parent, last)),
            }
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
    /// use jodinc::ore::identifier::Identifier;
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

    /// Gets the length of the identifier
    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        1 + self.parent.as_ref().map_or(0, |p| p.len())
    }

    /// Creates an iterator that iterates over the different identifiers of these two
    /// identifiers
    ///
    /// # Example
    /// ```
    /// use jodinc::core::identifier::Identifier;
    /// use jodinc::id;
    /// let left = id!(n1, n2, n3, n4);
    /// let right = id!(n1, n2, n5);
    /// let mut diff = Identifier::diff(&left, &right);
    ///
    /// assert_eq!(diff.next(), Some((Some(id!(n1, n2, n3)), Some(id!(n1, n2, n5)))));
    /// ```
    pub fn diff(left: &Self, right: &Self) -> IdentifierDiffIterator {
        let mut break_point = None;
        let mut count = 0;
        let mut zipped = left.iter().zip(right.iter()).enumerate();
        for (index, (left, right)) in zipped {
            if left != right {
                break_point = Some(index);
                break;
            }
            count += 1;
        }
        let diff_start = break_point.unwrap_or(count);
        IdentifierDiffIterator::new(left.clone(), right.clone(), diff_start)
    }

    /// Gets a sub identifier of an identifier
    ///
    /// # Example
    /// ```
    /// use jodinc::id;
    /// let id = id!(n1, n2, n3, n4);
    /// assert_eq!(id.sub_identifier(1..3), Some(id!(n2, n3)));
    /// assert_eq!(id.sub_identifier(1..=3), Some(id!(n2, n3, n4)));
    /// assert_eq!(id.sub_identifier(1..), Some(id!(n2, n3, n4)));
    /// ```
    pub fn sub_identifier<R: RangeBounds<usize>>(&self, range: R) -> Option<Self> {
        let start = match range.start_bound() {
            Bound::Included(i) => (*i),
            Bound::Excluded(i) => i + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => (*i),
            Bound::Excluded(i) => *i - 1,
            Bound::Unbounded => self.len() - 1,
        };
        if start >= self.len() || end >= self.len() {
            return None;
        }
        let ids: Vec<_> = self.into_iter().collect();
        Some(Identifier::from_iter(&ids[start..=end]))
    }

    /// Gets an OS compatible version of the toString of this identifier
    pub fn os_compat(&self) -> Option<OsString> {
        let as_string = self.to_string().replace("::", "_");

        OsString::try_from(as_string).ok()
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

// impl<S: AsRef<str>> FromIterator<S> for Identifier {
//     fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
//         let mut vec = Vec::from_iter(iter);
//         vec.reverse();
//         Identifier::from_iterator_backwards(vec).unwrap()
//     }
// }
impl<I: Into<Identifier>> FromIterator<I> for Identifier {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut vec: Vec<String> = iter.into_iter().flat_map(|i| i.into().iter()).collect();
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
        write!(f, "\"{}\"", self)
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
    /// use jodinc::ore::identifier::Identifier;
    /// let id1 = Identifier::from("Hello");
    /// let id2 = Identifier::from("Goodbye");
    /// assert_eq!(id1.partial_cmp(&id2), None);
    /// ```
    /// Some(Greater) => The `self` identifier is a super set of `other`
    /// ```
    /// use jodinc::ore::identifier::Identifier;
    /// use std::cmp::Ordering::Greater;
    /// let id1 = Identifier::from("Hello");
    /// let id2 = Identifier::new_concat("Hello", "World");
    /// assert_eq!(id1.partial_cmp(&id2), Some(Greater));
    /// ```
    /// Some(Equal) => The `self` identifier and the `other` identifier represent the same set
    /// ```
    /// use jodinc::ore::identifier::Identifier;
    /// use std::cmp::Ordering::Equal;
    /// let id1 = Identifier::from("Hello");
    /// let id2 = id1.clone();
    /// assert_eq!(id1.partial_cmp(&id2), Some(Equal));
    /// ```
    /// Some(Less) => The `self` identifier is a sub set of `other`
    /// ```
    /// use jodinc::ore::identifier::Identifier;
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

impl Div for Identifier {
    type Output = Identifier;

    fn div(self, rhs: Self) -> Self::Output {
        Identifier::new_concat(self, rhs)
    }
}

impl Div for &Identifier {
    type Output = Identifier;

    fn div(self, rhs: Self) -> Self::Output {
        Identifier::new_concat(self, rhs)
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
#[derive(Debug)]
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

pub struct IdentifierDiffIterator {
    left: Identifier,
    right: Identifier,
    diff_start: usize,
    index: usize,
}

impl IdentifierDiffIterator {
    pub fn new(left: Identifier, right: Identifier, diff_start: usize) -> Self {
        IdentifierDiffIterator {
            left,
            right,
            diff_start,
            index: 0,
        }
    }
}

impl Iterator for IdentifierDiffIterator {
    type Item = (Option<Identifier>, Option<Identifier>);

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.left.sub_identifier(..=self.diff_start + self.index);
        let right = self.right.sub_identifier(..=self.diff_start + self.index);
        match (left, right) {
            (None, None) => None,
            other => {
                self.index += 1;
                Some(other)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left_size = self.left.len() - self.diff_start;
        let right_size = self.right.len() - self.diff_start;
        let max = usize::max(left_size, right_size);
        (max, Some(max))
    }
}

impl FusedIterator for IdentifierDiffIterator {}

impl ExactSizeIterator for IdentifierDiffIterator {}

#[cfg(test)]
mod test {
    use crate::core::identifier::{Identifier, IdentifierChain};
    use itertools::Diff;
    use std::cmp::Ordering;
    use std::iter::FromIterator;

    #[test]
    fn id_from_iter() {
        let id = Identifier::from_iter(&["std", "iter", "FromIterator"]);
        assert_eq!(id.to_string(), "std::iter::FromIterator");
        assert_eq!(id!(std::iter::FromIterator), id);
        assert_eq!(id!(std::iter::FromIterator), id!(std, iter, FromIterator));
        assert_eq!(
            id!(std::iter::FromIterator),
            id!(id!("std"::"iter"), "FromIterator")
        );
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

    #[test]
    fn empty_id_concats() {
        assert_eq!(id!(Identifier::empty(), "test"), id!("test"));
        assert_eq!(id!("test", Identifier::empty()), id!("test"));
    }

    #[test]
    fn sub_range() {
        let id = id!(n1, n2, n3, n4);
        assert_eq!(id.sub_identifier(1..3), Some(id!(n2, n3)));
        assert_eq!(id.sub_identifier(1..=3), Some(id!(n2, n3, n4)));
        assert_eq!(id.sub_identifier(1..), Some(id!(n2, n3, n4)));
        assert_eq!(
            id.sub_identifier(0..2),
            id.sub_identifier(..2),
            "{:?} is equivalent to {:?}",
            0..2,
            ..2
        );
        assert_eq!(id.sub_identifier(..), Some(id.clone()));
        assert_eq!(id.sub_identifier(4..), None);
    }

    #[test]
    fn id_diff() {
        let left = id!(n1::n2::n3);
        let right = id!(n1::n4::n5);
        let mut diff = Identifier::diff(&left, &right);
        assert_eq!(diff.len(), 2);
        assert_eq!(diff.next().unwrap(), (Some(id!(n1::n2)), Some(id!(n1::n4))));
        assert_eq!(
            diff.next().unwrap(),
            (Some(id!(n1::n2::n3)), Some(id!(n1::n4::n5)))
        );
        assert_eq!(diff.next(), None);
    }
}
