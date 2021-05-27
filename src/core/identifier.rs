use pest::Span;

use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::iter::FromIterator;
use std::ops::Deref;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    parent: Option<Box<Identifier>>,
    id: String,
}

impl Identifier {
    pub fn parent(&self) -> &Option<Box<Identifier>> {
        &self.parent
    }
    pub fn into_parent(self) -> Option<Identifier> {
        self.parent.map(|r| *r)
    }

    pub fn this(&self) -> &str {
        &self.id
    }

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

    pub fn with_parent<N1: Into<Identifier>, N2: Into<Identifier>>(
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

    pub fn strip_highest_parent(mut self) -> Option<Identifier> {
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

pub trait Namespaced {
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

pub struct IdentifierIterator {
    id: VecDeque<String>,
}

impl Iterator for IdentifierIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.id.pop_back()
    }
}

pub struct Identifiable<T> {
    id: Identifier,
    val: T,
}

impl<T> Identifiable<T> {
    pub fn new<I: Into<Identifier>>(id: I, val: T) -> Self {
        Identifiable { id: id.into(), val }
    }

    pub fn val(&self) -> &T {
        &self.val
    }
    pub fn val_mut(&mut self) -> &mut T {
        &mut self.val
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
