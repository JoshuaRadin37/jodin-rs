//! Contains supporting code for inserting and creating assembly code for the compiler

use crate::assembly::instructions::{Asm, Assembly};
use crate::assembly::location::AsmLocation;
use crate::compilation::{Compilable, Context, PaddedWriter, Target};
use crate::error::JodinResult;
use crate::identifier::Identifier;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use more_collection_macros::set;
use crate::assembly::value::Value;

/// Present at the beginning of a label Marks that the label is relative to the most recent namespace
pub const RELATIVE_LABEL_MARKER: char = '@';
/// Present at the beginning of a label Marks that the label it is to be removed
pub const REMOVE_LABEL_MARKER: char = '#';
/// Present at the beginning of a label marks that the label should be searched for
pub const NONLOCAL_LABEL_MARKER: char = '$';

/// An assembly block marks the scope of local labels
#[derive(Debug, Default, Clone)]
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
    pub fn new<'s, O: Into<Option<&'s String>>>(name: O) -> Self {
        let name = name.into().map(|s| s.clone());
        Self {
            name,
            assembly: vec![]
        }
    }

    /// Creates an assembly block using an identifier
    pub fn with_id<I: Into<Identifier>>(id: I) -> Self {
        let id: Identifier = id.into();
        let name = id.to_string().replace("::", "_").to_lowercase();
        let output = Self::new(&name);
        output
    }

    /// Normalizes the block into standard assembly. Relatives `@<label>` and removes `#<labels>`.
    pub fn normalize(&self) -> Assembly {
        let mut to_normalize = self.clone();
        let base_namespace = self.name.as_ref().map(Identifier::new).unwrap_or(Identifier::empty());
        to_normalize.resolve_relative_labels(&base_namespace);
        let all_labels = to_normalize.find_all_labels();
        to_normalize.resolve_nonlocal_labels(&all_labels, &base_namespace);
        to_normalize._normalize(&base_namespace)
    }

    fn reformat_nonlocal(label: String, nonlocal_hash: u64) -> String {
        let len = label.len();
        let label_hash: u64 = label.char_indices()
            .fold(0, |acc, (index, ch)| {
                acc.wrapping_add(u64::from(ch).wrapping_mul(31u64.wrapping_pow((len - (index + 1)) as u32)))
            });
        (nonlocal_hash ^ label_hash).to_string()
    }

    fn resolve_relative_labels(&mut self, current_namespace: &Identifier) {
        for asm_comp in &mut self.assembly {
            match asm_comp {
                AssemblyBlockComponent::SingleInstruction(Asm::Label(lbl)) => {
                    if lbl.starts_with(RELATIVE_LABEL_MARKER) {
                        let normalized = Self::normalize_label(current_namespace, lbl);
                        *lbl = normalized;
                    }
                }
                AssemblyBlockComponent::SingleInstruction(Asm::Goto(AsmLocation::Label(lbl))) => {
                    if lbl.starts_with(RELATIVE_LABEL_MARKER) {
                        let normalized = Self::normalize_label(current_namespace, lbl);
                        *lbl = normalized;
                    }
                }
                AssemblyBlockComponent::SingleInstruction(Asm::CondGoto(AsmLocation::Label(lbl))) => {
                    if lbl.starts_with(RELATIVE_LABEL_MARKER) {
                        let normalized = Self::normalize_label(current_namespace, lbl);
                        *lbl = normalized;
                    }
                }
                AssemblyBlockComponent::Block(b) => {
                    let ref next_namespace = match &b.name {
                        None => { current_namespace.clone() }
                        Some(name) => { current_namespace + &Identifier::from(name) }
                    };
                    b.resolve_relative_labels(next_namespace);
                }
                _ => { }
            }
        }
    }

    fn find_nonlocal_label(label: &String, all_labels: &HashSet<String>, current_namespace: &Identifier) -> Option<String> {
        let ref label = label[1..].to_string();
        let mut namespace_ptr = Some(current_namespace.clone());
        while let Some(namespace) = namespace_ptr {

            let normalized = Self::normalize_label(&namespace, label);
            if all_labels.contains(&normalized) {
                return Some(normalized);
            }

            if namespace.is_empty() {
                namespace_ptr = None
            } else {
                namespace_ptr = Some(namespace.into_parent().unwrap_or(Identifier::empty()));
            }
        }
        None
    }

    fn resolve_nonlocal_labels(&mut self, all_labels: &HashSet<String>, current_namespace: &Identifier) {
        for asm_comp in &mut self.assembly {
            match asm_comp {
                AssemblyBlockComponent::SingleInstruction(Asm::Label(lbl)) => {
                    if lbl.starts_with(NONLOCAL_LABEL_MARKER) {
                        let normalized = Self::find_nonlocal_label(lbl, all_labels, current_namespace)
                            .expect(format!("Couldn't find a label in parents named {lbl}").as_str());
                        *lbl = normalized;
                    }
                }
                AssemblyBlockComponent::SingleInstruction(Asm::Goto(AsmLocation::Label(lbl))) => {
                    if lbl.starts_with(NONLOCAL_LABEL_MARKER) {
                        let normalized = Self::find_nonlocal_label(lbl, all_labels, current_namespace)
                            .expect(format!("Couldn't find a label in parents named {lbl}").as_str());
                        *lbl = normalized;
                    }
                }
                AssemblyBlockComponent::SingleInstruction(Asm::CondGoto(AsmLocation::Label(lbl))) => {
                    if lbl.starts_with(NONLOCAL_LABEL_MARKER) {
                        let normalized = Self::find_nonlocal_label(lbl, all_labels, current_namespace)
                            .expect(format!("Couldn't find a label in parents named {lbl}").as_str());
                        *lbl = normalized;
                    }
                }
                AssemblyBlockComponent::Block(b) => {
                    let ref next_namespace = match &b.name {
                        None => { current_namespace.clone() }
                        Some(name) => { current_namespace + &Identifier::from(name) }
                    };
                    b.resolve_nonlocal_labels(all_labels, next_namespace);
                }
                _ => { }
            }
        }
    }

    fn find_all_labels(&self) -> HashSet<String> {
        let mut output = set!();
        for asm_comp in &self.assembly {
            match asm_comp {
                AssemblyBlockComponent::SingleInstruction(Asm::Label(lbl)) => {
                    if !lbl.starts_with(NONLOCAL_LABEL_MARKER) {
                        output.insert(lbl.clone());
                    }
                }
                AssemblyBlockComponent::Block(b) => {
                    output.extend(b.find_all_labels());
                }
                _ => { }
            }
        }
        output
    }


    fn _normalize(self, current_namespace: &Identifier) -> Assembly {

        let mut output = Assembly::new();
        for comp in self.assembly {
            match comp {
                AssemblyBlockComponent::SingleInstruction(s) => {
                    output.push(s);
                }
                AssemblyBlockComponent::Block(b) => {
                    let namespace = Identifier::new_concat(
                        current_namespace,
                        b.name.as_ref().unwrap_or(&String::new()),
                    );
                    output.extend(b._normalize(&namespace));
                }
            }
        }
        output
    }

    fn normalize_label(current_namespace: &Identifier, lbl: &String) -> String {
        let id = current_namespace + &Identifier::from(lbl.replace(RELATIVE_LABEL_MARKER, ""));
        let as_os_string = id.os_compat().expect("This should be valid os string");
        let true_label = as_os_string
            .into_string()
            .expect("Should be valid as identifiers must be valid os strings anyway");
        debug!(
            "Normalized {:?} to {:?} (current_namespace: {:?}, id: {:?})",
            lbl, true_label, current_namespace, id
        );
        true_label
    }

    /// Inserts asm after a label in this block only. Does *NOT* recurse.
    pub fn insert_after_label<A>(&mut self, asm: A, label: impl AsRef<str>) -> bool
    where
        Self: InsertAsm<A>,
    {
        let label = label.as_ref();
        let position = self.assembly.iter().position(|asm_comp| {
            if let AssemblyBlockComponent::SingleInstruction(Asm::Label(found_lbl)) = asm_comp {
                found_lbl == label
            } else {
                false
            }
        });
        match position {
            None => false,
            Some(pos) => self.insert_asm_at_position(pos + 1, asm),
        }
    }

    /// Inserts asm after a label in this block only. Does *NOT* recurse.
    pub fn insert_before_label<A>(&mut self, asm: A, label: impl AsRef<str>) -> bool
    where
        Self: InsertAsm<A>,
    {
        let label = label.as_ref();
        let position = self.assembly.iter().position(|asm_comp| {
            if let AssemblyBlockComponent::SingleInstruction(Asm::Label(found_lbl)) = asm_comp {
                found_lbl == label
            } else {
                false
            }
        });
        match position {
            None => false,
            Some(pos) => self.insert_asm_at_position(pos, asm),
        }
    }
}

