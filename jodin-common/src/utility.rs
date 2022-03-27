//! Contains helper traits and function that can be used anywhere in the project

use smallvec::SmallVec;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::process::Output;

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

pub trait AcceptorMut<'a, Visitor>: Acceptor<'a, Visitor> {
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

pub fn usum<F: Fn(usize) -> usize>(from: usize, to: usize, f: F) -> usize {
    (from..=to).into_iter().map(|index| f(index)).sum()
}

pub struct LoggedWrite<W: io::Write> {
    log_level: log::Level,
    writer: W,
    buffer: SmallVec<[u8; 256]>,
    message: String,
}

impl<W: io::Write> LoggedWrite<W> {
    pub fn new(log_level: log::Level, writer: W, prefix: impl Into<Option<String>>) -> Self {
        LoggedWrite {
            log_level,
            writer,
            buffer: SmallVec::new(),
            message: prefix.into().unwrap_or("".to_string()),
        }
    }
}

impl<W: io::Write> Write for LoggedWrite<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        let mut split = self.buffer.split(|&s| s == b'\n');
        for line in split {
            let string = String::from_utf8_lossy(line);
            if self.message.is_empty() {
                log!(self.log_level, "{}", string);
            } else {
                log!(self.log_level, "{}: {}", self.message, string);
            }
        }
        if let Some((pos, _)) = self
            .buffer
            .iter()
            .enumerate()
            .rev()
            .find(|&(pos, &b)| b == b'\n')
        {
            self.buffer.drain(0..=pos);
        }
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// Path utils give some more utility functions for paths
pub trait PathUtils: AsRef<Path> {
    /// Gets the absolute path of this path
    ///
    /// This is functionality identical to [`Path::canonicalize`](std::path::Path::canonicalize)
    fn absolute_path(&self) -> Result<PathBuf, io::Error>;

    /// Whether this path is absolute or not.
    fn is_absolute(&self) -> bool;

    /// Whether this path is relative or not.
    fn is_relative(&self) -> bool;

    /// Resolves this path relative to a given path.
    fn resolve<P: PathUtils + Sized>(&self, other: P) -> Result<PathBuf, io::Error> {
        let self_abs = self.absolute_path()?;
        let other_abs = other.absolute_path()?;

        <&PathBuf>::find_common_parent(&self_abs, &other_abs)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "No common parent, can't resolve", // I think this error can only occur on systems with multiple volumes
                )
            })
            .and_then(|common_parent| {
                let self_front_stripped = self_abs.strip_prefix(&common_parent).unwrap(); // we know this is parent.
                let other_front_stripped = other_abs.strip_prefix(&common_parent).unwrap();
                let back_count = other_front_stripped.iter().count();

                match back_count {
                    0 => Ok(self_front_stripped.to_path_buf()),
                    more => {
                        let mut output = self_front_stripped.to_path_buf();
                        for _ in 0..more {
                            output = {
                                let mut path = PathBuf::from("..");
                                path.push(output);
                                path
                            };
                        }
                        Ok(output)
                    }
                }
            })
    }

    /// resolves this path if it's relative and gives the absolute path
    fn full_resolve<P: PathUtils + Sized>(&self, other: P) -> Result<PathBuf, io::Error> {
        if self.is_absolute() {
            Ok(self.as_ref().to_path_buf())
        } else {
            self.resolve(other)
        }
    }

    /// Relatives a given path relative to this.
    ///
    /// Implemented as the reverse of [`resolve`](PathUtils::resolve)
    fn relativize<P: PathUtils + Sized>(&self, other: P) -> Result<PathBuf, io::Error>
    where
        Self: Sized,
    {
        other.resolve(self)
    }

    /// Gets the common parent of two absolute paths
    ///
    /// # Panic
    /// Will panic if either path is not absolute
    fn find_common_parent<P>(path1: Self, path2: P) -> Option<PathBuf>
    where
        P: PathUtils + Sized,
        Self: Sized,
    {
        if !path1.is_absolute() || !path2.is_absolute() {
            return None;
        }

        let path1_ancestors = path1.as_ref().ancestors().collect::<Vec<_>>();
        let path2_ancestors = path2.as_ref().ancestors().collect::<Vec<_>>();

        let mut path1_iter = path1_ancestors.into_iter().rev();
        let mut path2_iter = path2_ancestors.into_iter().rev();

        let mut path_ptr = None;

        while let (Some(path1), Some(path2)) = (path1_iter.next(), path2_iter.next()) {
            if path1 != path2 {
                break;
            } else {
                path_ptr = Some(path1.to_path_buf());
            }
        }

        path_ptr
    }

    /// Gets the parent of a list of common paths.
    ///
    /// # Panic
    /// Will panic if an empty list of paths is given.
    fn find_common_parent_of_paths<P, I>(paths: I) -> Option<PathBuf>
    where
        P: PathUtils + Sized,
        I: IntoIterator<Item = P>,
    {
        let mut iter = paths.into_iter();
        if let Some(next) = iter.next() {
            let path = next.as_ref().to_path_buf();
            iter.fold(Some(path), |accum, next| match accum {
                None => None,
                Some(accum) => P::find_common_parent(next, accum),
            })
        } else {
            panic!("Can't find a common parent for an empty list of paths")
        }
    }
}

