//! This module contains the objects that can be loaded

use crate::error::VMError;
use crate::{VMTryLoadable, VirtualMachine};
use std::borrow::Borrow;
use std::fs::File;
use std::path::{Path, PathBuf};

/// A regular file
#[derive(Debug)]
pub struct RegularFile(File);

impl TryFrom<File> for RegularFile {
    type Error = VMError;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let metadata = value.metadata()?;
        if metadata.is_file() {
            Ok(RegularFile(value))
        } else {
            Err(VMError::WrongFileType)
        }
    }
}

impl VMTryLoadable for RegularFile {
    fn try_load_into_vm<VM>(self, vm: &mut VM) -> Result<(), VMError>
    where
        VM: VirtualMachine,
    {
        todo!()
    }
}

/// Represents a directory that can be loaded
#[derive(Debug)]
pub struct Directory(PathBuf);

impl TryFrom<PathBuf> for Directory {
    type Error = VMError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if value.is_dir() {
            Ok(Directory(value))
        } else {
            Err(VMError::WrongFileType)
        }
    }
}

impl VMTryLoadable for Directory {
    fn try_load_into_vm<VM>(self, vm: &mut VM) -> Result<(), VMError>
    where
        VM: VirtualMachine,
    {
        let dir = std::fs::read_dir(&self.0)?;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let node = FileSystemNode::try_from(path)?;
            node.try_load_into_vm(vm)?;
        }
        Ok(())
    }
}

/// Can either be a directory or a file
#[derive(Debug)]
pub enum FileSystemNode {
    Dir(Directory),
    File(RegularFile),
}

impl TryFrom<PathBuf> for FileSystemNode {
    type Error = VMError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<&Path> for FileSystemNode {
    type Error = VMError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let node = File::open(value)?;
        if value.is_dir() {
            Ok(FileSystemNode::Dir(Directory::try_from(
                value.to_path_buf(),
            )?))
        } else if value.is_file() {
            Ok(FileSystemNode::File(RegularFile::try_from(node)?))
        } else {
            Err(VMError::WrongFileType)
        }
    }
}

impl VMTryLoadable for FileSystemNode {
    fn try_load_into_vm<VM>(self, vm: &mut VM) -> Result<(), VMError>
    where
        VM: VirtualMachine,
    {
        match self {
            FileSystemNode::Dir(d) => d.try_load_into_vm(vm),
            FileSystemNode::File(f) => f.try_load_into_vm(vm),
        }
    }
}

impl VMTryLoadable for &Path {
    fn try_load_into_vm<VM>(self, vm: &mut VM) -> Result<(), VMError>
    where
        VM: VirtualMachine,
    {
        FileSystemNode::try_from(self)?.try_load_into_vm(vm)
    }
}
