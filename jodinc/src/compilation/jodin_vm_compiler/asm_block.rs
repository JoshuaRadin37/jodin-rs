//! Contains supporting code for inserting and creating assembly code for the compiler

use jodin_asm::mvp::bytecode::{Asm, Assembly};
use std::fmt::{Debug, Display, Formatter};

/// An assembly block marks the scope of local labels
#[derive(Debug)]
pub struct AssemblyBlock {
    pub name: Option<String>,
    assembly: Vec<AssemblyBlockComponent>,
}

impl AssemblyBlock {
    /// The length of the assembly block
    pub fn len(&self) -> usize {
        self.assembly.len()
    }

    /// Create an assembly block with an optional name
    pub fn new<'s, O: Into<Option<&'s str>>>(name: O) -> Self {
        Self {
            name: name.into().map(|s| s.to_string()),
            assembly: vec![],
        }
    }

    /// Normalizes the block into standard assembly
    pub fn normalize(&self) -> Assembly {
        todo!()
    }
}

impl Display for AssemblyBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssemblyBlock")
            .field("name", &self.name.as_deref().unwrap_or("<anonymous>"))
            .field("len", &self.len())
            .finish()
    }
}

/// A trait for inserting asm into blocks
pub trait InsertAsm<T> {
    fn insert_asm(&mut self, asm: T);
}

/// The components of assembly, allowing for a tree-like structure of assembly code
pub enum AssemblyBlockComponent {
    SingleInstruction(Asm),
    Block(AssemblyBlock),
}

impl Debug for AssemblyBlockComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblyBlockComponent::SingleInstruction(i) => {
                write!(f, "{:?}", i)
            }
            AssemblyBlockComponent::Block(block) => match f.alternate() {
                true => {
                    write!(f, "{:#?}", block)
                }
                false => {
                    write!(f, "{:?}", block)
                }
            },
        }
    }
}

impl InsertAsm<Asm> for AssemblyBlock {
    fn insert_asm(&mut self, asm: Asm) {
        self.assembly
            .push(AssemblyBlockComponent::SingleInstruction(asm));
    }
}

impl InsertAsm<AssemblyBlock> for AssemblyBlock {
    fn insert_asm(&mut self, asm: AssemblyBlock) {
        if self.len() > 0 {
            self.assembly.push(AssemblyBlockComponent::Block(asm));
        }
    }
}

impl<A> InsertAsm<Option<A>> for AssemblyBlock
where
    AssemblyBlock: InsertAsm<A>,
{
    fn insert_asm(&mut self, asm: Option<A>) {
        match asm {
            None => {}
            Some(asm) => {
                self.insert_asm(asm);
            }
        }
    }
}

pub struct SeperatedAsm<A1, A2>
where
    AssemblyBlock: InsertAsm<A1>,
    AssemblyBlock: InsertAsm<A2>,
{
    /// Insert at the beginning of the block
    pub before: A1,
    /// Insert at the end of the block
    pub after: A2,
}

impl<A1, A2> SeperatedAsm<A1, A2>
where
    AssemblyBlock: InsertAsm<A1>,
    AssemblyBlock: InsertAsm<A2>,
{
    pub fn new(before: A1, after: A2) -> Self {
        SeperatedAsm { before, after }
    }
}

impl<A> SeperatedAsm<A, ()>
where
    AssemblyBlock: InsertAsm<A>,
{
    pub fn before_only(before: A) -> SeperatedAsm<A, ()> {
        SeperatedAsm::<A, ()>::new(before, ())
    }
}

impl<A1, A2> InsertAsm<SeperatedAsm<A1, A2>> for AssemblyBlock
where
    AssemblyBlock: InsertAsm<A1>,
    AssemblyBlock: InsertAsm<A2>,
{
    fn insert_asm(&mut self, asm: SeperatedAsm<A1, A2>) {
        let saved = std::mem::replace(&mut self.assembly, vec![]);
        self.insert_asm(asm.before);
        self.assembly.extend(saved);
        self.insert_asm(asm.after);
    }
}

impl InsertAsm<()> for AssemblyBlock {
    fn insert_asm(&mut self, _asm: ()) {}
}