fn find_non_absolute_paths<P, I>(iter: I) -> Vec<P>
where
    P: PathUtils + Sized,
    I: IntoIterator<Item = P>,
{
    iter.into_iter().filter(|p: &P| !p.is_absolute()).collect()
}

macro_rules! impl_path_utils {
    ($ty:ty) => {
        impl PathUtils for $ty {
            fn absolute_path(&self) -> Result<PathBuf, io::Error> {
                <$ty as AsRef<Path>>::as_ref(self).canonicalize()
            }

            fn is_absolute(&self) -> bool {
                <$ty as AsRef<Path>>::as_ref(self).is_absolute()
            }

            fn is_relative(&self) -> bool {
                <$ty as AsRef<Path>>::as_ref(self).is_relative()
            }
        }
    };
}

impl_path_utils!(&'static str);
impl_path_utils!(&Path);
impl_path_utils!(PathBuf);

impl<P: PathUtils> PathUtils for &P {
    fn absolute_path(&self) -> Result<PathBuf, Error> {
        (*self).absolute_path()
    }

    fn is_absolute(&self) -> bool {
        (*self).is_absolute()
    }

    fn is_relative(&self) -> bool {
        (*self).is_relative()
    }
}

pub struct PseudoPath(PathBuf);

impl AsRef<Path> for PseudoPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl<P> From<P> for PseudoPath
where
    P: Into<PathBuf>,
{
    fn from(p: P) -> Self {
        Self(p.into())
    }
}

impl PathUtils for PseudoPath {
    fn absolute_path(&self) -> Result<PathBuf, Error> {
        let mut system = vec![];
        for id in self.0.iter() {
            match id.to_str() {
                Some(".") => {}
                Some("..") => {
                    system.pop();
                }
                _ => system.push(id),
            }
        }
        Ok(PathBuf::from_iter(system))
    }

    fn is_absolute(&self) -> bool {
        dbg!(self.0.has_root())
    }

    fn is_relative(&self) -> bool {
        !self.is_absolute()
    }
}

macro_rules! pp {
    ($expr:expr) => {
        PseudoPath::from($expr)
    };
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

    #[test]
    fn can_resolve_pseudo_paths() {
        let path = pp!("/path/to/././something/../something_else");
        let resolved = path.absolute_path().unwrap();
        assert_eq!(resolved, PathBuf::from("/path/to/something_else"));
    }

    #[test]
    fn relativize_paths() {
        assert_eq!(
            pp!("/test/path/to").resolve(pp!("/test")).unwrap(),
            Path::new("path/to")
        );
        assert_eq!(
            pp!("/test").relativize(pp!("/test/path/to")).unwrap(),
            Path::new("path/to"),
        );
        assert_eq!(
            pp!("/test/path2")
                .relativize(pp!("/test/path1/to"))
                .unwrap(),
            Path::new("../path1/to")
        );
        assert_eq!(
            pp!("/test/path2/././path3/../path3")
                .relativize(pp!("/test/path1/to"))
                .unwrap(),
            Path::new("../../path1/to")
        );
    }
}