impl From<Asm> for AssemblyBlock {
    fn from(a: Asm) -> Self {
        let mut output = AssemblyBlock::new(None);
        output.insert_asm(a);
        output
    }
}

impl From<Assembly> for AssemblyBlock {
    fn from(a: Assembly) -> Self {
        let mut output = AssemblyBlock::new(None);
        output.insert_asm(a);
        output
    }
}

/// Creates a label instruction with the [`RELATIVE_LABEL_MARKER`](RELATIVE_LABEL_MARKER) proceeding
/// it.
pub fn rel_label<S: AsRef<str>>(relative: S) -> String {
    format!("{}{}", RELATIVE_LABEL_MARKER, relative.as_ref())
}

/// Creates a label instruction with the [`NONLOCAL_LABEL_MARKER`](NONLOCAL_LABEL_MARKER) proceeding
/// it.
pub fn nonlocal_label<S: AsRef<str>>(relative: S) -> String {
    format!("{}{}", NONLOCAL_LABEL_MARKER, relative.as_ref())
}

pub fn temp_label(lbl: impl AsRef<str>) -> String {
    format!("{}{}", REMOVE_LABEL_MARKER, lbl.as_ref())
}

pub fn id_label(id: &Identifier, lbl: impl AsRef<str>) -> String {
    format!(
        "{}{}",
        id.os_compat()
            .expect("should be os-compatable")
            .to_str()
            .unwrap()
            .to_ascii_lowercase(),
        lbl.as_ref()
    )
}

