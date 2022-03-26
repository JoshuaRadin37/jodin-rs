use crate::CompilerError;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fmt::{write, Display, Formatter};
use std::fs::File;
use std::ops::{Add, AddAssign};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
pub struct ObjectPath {
    path_collection: BTreeSet<PathBuf>,
}

pub const PROJECT_PATH_VARIABLE: &str = "JODIN_PROJECT_DIRECTORY";
pub const EMPTY: &str = "[]";

impl ObjectPath {
    pub fn empty() -> Self {
        Self {
            path_collection: BTreeSet::new(),
        }
    }

    pub fn project_dir() -> Self {
        if let Ok(env_path) = std::env::var(PROJECT_PATH_VARIABLE) {
            Self::singleton(env_path)
        } else {
            Self::empty()
        }
    }

    pub fn singleton<S: AsRef<Path>>(path: S) -> Self {
        Self::empty() + path.as_ref()
    }

    pub fn from_files<P: AsRef<Path>, I: IntoIterator<Item = P>>(files: I) -> Self {
        let dirs = files
            .into_iter()
            .map(|p: P| p.as_ref().to_path_buf())
            .filter_map(|path| match std::fs::metadata(&path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        Some(path)
                    } else if metadata.is_file() {
                        if let Some(parent) = path.parent() {
                            Some(parent.to_path_buf())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                Err(_) => None,
            });
        Self::from_iter(dirs)
    }

    fn add_path(&mut self, path: &Path) -> crate::Result<()> {
        let usable = Self::usable_path(path)?;
        self.path_collection.insert(usable);
        Ok(())
    }

    fn add_paths<P: AsRef<Path>>(
        &mut self,
        paths: impl IntoIterator<Item = P>,
    ) -> crate::Result<()> {
        let extension = paths
            .into_iter()
            .map(Self::usable_path)
            .collect::<Result<BTreeSet<_>, _>>()?;
        self.path_collection.extend(extension);
        Ok(())
    }

    pub fn try_from_iter<P, I>(iter: I) -> crate::Result<ObjectPath>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = P>,
    {
        let paths = iter
            .into_iter()
            .map(Self::usable_path)
            .collect::<Result<BTreeSet<_>, _>>()?;
        Ok(Self {
            path_collection: paths,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &Path> {
        self.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.path_collection.is_empty()
    }

    pub fn len(&self) -> usize {
        self.path_collection.len()
    }

    fn usable_path(path: impl AsRef<Path>) -> crate::Result<PathBuf> {
        path.as_ref()
            .canonicalize()
            .map_err(|e| CompilerError::MissingObjectPath(path.as_ref().to_path_buf(), e))
    }
}

impl Display for ObjectPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            f.write_str(EMPTY)
        } else {
            let joined = std::env::join_paths(self.iter())
                .expect("couldn't join paths")
                .to_str()
                .map(|s| s.to_string())
                .unwrap();
            write!(f, "[{joined}]")
        }
    }
}

impl<'a> FromIterator<&'a Path> for ObjectPath {
    fn from_iter<T: IntoIterator<Item = &'a Path>>(iter: T) -> Self {
        Self::try_from_iter(iter)
            .map_err(|e| e.to_string())
            .unwrap()
    }
}

impl FromIterator<PathBuf> for ObjectPath {
    fn from_iter<T: IntoIterator<Item = PathBuf>>(iter: T) -> Self {
        Self::try_from_iter(iter)
            .map_err(|e| e.to_string())
            .unwrap()
    }
}

impl FromIterator<ObjectPath> for ObjectPath {
    fn from_iter<T: IntoIterator<Item = ObjectPath>>(iter: T) -> Self {
        let mut output = ObjectPath::empty();
        for path in iter {
            output.add_paths(path);
        }
        output
    }
}

impl IntoIterator for ObjectPath {
    type Item = PathBuf;
    type IntoIter = <BTreeSet<PathBuf> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.path_collection.into_iter()
    }
}

impl<'a> IntoIterator for &'a ObjectPath {
    type Item = &'a Path;
    type IntoIter = <BTreeSet<&'a Path> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.path_collection
            .iter()
            .map::<&Path, _>(|pb| pb.as_ref())
            .collect::<BTreeSet<_>>()
            .into_iter()
    }
}

impl<P: AsRef<Path>> Extend<P> for ObjectPath {
    fn extend<T: IntoIterator<Item = P>>(&mut self, iter: T) {
        self.add_paths(iter).expect("Could not add paths");
    }
}

impl Add for ObjectPath {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.extend(rhs);
        self
    }
}

impl Add<PathBuf> for ObjectPath {
    type Output = Self;

    fn add(mut self, rhs: PathBuf) -> Self::Output {
        self.add_path(&*rhs);
        self
    }
}

impl Add<&Path> for ObjectPath {
    type Output = Self;

    fn add(mut self, rhs: &Path) -> Self::Output {
        self.add_path(rhs);
        self
    }
}

impl<O> AddAssign<O> for ObjectPath
where
    ObjectPath: Add<O, Output = ObjectPath>,
{
    fn add_assign(&mut self, rhs: O) {
        let old = std::mem::replace(self, ObjectPath::empty());
        *self = old + rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::compilation::object_path::{ObjectPath, EMPTY};

    #[test]
    fn empty() {
        let empty = ObjectPath::empty();
        assert!(empty.is_empty(), "Empty should be empty");
        assert!(
            (ObjectPath::empty() + ObjectPath::empty()).is_empty(),
            "Adding two empties show result in an empty"
        );
        assert_eq!(
            empty.to_string(),
            EMPTY,
            "Empty to_string should be {:?}",
            EMPTY
        )
    }

    #[test]
    fn addition() {
        let add = ObjectPath::empty() + ObjectPath::singleton("target");
        assert_eq!(add.len(), 1, "Should have only 1 path");
        assert_eq!(add.to_string(), "target");
        let add = ObjectPath::singleton("target1") + ObjectPath::singleton("target2");
        assert_eq!(add.len(), 2, "2 paths!");
        assert_eq!(
            add.to_string(),
            format!(
                "[{:?}]",
                std::env::join_paths(&["target1", "target2"])
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
        );
    }

    #[test]
    fn addition_assign() {
        let mut add = ObjectPath::empty();
        add += ObjectPath::singleton("target");
        assert_eq!(add.len(), 1, "Should have only 1 path");
        assert_eq!(add.to_string(), "target");
        let mut add = ObjectPath::empty();
        add += ObjectPath::singleton("target1");
        add += ObjectPath::singleton("target2");
        assert_eq!(add.len(), 2, "2 paths!");
        assert_eq!(
            add.to_string(),
            format!(
                "[{:?}]",
                std::env::join_paths(&["target1", "target2"])
                    .unwrap()
                    .to_str()
                    .unwrap()
            )
        );
    }
}