impl Display for AssemblyBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssemblyBlock")
            .field("name", &self.name.as_deref().unwrap_or("<anonymous>"))
            .field("len", &self.len())
            .finish()
    }
}

pub trait InsertAsmHelper {
    fn len(&self) -> usize;
}

/// A trait for inserting asm into blocks
pub trait InsertAsm<T>: InsertAsmHelper {
    fn insert_asm(&mut self, asm: T) {
        self.insert_asm_at_position(self.len(), asm);
    }
    fn insert_asm_front(&mut self, asm: T) {
        self.insert_asm_at_position(0, asm);
    }
    fn insert_asm_at_position(&mut self, index: usize, asm: T) -> bool;
}

/// The components of assembly, allowing for a tree-like structure of assembly code
#[derive(Clone)]
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

impl InsertAsmHelper for AssemblyBlock {
    fn len(&self) -> usize {
        AssemblyBlock::len(self)
    }
}

impl InsertAsm<Asm> for AssemblyBlock {
    fn insert_asm(&mut self, asm: Asm) {
        self.assembly
            .push(AssemblyBlockComponent::SingleInstruction(asm));
    }

    fn insert_asm_at_position(&mut self, index: usize, asm: Asm) -> bool {
        if index > self.len() {
            return false;
        }
        self.assembly
            .insert(index, AssemblyBlockComponent::SingleInstruction(asm));
        true
    }
}

impl<A> InsertAsm<Vec<A>> for AssemblyBlock
where
    AssemblyBlock: InsertAsm<A>,
{
    fn insert_asm_at_position(&mut self, index: usize, mut asm: Vec<A>) -> bool {
        asm.reverse();
        for asm in asm {
            if !self.insert_asm_at_position(index, asm) {
                return false;
            }
        }
        true
    }
}

impl InsertAsm<AssemblyBlock> for AssemblyBlock {
    fn insert_asm_at_position(&mut self, index: usize, asm: AssemblyBlock) -> bool {
        if index > self.len() {
            return false;
        }
        if asm.name.is_some() {
            self.assembly
                .insert(index, AssemblyBlockComponent::Block(asm));
        } else {
            let instructions = asm.assembly;
            for comp in instructions.into_iter().rev() {
                self.assembly.insert(index, comp);
            }
        }

        true
    }
}

impl<A> InsertAsm<Option<A>> for AssemblyBlock
where
    AssemblyBlock: InsertAsm<A>,
{
    fn insert_asm_at_position(&mut self, index: usize, asm: Option<A>) -> bool {
        match asm {
            None => false,
            Some(asm) => self.insert_asm_at_position(index, asm),
        }
    }
}

impl InsertAsm<Value> for AssemblyBlock {
    fn insert_asm_at_position(&mut self, index: usize, asm: Value) -> bool {
        self.insert_asm_at_position(index, Asm::push(asm))
    }
}

impl InsertAsm<&str> for AssemblyBlock {
    fn insert_asm_at_position(&mut self, index: usize, asm: &str) -> bool {
        self.insert_asm_at_position(index, Asm::label(asm))
    }
}

impl InsertAsm<String> for AssemblyBlock {
    fn insert_asm_at_position(&mut self, index: usize, asm: String) -> bool {
        self.insert_asm_at_position(index, Asm::label(asm))
    }
}

macro_rules! insert_asm_value {
    ($ty:ty) => {
        impl InsertAsm<$ty> for AssemblyBlock {
            fn insert_asm_at_position(&mut self, index: usize, asm: $ty) -> bool {
                self.insert_asm_at_position(index, Asm::push(asm))
            }
        }
    };
}

insert_asm_value!(u8);
insert_asm_value!(u16);
insert_asm_value!(u32);
insert_asm_value!(u64);
insert_asm_value!(usize);
insert_asm_value!(i8);
insert_asm_value!(i16);
insert_asm_value!(i32);
insert_asm_value!(i64);
insert_asm_value!(isize);

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

    /// inserts before index, and after after index.
    fn insert_asm_at_position(&mut self, index: usize, asm: SeperatedAsm<A1, A2>) -> bool {
        let SeperatedAsm { before, after } = asm;
        if !self.insert_asm_at_position(index.saturating_sub(1), before) {
            return false;
        }
        self.insert_asm_at_position((index + 1).max(self.len()), after)
    }
}

impl InsertAsm<()> for AssemblyBlock {
    fn insert_asm_at_position(&mut self, _index: usize, _asm: ()) -> bool {
        true
    }
}

impl<T: Target> Compilable<T> for AssemblyBlock {
    fn compile<W: Write>(self, context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        let normalized: Assembly = self.normalize();
        Compilable::<T>::compile(normalized, context, w)
    }
}

impl<T: Target> Compilable<T> for Asm {
    fn compile<W: Write>(self, _context: &Context, w: &mut PaddedWriter<W>) -> JodinResult<()> {
        // let encoded = self.encode();
        let encoded = Some(self);
        for byte in encoded {
            writeln!(w, "{:?}", byte)?;
        }
        Ok(())
    }
}

trait RemoveUnused {
    fn remove_unused(self) -> Self;
}

impl RemoveUnused for Assembly {
    fn remove_unused(mut self) -> Self {
        let mut found_labels = HashSet::<String>::new();
        let mut used_labels = HashSet::<String>::new();
        for x in &self {
            if let Asm::Label(lbl) = x {
                found_labels.insert(lbl.clone());
            } else if let Asm::Goto(AsmLocation::Label(lbl)) = x {
                used_labels.insert(lbl.clone());
            } else if let Asm::CondGoto(AsmLocation::Label(lbl)) = x {
                used_labels.insert(lbl.clone());
            }
        }

        let unused = found_labels
            .difference(&used_labels)
            .collect::<HashSet<_>>();
        self.retain(|asm| match asm {
            Asm::Label(lbl) => !unused.contains(lbl),
            _ => true,
        });
        self
    }
}
